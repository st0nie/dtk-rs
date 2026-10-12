//! Borrowed Qt application scopes. The private native owner is destroyed only
//! after user code and all native exec/event/FFI frames have returned.
use crate::{
    Signal0, env_args_raw, require_application, require_core_application,
    require_main_application_thread,
};
use dtk_sys::ffi;
use std::marker::PhantomData;

/// Enter the common native runtime. GUI and headless scopes have the same
/// singleton reservation, argv ownership, panic cleanup and closing protocol.
fn with_runtime<R>(
    name: &str,
    gui: bool,
    quit_guard_id: usize,
    f: impl FnOnce(&ffi::ApplicationRuntime) -> R,
) -> R {
    require_main_application_thread();
    let (bytes, lengths) = env_args_raw();
    let owner = unsafe { ffi::application_runtime_new(name, bytes, lengths, gui, quit_guard_id) };
    let Some(runtime) = owner.as_ref() else {
        dtk_sys::unregister_cb(quit_guard_id);
        panic!("only one Qt application can exist at a time");
    };
    let result = f(runtime);
    // UniquePtr runs the same cleanup if f unwinds. No public owner can be
    // moved into a callback, leaked with forget(), or explicitly destroyed.
    drop(owner);
    result
}

fn exec(runtime: *mut ffi::ApplicationRuntime, name: &str) -> i32 {
    require_core_application();
    // SAFETY: runtime is private to a token borrowed inside with_runtime().
    assert!(
        unsafe { ffi::application_runtime_can_exec(&*runtime) },
        "{name}::exec cannot be called inside an active event loop"
    );
    unsafe { ffi::application_runtime_exec(runtime) }
}

/// A borrowed GUI DApplication, available only inside [`Self::with`] or
/// [`Self::with_quit_guard`], on the Linux process main thread.
///
/// The scope exclusively owns the native QApplication. Its token cannot enter
/// a `'static` Qt callback or escape the scope, so native application deletion
/// never runs inside a signal/event stack. On normal return or Rust unwinding,
/// cleanup first rejects reentry and disables callbacks, then synchronously
/// destroys top-level windows, constructed QObjects and application children
/// (including pending timers). No `exec()` or extra event-loop turn is needed.
///
/// QObject views/owners may escape but become invalid, including borrowed DTK
/// singletons that native code keeps alive across applications. Borrowed
/// singletons are not deleted; their binding-owned connections are reclaimed.
/// `exec()` exit alone does not close the scope: objects remain usable until
/// the outer closure returns. Native reentrant deletion during ordinary Qt
/// method calls and non-QObject borrowed lifetimes still need separate audits.
///
/// ```no_run
/// use dtk::{DApplication, DMainWindow, QTimer};
/// let code = DApplication::with("my-app", |app| {
///     let window = DMainWindow::new();
///     window.show();
///     QTimer::single_shot(0, DApplication::quit);
///     app.exec()
/// }); // native cleanup has completed before code is returned
/// std::process::exit(code); // never exit() from inside the scope
/// ```
///
/// The application token cannot be captured by a static callback:
/// ```compile_fail
/// use dtk::{DApplication, QTimer};
/// DApplication::with("app", |app| {
///     QTimer::single_shot(0, move || { app.exec(); });
/// });
/// ```
/// Nor can the borrowed token escape the outer closure:
/// ```compile_fail
/// use dtk::DApplication;
/// let app = DApplication::with("app", |app| app);
/// app.exec();
/// ```
/// It also cannot be returned by the quit-guard variant:
/// ```compile_fail
/// use dtk::DApplication;
/// let app = DApplication::with_quit_guard("app", || true, |app| app);
/// app.exec();
/// ```
/// There is no owning/Clone token to put in long-lived state:
/// ```compile_fail
/// use dtk::DApplication;
/// DApplication::with("app", |app| {
///     let owned: DApplication = app.clone();
///     drop(owned);
/// });
/// ```
pub struct DApplication {
    runtime: *mut ffi::ApplicationRuntime,
    ptr: *mut ffi::DApplication,
    object: *mut ffi::QObject,
    _not_send: PhantomData<*mut ()>,
}

impl DApplication {
    /// Run with one borrowed GUI application. Nested/concurrent applications
    /// and construction on a worker thread are rejected before native creation.
    pub fn with<R>(name: &str, f: impl FnOnce(&Self) -> R) -> R {
        Self::with_guard_id(name, 0, f)
    }

    /// Like [`Self::with`], but QEvent::Quit asks `guard`; returning false
    /// swallows the quit event. Retry later via a timer/static quit method.
    /// Captures are released at scope exit, even without an event loop.
    pub fn with_quit_guard<R>(
        name: &str,
        guard: impl FnMut() -> bool + 'static,
        f: impl FnOnce(&Self) -> R,
    ) -> R {
        // Reject before registering: otherwise wrong-thread/nested startup
        // would strand a boxed guard in that thread's callback registry.
        require_main_application_thread();
        assert!(
            unsafe { ffi::application_can_construct() },
            "only one Qt application can exist at a time"
        );
        let id = dtk_sys::register_cb_guard(guard);
        Self::with_guard_id(name, id, f)
    }

    fn with_guard_id<R>(name: &str, guard_id: usize, f: impl FnOnce(&Self) -> R) -> R {
        with_runtime(name, true, guard_id, |runtime| {
            let app = Self {
                runtime: runtime as *const _ as *mut _,
                ptr: unsafe { ffi::application_runtime_gui_application(runtime) },
                object: unsafe { ffi::application_runtime_application(runtime) },
                _not_send: PhantomData,
            };
            f(&app)
        })
    }

    /// Run the native event loop. Recursive exec (including from a modal loop)
    /// is rejected. The scope and its objects remain active after exec returns.
    pub fn exec(&self) -> i32 {
        require_application();
        exec(self.runtime, "DApplication")
    }

    /// Request exit on the owning GUI thread without capturing the scope token.
    pub fn quit() {
        require_application();
        unsafe { ffi::application_quit() }
    }
    pub fn set_quit_on_last_window_closed(quit: bool) {
        require_application();
        unsafe { ffi::application_set_quit_on_last_window_closed(quit) }
    }
    pub fn set_application_display_name(name: &str) {
        require_application();
        unsafe { ffi::application_set_application_display_name(name) }
    }
    /// DTK translations (zh_CN etc.); false on failure.
    pub fn load_translator(&self) -> bool {
        require_application();
        unsafe { ffi::application_load_translator(self.ptr) }
    }
    pub fn has_arg(arg: &str) -> bool {
        require_application();
        unsafe { ffi::application_has_arg(arg) }
    }
    /// True while a modal dialog or popup menu is active.
    pub fn popup_active() -> bool {
        require_application();
        unsafe { ffi::app_popup_active() }
    }
    /// QPalette::Window as (r, g, b), for light/dark theme detection.
    pub fn palette_window_rgb() -> (u8, u8, u8) {
        require_application();
        let rgb = unsafe { ffi::app_palette_window_rgb() };
        ((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8)
    }
    /// DPalette::Highlight as (r, g, b), the theme accent color.
    pub fn palette_highlight_rgb() -> (u8, u8, u8) {
        require_application();
        let rgb = unsafe { ffi::app_palette_highlight_rgb() };
        ((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8)
    }
}

unsafe impl Signal0 for DApplication {
    fn qobject_ptr(&self) -> *mut ffi::QObject {
        require_application();
        self.object
    }
}

/// A borrowed headless QCoreApplication, available only inside [`Self::with`].
///
/// It uses the same scoped ownership/cleanup protocol as [`DApplication`],
/// without a display or Qt platform plugin. Constructor-owned QtCore objects,
/// pending timers and their captures are destroyed at the outer boundary.
/// Escaping guarded views become invalid; QtCore values such as QVariant may
/// outlive the scope. GUI types and DGui singletons still require DApplication.
///
/// ```no_run
/// use dtk::{QCoreApplication, QTimer};
/// QCoreApplication::with("service", |app| {
///     QTimer::single_shot(0, QCoreApplication::quit);
///     assert_eq!(app.exec(), 0);
/// });
/// ```
///
/// The scope token cannot enter callbacks:
/// ```compile_fail
/// use dtk::{QCoreApplication, QTimer};
/// QCoreApplication::with("service", |app| {
///     QTimer::single_shot(0, move || { app.exec(); });
/// });
/// ```
/// Nor can it be returned from its scope:
/// ```compile_fail
/// use dtk::QCoreApplication;
/// let token = QCoreApplication::with("service", |app| app);
/// token.exec();
/// ```
pub struct QCoreApplication {
    runtime: *mut ffi::ApplicationRuntime,
    object: *mut ffi::QObject,
    _not_send: PhantomData<*mut ()>,
}

impl QCoreApplication {
    /// Run with a borrowed headless application on the process main thread.
    /// Guarded handles returned by `f` are invalid when this function returns.
    pub fn with<R>(name: &str, f: impl FnOnce(&Self) -> R) -> R {
        with_runtime(name, false, 0, |runtime| {
            let app = Self {
                runtime: runtime as *const _ as *mut _,
                object: unsafe { ffi::application_runtime_application(runtime) },
                _not_send: PhantomData,
            };
            f(&app)
        })
    }

    /// Run the event loop; recursive exec is rejected.
    pub fn exec(&self) -> i32 {
        exec(self.runtime, "QCoreApplication")
    }
    /// Request exit on the owning thread without capturing the scope token.
    pub fn quit() {
        require_core_application();
        unsafe { ffi::application_quit() };
    }
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
