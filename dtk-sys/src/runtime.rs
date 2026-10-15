//! Serialized native transactions and owned callback delivery.
//! Ordinary callbacks never run inside their originating Qt signal/event frame.
use crate::{Cb, PwEvent, SignalEvent, dispatch_pw, dispatch_signal, drop_callback, ffi};
use std::cell::{Cell, RefCell};
use std::collections::{HashSet, VecDeque};
use std::marker::PhantomData;

/// Bracket an entire safe wrapper operation, including wrapping its return.
/// Shared QObject handles serialize native access; they do not promise an
/// exclusive Rust reference or permit constructing `Pin<&mut T>` from `&self`.
pub struct NativeCall {
    frame: cxx::UniquePtr<ffi::NativeCallFrame>,
    _not_send: PhantomData<*mut ()>,
}

impl NativeCall {
    pub fn new() -> Self {
        Self::enter(false)
    }
    /// Internal, individually audited control: modal close, one-shot timer/quit.
    pub fn control() -> Self {
        Self::enter(true)
    }
    /// Audited painting operation on a borrowed `QPainter`: also admitted
    /// inside the synchronous paint callback that lends that painter (which
    /// rejects every ordinary QObject operation).
    pub fn paint() -> Self {
        crate::assert_not_closing();
        let frame = unsafe { ffi::runtime_paint_call_new() };
        assert!(
            !frame.is_null(),
            "Qt native state is borrowed by a suspended native call"
        );
        Self {
            frame,
            _not_send: PhantomData,
        }
    }
    fn enter(control: bool) -> Self {
        crate::assert_not_closing();
        let frame = unsafe { ffi::runtime_call_new(control) };
        assert!(
            !frame.is_null(),
            "Qt native state is borrowed by a synchronous callback or suspended native call"
        );
        Self {
            frame,
            _not_send: PhantomData,
        }
    }
}
impl Default for NativeCall {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for NativeCall {
    fn drop(&mut self) {
        // NativeCallFrame destruction does NOT dispatch Rust from C++.
        // Raw returns have already been wrapped before this Rust boundary.
        drop(std::mem::replace(&mut self.frame, cxx::UniquePtr::null()));
        if !std::thread::panicking()
            && !CLEANING.with(Cell::get)
            && unsafe { ffi::runtime_can_finish_call() }
        {
            drain_pending();
        }
    }
}

enum Pending {
    Ready(usize, SignalEvent),
    Signal(usize, SignalEvent),
    Widget(usize, PwEvent),
}
thread_local! {
    static PENDING: RefCell<VecDeque<Pending>> = const { RefCell::new(VecDeque::new()) };
    static READY: RefCell<HashSet<usize>> = RefCell::new(HashSet::new());
    // Captures destroyed by Qt must not Drop/reenter from QObject destructors.
    static RETIRED: RefCell<Vec<(usize, Cb)>> = const { RefCell::new(Vec::new()) };
    static CLEANUP: RefCell<Vec<Box<dyn FnOnce()>>> = const { RefCell::new(Vec::new()) };
    static CLEANING: Cell<bool> = const { Cell::new(false) };
}

pub(crate) fn emit_signal(id: usize, event: SignalEvent) {
    if crate::callbacks_closing() {
        return;
    }
    if unsafe { ffi::application_is_core() } {
        PENDING.with(|pending| pending.borrow_mut().push_back(Pending::Signal(id, event)));
        unsafe { ffi::runtime_schedule_callbacks() };
    } else {
        // Standalone callback registry unit tests have no Qt application.
        dispatch_signal(id, event);
    }
}
pub(crate) fn emit_ready(id: usize, event: SignalEvent) {
    if crate::callbacks_closing() {
        return;
    }
    if !unsafe { ffi::application_is_core() } {
        dispatch_signal(id, event);
        return;
    }
    if READY.with(|ready| ready.borrow_mut().insert(id)) {
        PENDING.with(|pending| pending.borrow_mut().push_back(Pending::Ready(id, event)));
        unsafe { ffi::runtime_schedule_callbacks() };
    }
}
struct ReadyBoundary(usize);
impl Drop for ReadyBoundary {
    fn drop(&mut self) {
        READY.with(|ready| {
            ready.borrow_mut().remove(&self.0);
        });
        unsafe { ffi::relay_finish_ready(self.0) };
    }
}

pub(crate) fn emit_widget(id: usize, event: PwEvent) {
    if matches!(event, PwEvent::Paint { .. }) || !unsafe { ffi::application_is_core() } {
        // Stack-scoped painters cannot be queued or replayed.
        dispatch_pw(id, event);
    } else if !crate::callbacks_closing() {
        PENDING.with(|pending| pending.borrow_mut().push_back(Pending::Widget(id, event)));
        unsafe { ffi::runtime_schedule_callbacks() };
    }
}
pub(crate) fn retire_callback(id: usize, cb: Cb) {
    RETIRED.with(|retired| retired.borrow_mut().push((id, cb)));
    unsafe { ffi::runtime_schedule_callbacks() };
}

/// Always defer a value's Drop (e.g. a Qt connection guard released from a
/// functor destructor) to the next Rust cleanup boundary, never inline.
pub(crate) fn retire_value(value: Box<dyn std::any::Any>) {
    if !unsafe { ffi::application_is_core() } {
        drop(value);
        return;
    }
    CLEANUP.with(|cleanup| cleanup.borrow_mut().push(Box::new(move || drop(value))));
    unsafe { ffi::runtime_schedule_callbacks() };
}

/// Defer a unique allocation's destructor while native code may hold pointers.
/// The allocation remains owned in this queue; no deleter or owner is duplicated.
pub fn defer_cleanup(f: impl FnOnce() + 'static) {
    if unsafe { ffi::runtime_native_borrow_active() } {
        CLEANUP.with(|cleanup| cleanup.borrow_mut().push(Box::new(f)));
        unsafe { ffi::runtime_schedule_callbacks() };
    } else {
        f();
    }
}

struct CleanupBoundary;
impl Drop for CleanupBoundary {
    fn drop(&mut self) {
        CLEANING.with(|active| active.set(false));
    }
}

fn drain_cleanup() {
    if CLEANING.with(Cell::get) || unsafe { ffi::runtime_native_borrow_active() } {
        return;
    }
    CLEANING.with(|active| active.set(true));
    let _boundary = CleanupBoundary;
    // Drop outside RefCell borrows: user destructors can unregister ids and
    // retire more captures/deleters. Drain to a fixed point, including closing.
    loop {
        let retired = RETIRED.with(|retired| std::mem::take(&mut *retired.borrow_mut()));
        let cleanup = CLEANUP.with(|cleanup| std::mem::take(&mut *cleanup.borrow_mut()));
        if retired.is_empty() && cleanup.is_empty() {
            break;
        }
        for (id, cb) in retired {
            drop_callback(id, cb);
        }
        for f in cleanup {
            if std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).is_err() {
                eprintln!("dtk-rs: deferred destructor panicked, contained at runtime boundary");
            }
        }
    }
}

pub(crate) fn drain_pending() {
    // Native destruction cancels ids immediately. Retire their captures before
    // unrelated notifications can observe a half-cleaned-up owner; this is now
    // a Rust boundary, not the QObject destructor that requested retirement.
    drain_cleanup();
    // Modal/native loops may run other ids, never the same checked-out FnMut.
    // dispatch_signal/dispatch_pw retain their per-id copied-argument queues.
    while !crate::callbacks_closing() && unsafe { ffi::runtime_can_dispatch_callbacks() } {
        let event = PENDING.with(|pending| pending.borrow_mut().pop_front());
        match event {
            Some(Pending::Ready(id, event)) => {
                // Keep the level marked until the callback returns, including
                // nested native loops. Repeated readiness is not another read.
                let _ready = ReadyBoundary(id);
                dispatch_signal(id, event);
            }
            Some(Pending::Signal(id, event)) => dispatch_signal(id, event),
            Some(Pending::Widget(id, event)) => dispatch_pw(id, event),
            None => break,
        }
        drain_cleanup();
    }
    drain_cleanup();
}
pub(crate) fn end_scope() {
    PENDING.with(|pending| pending.borrow_mut().clear());
    READY.with(|ready| ready.borrow_mut().clear());
    drain_cleanup();
}
pub(crate) fn is_empty() -> bool {
    PENDING.with(|pending| pending.borrow().is_empty())
        && READY.with(|ready| ready.borrow().is_empty())
        && RETIRED.with(|retired| retired.borrow().is_empty())
        && CLEANUP.with(|cleanup| cleanup.borrow().is_empty())
}
