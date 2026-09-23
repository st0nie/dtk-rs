//! The canonical dtk-rs threading pattern: background worker → GUI thread
//! via `std::sync::mpsc` + `eventfd` + `QSocketNotifier`.
//!
//! Run:  cargo run --example mpsc_eventfd
//! Headless self-check:  QT_QPA_PLATFORM=offscreen ./target/debug/examples/mpsc_eventfd --smoke
//!
//! Why not just touch widgets from the worker thread?
//! - Qt widgets may only be touched from the GUI thread.
//! - dtk-rs wrappers are `!Send`, so the compiler stops you anyway.
//!
//! The pattern (Qt-side equivalent of `QMetaObject::invokeMethod` queued calls):
//!
//! ```text
//!   worker thread                GUI thread (event loop)
//!   ┌──────────────┐  mpsc::Sender<Msg>   ┌─────────────────────────┐
//!   │ do work      │ ──────────────────▶  │ mpsc::Receiver<Msg>     │
//!   │ tx.send(msg) │                      │ (drained with try_recv) │
//!   │ poke eventfd │ ── eventfd byte ──▶  │ QSocketNotifier::       │
//!   └──────────────┘                      │   on_activated(...)     │
//!                                         └─────────────────────────┘
//! ```
//!
//! - `Msg` is a plain `Send` Rust value — no widget pointers cross threads.
//! - eventfd is only a *wakeup*: one byte per message, drained in the callback.
//! - Commands back to the worker (start/cancel) go through `Arc<AtomicBool>`.
//! - No polling timer needed: the notifier fires exactly when data arrives.

use dtk::widgets::DProgressBar;
use dtk::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::time::Duration;

/// messages from worker to GUI; plain data, `Send` by construction
enum Msg {
    Progress(i32),
    Done(String),
}

fn main() {
    let app = DApplication::new("dtk-rs-mpsc-eventfd");

    let win = DMainWindow::new();
    win.set_window_title("mpsc + eventfd");
    win.titlebar().set_title("Worker → GUI: mpsc + eventfd");

    let central = QWidget::new(None);
    let vbox = QVBoxLayout::new(Some(&central));
    vbox.set_contents_margins(24, 24, 24, 24);
    vbox.set_spacing(12);

    let status = Rc::new(DLabel::new("Idle."));
    vbox.add_widget(&status.as_widget());

    let bar = Rc::new(DProgressBar::new());
    bar.set_range(0, 100);
    vbox.add_widget(&bar.as_widget());

    let btn = Rc::new(DSuggestButton::new("Start"));
    vbox.add_widget(&btn.as_widget());

    win.set_central_widget(&central);
    win.resize(380, 160);

    // ---- wiring: mpsc for data, eventfd for wakeup ----
    let (tx, rx) = mpsc::channel::<Msg>();
    let fd = unsafe { eventfd(0, EFD_NONBLOCK | EFD_CLOEXEC) };
    assert!(fd >= 0, "eventfd failed");

    let cancel = Arc::new(AtomicBool::new(false));
    let paused = Arc::new(AtomicBool::new(false));

    // GUI side: notifier fires on eventfd → drain channel, update widgets.
    let notifier = QSocketNotifier::new(fd, &win);
    {
        let status = status.clone();
        let bar = bar.clone();
        let btn = btn.clone();
        let paused = paused.clone();
        let finished = Rc::new(std::cell::Cell::new(false));
        {
            let bar = bar.clone();
            let finished = finished.clone();
            notifier.on_activated(move || {
                // drain the eventfd counter first, or the level-triggered
                // notifier refires forever
                let mut n: u64 = 0;
                while unsafe { read(fd, &mut n as *mut u64 as *mut u8, 8) } == 8 {}
                // then drain every queued message (multiple sends per wakeup possible)
                while let Ok(msg) = rx.try_recv() {
                    match msg {
                        Msg::Progress(p) => {
                            bar.set_value(p);
                            // stale in-flight progress must not overwrite "Paused."
                            if !paused.load(Ordering::SeqCst) {
                                status.set_text(&format!("Working… {p}%"));
                            }
                        }
                        Msg::Done(text) => {
                            bar.set_value(100);
                            status.set_text(&text);
                            btn.set_text("Start");
                            finished.set(true);
                        }
                    }
                }
            });
        }
        if std::env::args().any(|a| a == "--smoke") {
            // offscreen self-check: poll until the worker delivers Done (loaded CI is slow)
            let tries = Rc::new(Cell::new(0));
            let poll = Rc::new(RefCell::new(None::<Box<dyn FnMut()>>));
            let poll2 = poll.clone();
            *poll.borrow_mut() = Some(Box::new(move || {
                if finished.get() {
                    assert!(bar.value() > 0, "no progress reached the GUI thread");
                    println!("smoke ok");
                    DApplication::quit();
                    return;
                }
                tries.set(tries.get() + 1);
                assert!(tries.get() < 40, "worker never sent Done"); // ~20s budget
                let poll = poll2.clone();
                QTimer::single_shot(500, move || {
                    if let Some(f) = &mut *poll.borrow_mut() {
                        f();
                    }
                });
            }));
            let p = poll.clone();
            QTimer::single_shot(500, move || {
                if let Some(f) = &mut *p.borrow_mut() {
                    f();
                }
            });
        }
    }

    // control channel GUI → worker: plain atomics are enough for start/cancel
    let running = Arc::new(AtomicBool::new(false));
    let spawn_worker = Rc::new({
        let running = running.clone();
        let cancel = cancel.clone();
        let paused = paused.clone();
        move || {
            running.store(true, Ordering::SeqCst);
            cancel.store(false, Ordering::SeqCst);
            paused.store(false, Ordering::SeqCst);
            let tx = tx.clone();
            let running = running.clone();
            let cancel = cancel.clone();
            let paused = paused.clone();
            std::thread::spawn(move || {
                for p in (0..=100).step_by(5) {
                    if cancel.load(Ordering::SeqCst) {
                        break;
                    }
                    // ponytail: 50ms poll while paused, fine for a demo
                    while paused.load(Ordering::SeqCst) && !cancel.load(Ordering::SeqCst) {
                        std::thread::sleep(Duration::from_millis(50));
                    }
                    if cancel.load(Ordering::SeqCst) {
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(120)); // fake work
                    if tx.send(Msg::Progress(p)).is_err() {
                        return; // GUI gone
                    }
                    poke(fd);
                }
                if !cancel.load(Ordering::SeqCst) {
                    let _ = tx.send(Msg::Done("Done.".into()));
                    poke(fd);
                }
                running.store(false, Ordering::SeqCst);
            });
        }
    });

    {
        let spawn_worker = spawn_worker.clone();
        let running = running.clone();
        let paused = paused.clone();
        let status = status.clone();
        let btn = btn.clone();
        let btn_handle = btn.clone();
        btn.on_clicked(move || {
            if running.load(Ordering::SeqCst) {
                // toggle pause/resume
                if paused.fetch_xor(true, Ordering::SeqCst) {
                    btn_handle.set_text("Pause");
                    status.set_text("Working…");
                } else {
                    btn_handle.set_text("Resume");
                    status.set_text("Paused.");
                }
            } else {
                spawn_worker();
                btn_handle.set_text("Pause");
            }
        });
    }

    win.show();

    // --smoke: auto-start without clicking
    if std::env::args().any(|a| a == "--smoke") {
        spawn_worker();
    }

    std::mem::forget(btn); // Rc<DSuggestButton>: leak() is on the inner wrapper, forget the Rc
    let code = app.exec();
    cancel.store(true, Ordering::SeqCst); // let a running worker exit on its own
    unsafe { close(fd) };
    std::process::exit(code);
}

fn poke(fd: i32) {
    let one: u64 = 1;
    unsafe { write(fd, &one as *const u64 as *const u8, 8) };
}

const EFD_NONBLOCK: i32 = 0o4000;
const EFD_CLOEXEC: i32 = 0o2000000;

// no libc dependency in this crate; declare the syscalls directly
unsafe extern "C" {
    fn eventfd(initval: u32, flags: i32) -> i32;
    fn read(fd: i32, buf: *mut u8, n: usize) -> isize;
    fn write(fd: i32, buf: *const u8, n: usize) -> isize;
    fn close(fd: i32) -> i32;
}
