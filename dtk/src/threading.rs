//! Typed worker-to-Qt delivery. Qt objects remain on their owning thread;
//! workers send only `Send` values. Supports both the GUI application and a
//! scoped headless `QCoreApplication` owner. Linux eventfd is an implementation detail, not an
//! application-level fd or a polling timer.
use crate::{QSocketNotifier, Signal0};
use std::io;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};

struct Wake {
    fd: OwnedFd,
    pending: AtomicBool,
    closed: AtomicBool,
}

impl Wake {
    fn signal(&self) -> io::Result<()> {
        // One wakeup per batch, not one syscall/Qt activation per message.
        if self.pending.swap(true, Ordering::AcqRel) {
            return Ok(());
        }
        let one = 1u64;
        loop {
            let n = unsafe {
                libc::write(
                    self.fd.as_raw_fd(),
                    (&one as *const u64).cast(),
                    std::mem::size_of_val(&one),
                )
            };
            if n == 8 {
                return Ok(());
            }
            let error = if n < 0 {
                io::Error::last_os_error()
            } else {
                io::Error::new(io::ErrorKind::WriteZero, "short eventfd write")
            };
            if error.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            if error.kind() == io::ErrorKind::WouldBlock {
                // Saturated counter is already readable; the GUI will drain it.
                return Ok(());
            }
            self.pending.store(false, Ordering::Release);
            self.closed.store(true, Ordering::Release);
            return Err(error);
        }
    }

    fn drain(&self) -> io::Result<()> {
        let mut count = 0u64;
        loop {
            let n = unsafe {
                libc::read(
                    self.fd.as_raw_fd(),
                    (&mut count as *mut u64).cast(),
                    std::mem::size_of_val(&count),
                )
            };
            if n == 8 {
                continue;
            }
            let error = if n < 0 {
                io::Error::last_os_error()
            } else {
                io::Error::new(io::ErrorKind::UnexpectedEof, "short eventfd read")
            };
            if error.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            if error.kind() == io::ErrorKind::WouldBlock {
                return Ok(());
            }
            self.closed.store(true, Ordering::Release);
            self.pending.store(false, Ordering::Release);
            return Err(error);
        }
    }
}

/// A cloneable, thread-safe sender for values handled on the owner's Qt thread.
/// Once the owner is destroyed, its callback/receiver is dropped and `send` fails.
pub struct GuiSender<T: Send + 'static> {
    tx: mpsc::Sender<T>,
    wake: Arc<Wake>,
}

impl<T: Send + 'static> Clone for GuiSender<T> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
            wake: self.wake.clone(),
        }
    }
}

impl<T: Send + 'static> GuiSender<T> {
    /// Create on the Qt object's thread. `on_message` runs there, never on a worker.
    /// The notifier and receiver live until `owner` is destroyed. A scoped
    /// `QCoreApplication` may be the owner for headless service delivery.
    pub fn new(owner: &impl Signal0, mut on_message: impl FnMut(T) + 'static) -> io::Result<Self> {
        let fd = unsafe { libc::eventfd(0, libc::EFD_NONBLOCK | libc::EFD_CLOEXEC) };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        let wake = Arc::new(Wake {
            fd: unsafe { OwnedFd::from_raw_fd(fd) },
            pending: AtomicBool::new(false),
            closed: AtomicBool::new(false),
        });
        let (tx, rx) = mpsc::channel();
        let notifier = QSocketNotifier::new(wake.fd.as_raw_fd(), owner);
        let gui_wake = wake.clone();
        let id = notifier.on_activated(move || {
            if let Err(error) = gui_wake.drain() {
                eprintln!("dtk-rs: GUI wakeup fd failed: {error}");
                return;
            }
            // Reset before draining the channel. Any concurrent sender after this
            // point will post another wakeup, so no message can be stranded.
            gui_wake.pending.store(false, Ordering::Release);
            // Bound work per event-loop turn so a fast worker cannot starve UI input.
            const MAX_PER_TURN: usize = 256;
            let mut processed = 0;
            for msg in rx.try_iter().take(MAX_PER_TURN) {
                processed += 1;
                // Panicking in a handler must not strand the remaining messages.
                if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| on_message(msg)))
                    .is_err()
                {
                    eprintln!("dtk-rs: GUI message handler panicked");
                }
            }
            if processed == MAX_PER_TURN {
                // There may be more queued messages even when all workers have
                // finished. Schedule another turn (a harmless extra wake if empty).
                let _ = gui_wake.signal();
            }
        });
        if id == 0 {
            notifier.delete_later();
            return Err(io::Error::other(
                "Qt QSocketNotifier activation signal unavailable",
            ));
        }
        Ok(Self { tx, wake })
    }

    /// Enqueue a value from any thread, including the GUI thread. Successful
    /// enqueue is not a delivery guarantee if the Qt owner is destroyed before
    /// dispatch. Requires a running event loop; returns BrokenPipe after deletion.
    pub fn send(&self, value: T) -> io::Result<()> {
        if self.wake.closed.load(Ordering::Acquire) {
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "GUI wakeup closed",
            ));
        }
        self.tx
            .send(value)
            .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "GUI owner destroyed"))?;
        self.wake.signal()
    }
}
