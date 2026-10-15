//! Typed native signal connections (CXX-Qt `extern "C++Qt"` imports).
//!
//! A typed connection is an ordinary callback id in the shared registry: the
//! native functor only owns a [`SignalToken`], converts Qt arguments to owned
//! Rust values inside the emission frame, and queues them. The user closure
//! runs later from the same runtime pump as relay callbacks, after every
//! native transaction has returned. It never receives the sender reference.
use crate::{SignalEvent, remove_callback, runtime};
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::thread::ThreadId;

thread_local! {
    // RAII connection guards (e.g. cxx_qt::QMetaObjectConnectionGuard), keyed by
    // callback id. Thread-local: only the owning Qt thread may disconnect.
    static CONNECTIONS: RefCell<HashMap<usize, Box<dyn Any>>> = RefCell::new(HashMap::new());
}

/// Moved into the native functor. `Send` only so it satisfies CXX-Qt's
/// closure bound; emission and disposal on any other thread are ignored.
pub struct SignalToken<A> {
    id: usize,
    thread: ThreadId,
    _arg: PhantomData<fn(A)>,
}

impl<A: 'static> SignalToken<A> {
    /// Queue one owned argument value for the registered Rust callback.
    /// Never invokes the callback synchronously inside the Qt emission.
    pub fn emit(&self, value: A) {
        if std::thread::current().id() == self.thread {
            runtime::emit_signal(self.id, SignalEvent::Owned(Box::new(value)));
        }
    }
    pub fn id(&self) -> usize {
        self.id
    }
}

impl<A> Drop for SignalToken<A> {
    // Qt destroys the functor when the sender dies or the connection is
    // disconnected; this may run inside QObject destruction. Cancel the id
    // and retire both the capture and the connection guard to a Rust boundary.
    fn drop(&mut self) {
        if std::thread::current().id() != self.thread {
            return;
        }
        remove_callback(self.id, true);
        if let Some(guard) = CONNECTIONS.with(|c| c.borrow_mut().remove(&self.id)) {
            runtime::retire_value(guard);
        }
    }
}

/// A unique heap copy of a Qt value made inside a signal emission (e.g. a
/// `QFont` or `QPersistentModelIndex`), owned until the safe layer adopts it
/// into its value wrapper. Dropped undelivered (cancelled id, closing scope)
/// it calls its matching dtk-sys deleter; it never refers to Qt storage.
pub struct OwnedValue<T> {
    ptr: *mut T,
    delete: unsafe fn(*mut T),
}

impl<T> OwnedValue<T> {
    /// # Safety
    /// `ptr` must be a fresh, uniquely owned allocation that `delete` frees.
    pub unsafe fn new(ptr: *mut T, delete: unsafe fn(*mut T)) -> Self {
        assert!(!ptr.is_null(), "Qt value copy failed");
        Self { ptr, delete }
    }
    /// Transfer the unique allocation to a wrapper with the same deleter.
    pub fn into_raw(self) -> *mut T {
        let ptr = self.ptr;
        std::mem::forget(self);
        ptr
    }
}

impl<T> Drop for OwnedValue<T> {
    fn drop(&mut self) {
        unsafe { (self.delete)(self.ptr) }
    }
}

/// Register `f` and let `connect` create the native connection owning the
/// token. `connect` returns an RAII guard whose Drop disconnects, or `None`
/// on failure. Returns the callback id for `unregister_cb`; 0 = failed.
///
/// The caller (the safe `dtk` layer) brackets this with a `NativeCall` and
/// verifies the sender is live on its Qt owner thread.
pub fn connect<A: 'static>(
    mut f: impl FnMut(A) + 'static,
    connect: impl FnOnce(SignalToken<A>) -> Option<Box<dyn Any>>,
) -> usize {
    let id = crate::register_callback(crate::Cb::Owned(Box::new(move |value| {
        match value.downcast::<A>() {
            Ok(value) => f(*value),
            Err(_) => unreachable!("typed signal argument mismatch"),
        }
    })));
    let token = SignalToken {
        id,
        thread: std::thread::current().id(),
        _arg: PhantomData,
    };
    match connect(token) {
        Some(guard) => {
            CONNECTIONS.with(|c| c.borrow_mut().insert(id, guard));
            id
        }
        None => {
            crate::unregister_cb(id);
            0
        }
    }
}

/// Explicit disconnect from `unregister_cb`. The guard is dropped outside the
/// registry borrow: disconnecting destroys the functor, whose token reenters.
pub(crate) fn disconnect(id: usize) {
    let guard = CONNECTIONS.with(|c| c.borrow_mut().remove(&id));
    drop(guard);
}

/// Whether `id` still owns a live typed connection on this thread.
pub fn is_connected(id: usize) -> bool {
    CONNECTIONS.with(|c| c.borrow().contains_key(&id))
}

pub(crate) fn is_empty() -> bool {
    CONNECTIONS.with(|c| c.borrow().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    struct Guard(Rc<Cell<usize>>);
    impl Drop for Guard {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    // No Qt application here: the queue dispatches immediately, which is the
    // registry fallback also used by the relay unit tests.
    #[test]
    fn typed_token_delivers_owned_values_and_disposes() {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let disconnects = Rc::new(Cell::new(0));
        let token = Rc::new(RefCell::new(None));
        let (sink, slot, count) = (seen.clone(), token.clone(), disconnects.clone());
        let id = connect(
            move |text: String| sink.borrow_mut().push(text),
            move |t| {
                *slot.borrow_mut() = Some(t);
                Some(Box::new(Guard(count)))
            },
        );
        assert_ne!(id, 0);
        token.borrow().as_ref().unwrap().emit("中文".to_owned());
        assert_eq!(*seen.borrow(), ["中文"]);
        // Native functor destruction (sender died): callback cancelled and
        // the guard retired, not leaked.
        drop(token.borrow_mut().take());
        assert_eq!(disconnects.get(), 1);
        assert!(!crate::unregister_cb(id));
        assert!(crate::callback_registry_is_empty());
    }

    #[test]
    fn explicit_disconnect_and_failed_connect_roll_back() {
        let disconnects = Rc::new(Cell::new(0));
        let count = disconnects.clone();
        let id = connect(
            |_: bool| {},
            move |t| {
                std::mem::forget(t); // stands in for a functor Qt still owns
                Some(Box::new(Guard(count)))
            },
        );
        assert!(crate::unregister_cb(id));
        assert_eq!(disconnects.get(), 1);
        assert_eq!(connect(|_: i32| {}, |_| None), 0);
        assert!(crate::callback_registry_is_empty());
    }
}
