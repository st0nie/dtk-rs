//! Scoped, headless QtCore runtime. GUI DApplication's legacy lifecycle is
//! deliberately separate; this module does not promise GUI shutdown cleanup.
use crate::{Signal0, env_args_raw, require_core_application, require_main_application_thread};
use dtk_sys::ffi;
use std::marker::PhantomData;

/// A borrowed headless QCoreApplication, available only inside [`Self::with`].
///
/// The scope exclusively owns the native application. Unlike an owning app
/// handle it cannot be moved into a `'static` Qt callback and destroyed while
/// native event frames are still running. All constructor-owned QtCore objects
/// made by the binding in the scope are destroyed at the outer boundary, including
/// deferred deletes and unfired timers. Escaping guarded views/owners become
/// invalid; ordinary values such as QVariant may safely outlive the scope.
///
/// GUI widgets, DGui singleton APIs and QPixmap still require DApplication.
/// Call this on the process main thread, with no other Qt application alive.
///
/// ```no_run
/// use dtk::{QCoreApplication, QTimer};
/// QCoreApplication::with("service", |app| {
///     QTimer::single_shot(0, QCoreApplication::quit);
///     assert_eq!(app.exec(), 0);
/// }); // deterministic QtCore object/callback cleanup, even on panic
/// ```
///
/// The scope token cannot escape into callbacks:
/// ```compile_fail
/// use dtk::{QCoreApplication, QTimer};
/// QCoreApplication::with("service", |app| {
///     QTimer::single_shot(0, move || { app.exec(); });
/// });
/// ```
/// Nor can the borrowed token be returned from its scope:
/// ```compile_fail
/// use dtk::QCoreApplication;
/// let token = QCoreApplication::with("service", |app| app);
/// token.exec();
/// ```
pub struct QCoreApplication {
    runtime: *mut ffi::CoreRuntime,
    object: *mut ffi::QObject,
    _not_send: PhantomData<*mut ()>,
}

impl QCoreApplication {
    /// Run `f` with a borrowed QtCore application. Native objects are destroyed
    /// after `f` returns (or unwinds), not from inside native signal frames.
    /// The value returned by `f` may include guarded handles, but they will be
    /// invalid when this function returns. No nested/concurrent Qt apps allowed.
    pub fn with<R>(name: &str, f: impl FnOnce(&Self) -> R) -> R {
        require_main_application_thread();
        let (bytes, lengths) = env_args_raw();
        let owner = unsafe { ffi::core_runtime_new(name, bytes, lengths) };
        let runtime = owner
            .as_ref()
            .expect("only one Qt application can exist at a time");
        let app = Self {
            runtime: runtime as *const ffi::CoreRuntime as *mut ffi::CoreRuntime,
            object: unsafe { ffi::core_runtime_application(runtime) },
            _not_send: PhantomData,
        };
        let result = f(&app);
        // The UniquePtr also runs this cleanup if f panics. No public native
        // owner can be stolen/forgotten or moved into a callback from this API.
        drop(owner);
        result
    }

    /// Run Qt's native event loop. Recursive exec is rejected; after it returns
    /// the scope remains active until `with` finishes.
    pub fn exec(&self) -> i32 {
        require_core_application();
        // SAFETY: Self is only lent from with(); native owner remains live.
        assert!(
            unsafe { ffi::core_runtime_can_exec(&*self.runtime) },
            "QCoreApplication::exec cannot be called inside an active event loop"
        );
        unsafe { ffi::core_runtime_exec(self.runtime) }
    }

    /// Request exit of the currently running Qt loop on its owning thread.
    /// Suitable for a timer callback without capturing a borrowed scope token.
    pub fn quit() {
        require_core_application();
        unsafe { ffi::application_quit() };
    }

    /// Check an argument in Qt's current argv (without delimiter encoding).
    pub fn has_arg(arg: &str) -> bool {
        require_core_application();
        unsafe { ffi::application_has_arg(arg) }
    }
}

unsafe impl Signal0 for QCoreApplication {
    fn qobject_ptr(&self) -> *mut ffi::QObject {
        require_core_application();
        self.object
    }
}
