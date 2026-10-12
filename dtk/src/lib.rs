//! Safe Rust bindings for DTK6 (dtkwidget).
//!
//! Lifetime: Qt parents own their children; every `T::new()`/similar constructor
//! returns `Owned<T>`, which queues Qt deletion on drop *unless*
//! Qt ever adopted it via a parent (added to a layout, set as a central widget,
//! passed as another constructor's parent, ...) — the QObject guard records
//! parent changes so an earlier owner never reclaims a detached object. `Owned<T>` derefs
//! to the plain, `Clone`-able, non-owning view; share that (or an `Rc<Owned<T>>`)
//! into closures/long-lived structs. Every view (owning or not) still must not
//! be used after Qt destroys the object; a `QPointer`-backed guard makes that a
//! panic instead of a dangling dereference. A running event loop executes the
//! `deleteLater()` queued by an unadopted owner. Independently, DApplication::with
//! and QCoreApplication::with synchronously clean up at their outer boundary,
//! including no-exec/panic; escaped controls and singleton views remain dead.
//! Qt GUI objects are single-threaded; wrappers are !Send/!Sync.

use dtk_sys::ffi;
use std::marker::PhantomData;
use std::rc::Rc;

mod application;
pub use application::{DApplication, QCoreApplication};
pub mod threading;
pub use threading::GuiSender;

/// null for None (Qt parent pointer)
fn opt_ptr(w: Option<&QWidget>) -> *mut ffi::QWidget {
    require_application();
    w.map_or(std::ptr::null_mut(), QWidget::live_ptr)
}

/// Reconstitute an owned Qt QHash<QString, QString> result encoded as adjacent
/// UTF-8 key/value strings; no delimiter or borrowed Qt storage is involved.
pub(crate) fn string_map_from_pairs(
    values: Vec<String>,
) -> std::collections::BTreeMap<String, String> {
    let mut pairs = values.into_iter();
    let mut map = std::collections::BTreeMap::new();
    while let Some(key) = pairs.next() {
        let value = pairs.next().expect("C++ map returned odd key/value count");
        map.insert(key, value);
    }
    map
}

#[cfg(test)]
mod string_map_tests {
    use super::string_map_from_pairs;

    #[test]
    fn binary_safe_owned_pairs() {
        let map = string_map_from_pairs(vec![
            "键|名\0尾".to_owned(),
            "值\0中文".to_owned(),
            "empty".to_owned(),
            String::new(),
        ]);
        assert_eq!(map.get("键|名\0尾").map(String::as_str), Some("值\0中文"));
        assert_eq!(map.get("empty").map(String::as_str), Some(""));
    }

    #[test]
    #[should_panic(expected = "odd key/value count")]
    fn rejects_malformed_native_pairs() {
        string_map_from_pairs(vec!["orphan".to_owned()]);
    }
}

/// Qt widgets and event-driven QObjects require the QApplication's own GUI
/// thread, not merely a process-global QApplication created elsewhere.
pub(crate) fn require_application() {
    assert!(
        unsafe { ffi::application_is_gui() },
        "create DApplication before constructing Qt objects and call them on its GUI thread"
    );
}

/// QtCore objects may use a headless scoped QCoreApplication or the GUI app,
/// but still require that runtime's owning thread and a non-closing scope.
pub(crate) fn require_core_application() {
    assert!(
        unsafe { ffi::application_is_core() },
        "create QCoreApplication or DApplication before constructing QtCore objects and use its owning Qt thread"
    );
}

fn require_main_application_thread() {
    assert!(
        unsafe { ffi::application_is_main_thread() },
        "Qt applications must be constructed on the process main thread"
    );
}

/// `impl Default` for wrappers whose `new()` is the default ctor
macro_rules! impl_default {
    ($($name:ident),* $(,)?) => {
        $(impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        })*
    };
}
// Generated classes use C++ static_cast for secondary Qt bases; hand-bound
// Qt classes have QObject/QWidget as their primary (offset-zero) base.
macro_rules! qt_object_base {
    ($ptr:expr) => { $ptr.cast::<ffi::QObject>() };
    ($ptr:expr, $cast:path) => { unsafe { $cast($ptr) } as *mut ffi::QObject };
}
macro_rules! qt_widget_base_ty {
    (object) => { () };
    (widget) => { *mut ffi::QWidget };
}
macro_rules! qt_widget_base {
    (object, $ptr:expr) => { () };
    (widget, $ptr:expr) => { $ptr.cast::<ffi::QWidget>() };
    (widget, $ptr:expr, $cast:path) => { unsafe { $cast($ptr) } as *mut ffi::QWidget };
}

macro_rules! widget_wrapper {
    ($name:ident, $ffi:ty $(, $object_cast:path, $widget_cast:path)?) => {
        object_wrapper_impl!($name, $ffi, widget, $crate::require_application $(, $object_cast)? ; $($widget_cast)?);
        impl $name {
            pub(crate) fn widget_ptr(&self) -> *mut ffi::QWidget {
                self.live_ptr();
                self._widget
            }
            /// Share the existing guard with a QWidget base-class view.
            pub fn as_widget(&self) -> QWidget {
                QWidget::from_guarded(self.widget_ptr(), self.guard.clone())
            }
            pub fn show(&self) {
                unsafe { ffi::widget_show(self.widget_ptr()) }
            }
            pub fn hide(&self) {
                unsafe { ffi::widget_hide(self.widget_ptr()) }
            }
            pub fn resize(&self, w: i32, h: i32) {
                unsafe { ffi::widget_resize(self.widget_ptr(), w, h) }
            }
            /// widget size in pixels (window: includes the DTK titlebar)
            pub fn width(&self) -> i32 {
                unsafe { ffi::widget_width(self.widget_ptr()) }
            }
            pub fn height(&self) -> i32 {
                unsafe { ffi::widget_height(self.widget_ptr()) }
            }
            pub fn move_to(&self, x: i32, y: i32) {
                unsafe { ffi::widget_move(self.widget_ptr(), x, y) }
            }
            pub fn set_enabled(&self, on: bool) {
                unsafe { ffi::widget_set_enabled(self.widget_ptr(), on) }
            }
            pub fn set_window_title(&self, title: &str) {
                unsafe { ffi::widget_set_window_title(self.widget_ptr(), title) }
            }
            pub fn set_window_icon(&self, icon: &QIcon) {
                unsafe { ffi::widget_set_window_icon(self.widget_ptr(), icon.ptr) }
            }
            pub fn set_fixed_size(&self, w: i32, h: i32) {
                unsafe { ffi::widget_set_fixed_size(self.widget_ptr(), w, h) }
            }
            pub fn raise(&self) {
                unsafe { ffi::widget_raise(self.widget_ptr()) }
            }
            pub fn update(&self) {
                unsafe { ffi::widget_update(self.widget_ptr()) }
            }
            pub fn set_focus(&self) {
                unsafe { ffi::widget_set_focus(self.widget_ptr()) }
            }
            pub fn activate_window(&self) {
                unsafe { ffi::widget_activate_window(self.widget_ptr()) }
            }
            pub fn close(&self) {
                unsafe { ffi::widget_close(self.widget_ptr()) }
            }
            pub fn is_visible(&self) -> bool {
                unsafe { ffi::widget_is_visible(self.widget_ptr()) }
            }
            pub fn set_focus_policy(&self, policy: i32) {
                unsafe { ffi::widget_set_focus_policy(self.widget_ptr(), policy) }
            }
            pub fn set_font(&self, font: &QFont) {
                unsafe { ffi::widget_set_font(self.widget_ptr(), font.ptr) }
            }
            pub fn set_cursor(&self, shape: i32) {
                unsafe { ffi::widget_set_cursor(self.widget_ptr(), shape) }
            }
            pub fn unset_cursor(&self) {
                unsafe { ffi::widget_unset_cursor(self.widget_ptr()) }
            }
            pub fn palette(&self) -> QPalette {
                QPalette::from_raw(unsafe { ffi::widget_palette(self.widget_ptr()) })
            }
            pub fn set_palette(&self, pal: &QPalette) {
                unsafe { ffi::widget_set_palette(self.widget_ptr(), pal.ptr) }
            }
            pub fn standard_icon_pixmap(&self, icon: i32, size: i32) -> QPixmap {
                QPixmap::from_raw(unsafe { ffi::standard_icon_pixmap(self.widget_ptr(), icon, size) })
            }
        }
    };
}

/// Owns exactly one heap allocation returned by a generated non-QObject
/// constructor. Unlike a borrowed pointer getter, this calls its C++ deleter.
/// Dereferencing exposes only a shared view; ownership is never duplicated.
pub struct OwnedOpaque<T> {
    inner: T,
    deleter: fn(&T),
}

impl<T> OwnedOpaque<T> {
    pub(crate) fn from_owned(inner: T, deleter: fn(&T)) -> Self {
        Self { inner, deleter }
    }
}

impl<T> std::ops::Deref for OwnedOpaque<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> Drop for OwnedOpaque<T> {
    fn drop(&mut self) {
        (self.deleter)(&self.inner);
    }
}

#[cfg(test)]
mod owned_opaque_tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn unique_owner_calls_deleter_once() {
        struct Probe(Rc<Cell<usize>>);
        let drops = Rc::new(Cell::new(0));
        let owner = OwnedOpaque::from_owned(Probe(drops.clone()), |probe| {
            probe.0.set(probe.0.get() + 1);
        });
        let borrowed = &*owner;
        assert_eq!(borrowed.0.get(), 0);
        drop(owner);
        assert_eq!(drops.get(), 1);
    }
}

/// Implemented by every QObject/QWidget wrapper this crate can put behind
/// [`Owned<T>`]. `live_qobject()` distinguishes "Qt already destroyed this"
/// from "still alive", so `Owned::drop` never touches a dangling pointer.
/// # Safety
/// A `Some(ptr)` result must be a currently live, correctly base-adjusted
/// `QObject*` on its owning Qt thread. `was_parented()` must not return false
/// once Qt adopted the object, even if Qt later returned or detached it:
/// otherwise an old constructor owner could delete a newly transferred object.
/// Returning a stale or wrongly adjusted pointer also passes invalid memory
/// into Qt from `Owned::drop`.
pub unsafe trait QtLifecycle {
    fn live_qobject(&self) -> Option<*mut ffi::QObject>;
    /// Qt has adopted this particular object at least once since this view
    /// was created. An old owner cannot reclaim it after a later takeXxx().
    fn was_parented(&self) -> bool;
}

/// Uniquely governs a freshly constructed Qt QObject/QWidget until Qt itself
/// takes ownership by giving it a parent — added to a layout that is itself
/// installed on a widget, set as a central widget, passed as another
/// object's constructor parent, reparented via `set_parent`, and so on. The
/// QPointer guard watches Qt ParentChange events: **after the first adoption**
/// this particular constructor owner permanently relinquishes deletion, even
/// if Qt later detaches the object. An individually audited `take` method
/// may return a new owner; other pointer getters stay borrowed.
///
/// Dropping an object still alive and *never* parented queues Qt `deleteLater`.
/// This needs an active event loop to actually delete it; dropping an object
/// after the event loop stops delays destruction until the outer application
/// scope exits. Both GUI and headless scopes synchronously reclaim tracked
/// objects there, even without exec. Drop itself uses deferred deletion rather
/// than synchronous `delete`, which could destroy native event-handler frames.
///
/// `Deref`s to the plain, `Clone`-able view: clone that (or call a method
/// through the deref, which itself resolves via `Clone`) to share the object
/// into closures, layouts, or long-lived structs. Those clones never delete
/// anything themselves — only this unique owner does, and only once.
#[must_use = "dropping this queues deletion if Qt never adopted this object"]
pub struct Owned<T: QtLifecycle>(T);

impl<T: QtLifecycle> Owned<T> {
    pub(crate) fn new(inner: T) -> Self {
        if let Some(object) = inner.live_qobject() {
            // Only verified constructor/transfer owners enter the scoped
            // runtime's cleanup registry. Borrowed getters must not enter it.
            unsafe { ffi::application_runtime_track_object(object) };
        }
        Self(inner)
    }
}

impl<T: QtLifecycle> std::ops::Deref for Owned<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: QtLifecycle + std::fmt::Debug> std::fmt::Debug for Owned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl<T: QtLifecycle> Drop for Owned<T> {
    fn drop(&mut self) {
        if let Some(obj) = self.0.live_qobject() {
            // SAFETY: `obj` is live per QtLifecycle's contract just above.
            // Ownership never travels backward: once Qt adopted the object,
            // only a fresh owner returned by takeXxx() may schedule deletion.
            if !self.0.was_parented() && !unsafe { ffi::object_has_parent(obj) } {
                unsafe { ffi::object_delete_later(obj) };
            }
        }
    }
}

// Every generic `&impl Signal0`/`<L: AsLayout>` call site must keep accepting
// an `Owned<T>` reference exactly like a plain view (see e.g. `QTimer::new`,
// `QAction::new`, `layout.add_layout`): delegate straight to the inner view.
unsafe impl<T: Signal0 + QtLifecycle> Signal0 for Owned<T> {
    fn qobject_ptr(&self) -> *mut ffi::QObject {
        self.0.qobject_ptr()
    }
}
unsafe impl<T: SignalI32 + QtLifecycle> SignalI32 for Owned<T> {
    fn qobject_ptr(&self) -> *mut ffi::QObject {
        self.0.qobject_ptr()
    }
}
unsafe impl<T: SignalBool + QtLifecycle> SignalBool for Owned<T> {
    fn qobject_ptr(&self) -> *mut ffi::QObject {
        self.0.qobject_ptr()
    }
}
unsafe impl<T: SignalI32I32 + QtLifecycle> SignalI32I32 for Owned<T> {
    fn qobject_ptr(&self) -> *mut ffi::QObject {
        self.0.qobject_ptr()
    }
}
unsafe impl<T: AsLayout + QtLifecycle> AsLayout for Owned<T> {
    fn layout_ptr(&self) -> *mut ffi::QLayout {
        self.0.layout_ptr()
    }
}

/// `impl Default` for `Owned<T>` wrappers whose `T::new()` is the default ctor
macro_rules! impl_default_owned {
    ($($name:ident),* $(,)?) => {
        $(impl Default for Owned<$name> {
            fn default() -> Self {
                $name::new()
            }
        })*
    };
}
pub(crate) use impl_default_owned;

macro_rules! opaque_wrapper_impl {
    ($name:ident, $ffi:ty, $($attrs:tt)*) => {
        /// Non-owning C++ pointer view; QObject methods are added separately
        /// only for types with a verified Qt ancestor.
        $($attrs)*
        pub struct $name {
            pub(crate) ptr: *mut $ffi,
            scope_guard: Option<dtk_sys::ObjectGuard>,
            _not_send: PhantomData<*mut ()>,
        }
        impl $name {
            #[allow(dead_code)] // borrowed getters are not emitted for every class
            pub(crate) fn from_raw(ptr: *mut $ffi) -> Self {
                let mut view = Self::from_owned_raw(ptr);
                view.scope_guard = dtk_sys::ObjectGuard::current_scope();
                view
            }
            #[allow(dead_code)] // constructors/value copies are not emitted for every class
            pub(crate) fn from_owned_raw(ptr: *mut $ffi) -> Self {
                assert!(!ptr.is_null());
                Self { ptr, scope_guard: None, _not_send: PhantomData }
            }
            /// Scope liveness only: non-QObject native lifetime within the
            /// scope is still governed by the unsafe getter's contract.
            pub fn is_alive(&self) -> bool {
                self.scope_guard.as_ref().is_none_or(dtk_sys::ObjectGuard::is_alive)
            }
            #[allow(dead_code)] // some opaque types expose only static methods
            pub(crate) fn live_ptr(&self) -> *mut $ffi {
                dtk_sys::assert_not_closing();
                assert!(self.is_alive(), concat!(stringify!($name), " application scope has closed"));
                self.ptr
            }
        }
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, concat!(stringify!($name), "({:p})"), self.ptr)
            }
        }
    };
}

// No Copy/Clone on opaque pointer views: an OwnedOpaque<T> must not be
// duplicated through Deref, and an unguarded borrowed view must not be
// inadvertently propagated beyond its native owner's lifetime.
macro_rules! opaque_wrapper {
    ($name:ident, $ffi:ty) => {
        opaque_wrapper_impl!($name, $ffi,);
    };
}

macro_rules! object_wrapper_impl {
    ($name:ident, $ffi:ty, $kind:ident, $runtime_check:path $(, $object_cast:path)? ; $($widget_cast:path)?) => {
        /// Non-owning checked view; the Qt parent or explicit delete_later
        /// manages the QObject, while clones share the same QPointer guard.
        #[derive(Clone)]
        pub struct $name {
            pub(crate) ptr: *mut $ffi,
            qobject: *mut ffi::QObject,
            _widget: qt_widget_base_ty!($kind),
            pub(crate) guard: std::rc::Rc<dtk_sys::ObjectGuard>,
            _not_send: PhantomData<*mut ()>,
        }
        impl $name {
            #[allow(dead_code)]
            pub(crate) fn from_raw(ptr: *mut $ffi) -> Self {
                $runtime_check();
                assert!(!ptr.is_null());
                let qobject = qt_object_base!(ptr $(, $object_cast)?);
                let widget = qt_widget_base!($kind, ptr $(, $widget_cast)?);
                Self {
                    ptr,
                    qobject,
                    _widget: widget,
                    guard: std::rc::Rc::new(unsafe { dtk_sys::ObjectGuard::from_object(qobject) }),
                    _not_send: PhantomData,
                }
            }
            pub fn is_alive(&self) -> bool {
                self.guard.is_alive()
            }
            pub(crate) fn live_ptr(&self) -> *mut $ffi {
                assert!(self.is_alive(), concat!(stringify!($name), " was destroyed by Qt"));
                self.ptr
            }
            #[allow(dead_code)]
            pub(crate) fn as_qobject(&self) -> *mut ffi::QObject {
                self.live_ptr();
                self.qobject
            }
            /// Defer QObject destruction to Qt's event loop.
            pub fn delete_later(&self) {
                unsafe { ffi::object_delete_later(self.as_qobject()) }
            }
        }
        unsafe impl Signal0 for $name {
            fn qobject_ptr(&self) -> *mut ffi::QObject { self.as_qobject() }
        }
        unsafe impl SignalI32 for $name {
            fn qobject_ptr(&self) -> *mut ffi::QObject { self.as_qobject() }
        }
        unsafe impl SignalBool for $name {
            fn qobject_ptr(&self) -> *mut ffi::QObject { self.as_qobject() }
        }
        unsafe impl SignalI32I32 for $name {
            fn qobject_ptr(&self) -> *mut ffi::QObject { self.as_qobject() }
        }
        unsafe impl QtLifecycle for $name {
            fn live_qobject(&self) -> Option<*mut ffi::QObject> {
                self.is_alive().then_some(self.qobject)
            }
            fn was_parented(&self) -> bool {
                self.guard.was_parented()
            }
        }
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, concat!(stringify!($name), "({:p})"), self.ptr)
            }
        }
    };
}

macro_rules! object_wrapper {
    ($name:ident, $ffi:ty $(, $object_cast:path)?) => {
        object_wrapper_impl!($name, $ffi, object, $crate::require_application $(, $object_cast)? ;);
    };
}

// These explicitly audited QObject classes need only QtCore, not QtWidgets.
macro_rules! core_object_wrapper {
    ($name:ident, $ffi:ty $(, $object_cast:path)?) => {
        object_wrapper_impl!($name, $ffi, object, $crate::require_core_application $(, $object_cast)? ;);
    };
}

/// Non-owning QWidget view; Qt owns the object, while the shared QPointer guard
/// prevents this handle and its clones from dereferencing it after destruction.
#[derive(Clone)]
pub struct QWidget {
    pub(crate) ptr: *mut ffi::QWidget,
    guard: Rc<dtk_sys::ObjectGuard>,
    _not_send: PhantomData<*mut ()>,
}

impl QWidget {
    pub(crate) fn from_raw(ptr: *mut ffi::QWidget) -> Self {
        require_application();
        assert!(!ptr.is_null());
        Self::from_guarded(
            ptr,
            Rc::new(unsafe { dtk_sys::ObjectGuard::from_widget(ptr) }),
        )
    }

    pub(crate) fn from_guarded(ptr: *mut ffi::QWidget, guard: Rc<dtk_sys::ObjectGuard>) -> Self {
        assert!(!ptr.is_null() && guard.is_alive());
        Self {
            ptr,
            guard,
            _not_send: PhantomData,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.guard.is_alive()
    }

    pub(crate) fn live_ptr(&self) -> *mut ffi::QWidget {
        assert!(self.is_alive(), "QWidget was destroyed by Qt");
        self.ptr
    }
    /// Plain top-level or child QWidget. Wrap the parent argument in `Some`
    /// to install it directly as a Qt child; drop the returned `Owned<QWidget>`
    /// without ever parenting it and Qt deletes it for you.
    pub fn new(parent: Option<&QWidget>) -> Owned<QWidget> {
        let p = opt_ptr(parent);
        Owned::new(Self::from_raw(unsafe { ffi::widget_new(p) }))
    }
    pub fn show(&self) {
        unsafe { ffi::widget_show(self.live_ptr()) }
    }
    /// deliver mouse moves without a button held (they propagate to the parent)
    pub fn set_mouse_tracking(&self, on: bool) {
        unsafe { ffi::widget_set_mouse_tracking(self.live_ptr(), on) }
    }
    pub fn hide(&self) {
        unsafe { ffi::widget_hide(self.live_ptr()) }
    }
    pub fn width(&self) -> i32 {
        unsafe { ffi::widget_width(self.live_ptr()) }
    }
    pub fn height(&self) -> i32 {
        unsafe { ffi::widget_height(self.live_ptr()) }
    }
    /// schedule a repaint
    pub fn update(&self) {
        unsafe { ffi::widget_update(self.live_ptr()) }
    }
    /// grab keyboard focus
    pub fn set_focus(&self) {
        unsafe { ffi::widget_set_focus(self.live_ptr()) }
    }
    /// qt::cursor::* shape; stays until unset_cursor
    pub fn set_cursor(&self, shape: i32) {
        unsafe { ffi::widget_set_cursor(self.live_ptr(), shape) }
    }
    pub fn unset_cursor(&self) {
        unsafe { ffi::widget_unset_cursor(self.live_ptr()) }
    }
    /// deepin-terminal style: install the tab bar as titlebar custom widget
    /// (zero-margin layout, vertically centered)
    pub fn titlebar_set_tabbar(&self, tabbar: &QWidget) {
        unsafe { ffi::main_window_titlebar_set_tabbar(self.live_ptr(), tabbar.live_ptr()) }
    }
    /// add a widget into the titlebar, left-aligned (DMainWindow only)
    pub fn titlebar_add_widget(&self, child: &QWidget) {
        unsafe { ffi::main_window_titlebar_add_widget(self.live_ptr(), child.live_ptr()) }
    }
    /// set the titlebar icon (DMainWindow only; no-op otherwise)
    pub fn set_titlebar_icon(&self, icon: &QIcon) {
        unsafe { ffi::widget_set_titlebar_icon(self.live_ptr(), icon.ptr) }
    }
    /// IME candidate window anchor: cursor rect in widget coords (PaintWidget only)
    pub fn set_ime_cursor_rect(&self, x: i32, y: i32, w: i32, h: i32) {
        unsafe { ffi::paint_widget_set_ime_rect(self.live_ptr(), x, y, w, h) }
    }
    pub fn resize(&self, w: i32, h: i32) {
        unsafe { ffi::widget_resize(self.live_ptr(), w, h) }
    }
    pub fn move_to(&self, x: i32, y: i32) {
        unsafe { ffi::widget_move(self.live_ptr(), x, y) }
    }
    /// deepin-terminal-style tab labels: centered, elided clear of the close button
    pub fn install_tab_label_style(&self) {
        unsafe { ffi::tabbar_install_style(self.live_ptr()) }
    }
    /// force the tab bar's internal layout (and ancestors) to relayout now, not
    /// on the next event pass; call after add/remove/set_tab_text to avoid a
    /// 1-frame misaligned paint
    pub fn flush_layout(&self) {
        unsafe { ffi::tabbar_flush_layout(self.live_ptr()) }
    }
    /// unlatch QTabBar's scroll buttons: once shown, their own width keeps the
    /// "scroll needed" condition true forever. Hiding re-runs DTK's mirror logic
    /// (buttons hidden, spacers zeroed); Qt re-shows them on the next layout if
    /// scrolling is genuinely needed, so this is safe to call any time.
    pub fn unlatch_scroll_buttons(&self) {
        unsafe { ffi::tabbar_unlatch_scroll_buttons(self.live_ptr()) }
    }
    /// Reparent as in Qt: the widget becomes hidden and must be shown again.
    pub fn set_parent(&self, parent: &QWidget) {
        unsafe { ffi::widget_set_parent(self.live_ptr(), parent.live_ptr()) }
    }
    /// deferred delete (next event-loop turn)
    pub fn delete_later(&self) {
        unsafe { ffi::object_delete_later(self.live_ptr().cast()) }
    }
    /// key sequence like "Ctrl+Shift+C"; fires while the widget lives.
    /// Returns the callback id for [`unregister_callback`].
    pub fn add_shortcut(&self, key: &str, f: impl FnMut() + 'static) -> usize {
        let ptr = self.live_ptr();
        let id = dtk_sys::register_cb0(f);
        unsafe { ffi::shortcut_new(ptr, key, id) };
        id
    }
}

unsafe impl Signal0 for QWidget {
    fn qobject_ptr(&self) -> *mut ffi::QObject {
        self.live_ptr().cast()
    }
}
unsafe impl SignalI32 for QWidget {
    fn qobject_ptr(&self) -> *mut ffi::QObject {
        self.live_ptr().cast()
    }
}
unsafe impl SignalBool for QWidget {
    fn qobject_ptr(&self) -> *mut ffi::QObject {
        self.live_ptr().cast()
    }
}
unsafe impl QtLifecycle for QWidget {
    fn live_qobject(&self) -> Option<*mut ffi::QObject> {
        self.is_alive().then(|| self.ptr.cast())
    }
    fn was_parented(&self) -> bool {
        self.guard.was_parented()
    }
}

impl std::fmt::Debug for QWidget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QWidget({:p})", self.ptr)
    }
}

// Qt button base handle (returned by e.g. DDialog button accessors)
widget_wrapper!(QAbstractButton, ffi::QAbstractButton);

// fully user-drawn widget: every paint/input event goes to the Rust handler
widget_wrapper!(PaintWidget, ffi::QWidget);

/// key event (key = qt::key::*, mods = qt::modifier::*)
#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub key: i32,
    pub mods: i32,
    pub text: String,
    pub press: bool,
    pub autorepeat: bool,
}

/// mouse event (kind = qt::mouse_kind::*, button = qt::mouse_button::*)
#[derive(Debug, Clone)]
pub struct MouseEvent {
    pub kind: i32,
    pub button: i32,
    pub x: i32,
    pub y: i32,
    pub mods: i32,
}

/// Events delivered to a [`PaintWidget`] handler. A painter is borrowed only
/// for the duration of one paint callback and cannot escape into stored state.
///
/// ```compile_fail
/// use dtk::{PaintWidget, PaintWidgetEvent, Painter};
/// use std::cell::RefCell;
/// thread_local! {
///     static SAVED: RefCell<Option<&'static Painter>> = RefCell::new(None);
/// }
/// let _widget = PaintWidget::new(None, |event| {
///     if let PaintWidgetEvent::Paint(painter, _, _) = event {
///         SAVED.with(|slot| *slot.borrow_mut() = Some(painter));
///     }
/// });
/// ```
pub enum PaintWidgetEvent<'paint> {
    /// Painter valid only inside this callback; w/h = widget size.
    Paint(&'paint Painter, i32, i32),
    Key(KeyEvent),
    Mouse(MouseEvent),
    Wheel {
        dy: i32,
        x: i32,
        y: i32,
        mods: i32,
    },
    /// input method: committed text + in-progress preedit
    Ime {
        commit: String,
        preedit: String,
    },
    Resize {
        w: i32,
        h: i32,
    },
    Focus(bool),
}

impl PaintWidget {
    /// handler receives every event; register nothing else
    pub fn new(
        parent: Option<&QWidget>,
        handler: impl for<'paint> FnMut(PaintWidgetEvent<'paint>) + 'static,
    ) -> Owned<PaintWidget> {
        let parent_ptr = opt_ptr(parent);
        let mut handler = handler;
        let id = dtk_sys::register_cb_pw(move |ev| {
            let ev = match ev {
                dtk_sys::PwEvent::Paint { painter, w, h, .. } => {
                    // QPainter belongs to this stack-scoped C++ paint event;
                    // never put its pointer in an owned, storable Rust event.
                    handler(PaintWidgetEvent::Paint(&Painter { ptr: painter }, w, h));
                    return;
                }
                dtk_sys::PwEvent::Key {
                    key,
                    mods,
                    text,
                    press,
                    autorepeat,
                } => PaintWidgetEvent::Key(KeyEvent {
                    key,
                    mods,
                    text,
                    press,
                    autorepeat,
                }),
                dtk_sys::PwEvent::Mouse {
                    kind,
                    button,
                    x,
                    y,
                    mods,
                } => PaintWidgetEvent::Mouse(MouseEvent {
                    kind,
                    button,
                    x,
                    y,
                    mods,
                }),
                dtk_sys::PwEvent::Wheel { dy, x, y, mods } => {
                    PaintWidgetEvent::Wheel { dy, x, y, mods }
                }
                dtk_sys::PwEvent::Ime { commit, preedit } => {
                    PaintWidgetEvent::Ime { commit, preedit }
                }
                dtk_sys::PwEvent::Resize { w, h } => PaintWidgetEvent::Resize { w, h },
                dtk_sys::PwEvent::Focus(gained) => PaintWidgetEvent::Focus(gained),
            };
            handler(ev);
        });
        Owned::new(Self::from_raw(unsafe {
            ffi::paint_widget_new(id, parent_ptr)
        }))
    }
    /// test/helper: synchronously deliver a key-press (bypasses the OS event source)
    pub fn inject_key(&self, key: i32, mods: i32, text: &str) {
        unsafe { ffi::paint_widget_inject_key(self.live_ptr(), key, mods, text) }
    }
    /// IME candidate window anchor: cursor rect in widget coords (call when it moves)
    pub fn set_ime_cursor_rect(&self, x: i32, y: i32, w: i32, h: i32) {
        unsafe { ffi::paint_widget_set_ime_rect(self.live_ptr(), x, y, w, h) }
    }
}

/// vertical scrollbar (DScrollBar = QScrollBar in DTK6); child of a widget
#[derive(Clone)]
pub struct ScrollBar {
    w: QWidget,
}

impl ScrollBar {
    pub fn new(parent: &QWidget) -> Owned<ScrollBar> {
        require_application();
        Owned::new(Self {
            w: QWidget::from_raw(unsafe { ffi::scrollbar_new(parent.live_ptr()) }),
        })
    }
    pub fn as_widget(&self) -> QWidget {
        self.w.clone()
    }
    pub fn set_range(&self, min: i32, max: i32) {
        unsafe { ffi::scrollbar_set_range(self.w.live_ptr(), min, max) }
    }
    pub fn maximum(&self) -> i32 {
        unsafe { ffi::scrollbar_maximum(self.w.live_ptr()) }
    }
    pub fn set_value(&self, v: i32) {
        unsafe { ffi::scrollbar_set_value(self.w.live_ptr(), v) }
    }
    pub fn value(&self) -> i32 {
        unsafe { ffi::scrollbar_value(self.w.live_ptr()) }
    }
    pub fn set_page_step(&self, v: i32) {
        unsafe { ffi::scrollbar_set_page_step(self.w.live_ptr(), v) }
    }
    /// valueChanged(int); 0 on failure
    pub fn on_value_changed(&self, f: impl FnMut(i32) + 'static) -> usize {
        self.w.connect_signal_i32("valueChanged(int)", f)
    }
}
unsafe impl QtLifecycle for ScrollBar {
    fn live_qobject(&self) -> Option<*mut ffi::QObject> {
        self.w.live_qobject()
    }
    fn was_parented(&self) -> bool {
        self.w.was_parented()
    }
}

/// system clipboard (QGuiApplication::clipboard)
pub struct Clipboard;

impl Clipboard {
    pub fn set_text(text: &str) {
        require_application();
        unsafe { ffi::clipboard_set_text(text, 0) }
    }
    pub fn text() -> String {
        require_application();
        unsafe { ffi::clipboard_text(0) }
    }
    /// X11 primary selection (copy-on-select)
    pub fn set_selection(text: &str) {
        require_application();
        unsafe { ffi::clipboard_set_text(text, 1) }
    }
    pub fn selection() -> String {
        require_application();
        unsafe { ffi::clipboard_text(1) }
    }
}

// QAction is a QObject, not a QWidget; its Qt parent controls lifetime.
object_wrapper!(QAction, ffi::QAction);

impl QAction {
    pub fn new(text: &str, parent: &impl Signal0) -> Owned<QAction> {
        require_application();
        Owned::new(Self::from_raw(unsafe {
            ffi::action_new(text, parent.qobject_ptr())
        }))
    }
    pub fn set_enabled(&self, enabled: bool) {
        unsafe { ffi::action_set_enabled(self.live_ptr(), enabled) }
    }
    pub fn is_enabled(&self) -> bool {
        unsafe { ffi::action_is_enabled(self.live_ptr()) }
    }
    pub fn set_visible(&self, visible: bool) {
        unsafe { ffi::action_set_visible(self.live_ptr(), visible) }
    }
    pub fn is_visible(&self) -> bool {
        unsafe { ffi::action_is_visible(self.live_ptr()) }
    }
    pub fn set_text(&self, text: &str) {
        unsafe { ffi::action_set_text(self.live_ptr(), text) }
    }
    pub fn text(&self) -> String {
        unsafe { ffi::action_text(self.live_ptr()) }
    }
    pub fn trigger(&self) {
        unsafe { ffi::action_trigger(self.live_ptr()) }
    }
    /// Connect `triggered(bool)`, including the checked state for checkable actions.
    pub fn on_triggered(&self, f: impl FnMut(bool) + 'static) -> usize {
        self.connect_signal_bool("triggered(bool)", f)
    }
}

// Popup menu (DMenu = QMenu typedef in DTK6); parent owns its actions.
widget_wrapper!(DMenu, ffi::QMenu);

impl DMenu {
    pub fn new(parent: &QWidget) -> Owned<DMenu> {
        require_application();
        Owned::new(Self::from_raw(unsafe { ffi::menu_new(parent.live_ptr()) }))
    }
    /// Append an action; its callback is released when the action/menu is destroyed.
    /// Returns an id for [`unregister_callback`] if early cancellation is needed.
    pub fn add_action(&self, text: &str, f: impl FnMut() + 'static) -> usize {
        let ptr = self.live_ptr();
        let id = dtk_sys::register_cb0(f);
        unsafe { ffi::menu_add_action_cb(ptr, text, id) };
        id
    }
    pub fn add_separator(&self) {
        unsafe { ffi::menu_add_separator(self.live_ptr()) }
    }
    /// Add an existing QAction without changing its QObject parent/ownership.
    /// Parent the action to this menu if it should live and die with the menu.
    pub fn add_action_object(&self, action: &QAction) {
        unsafe { ffi::menu_add_existing_action(self.live_ptr(), action.live_ptr()) }
    }
    /// Popup at (x, y) in ref widget coords; like QMenu, closing only hides it.
    pub fn popup(&self, ref_: &QWidget, x: i32, y: i32) {
        unsafe { ffi::menu_popup(self.live_ptr(), ref_.live_ptr(), x, y) }
    }
}

/// Connect an arg-less signal on a QObject (e.g. clicked or timeout).
/// # Safety
/// Implementors must return a live, correctly adjusted QObject pointer on the
/// calling Qt thread, or reject the call before returning a dangling pointer.
pub unsafe trait Signal0 {
    fn qobject_ptr(&self) -> *mut ffi::QObject;
    /// Full Qt signature, e.g. "clicked(bool)" / "timeout()"; signal args are ignored.
    /// Returns the callback id for [`unregister_callback`]; 0 = connect failed.
    fn connect_signal(&self, signal: &str, f: impl FnMut() + 'static) -> usize {
        let sender = self.qobject_ptr();
        let id = dtk_sys::register_cb0(f);
        if unsafe { ffi::relay_connect0(sender, signal, id) } {
            unsafe { dtk_sys::register_qt_relay(id) };
            id
        } else {
            dtk_sys::unregister_cb(id); // roll back registration
            0
        }
    }
}

/// Signal with one i32 arg (e.g. currentRowChanged(int)).
/// # Safety
/// `qobject_ptr()` must return a live, correctly adjusted QObject pointer on
/// the Qt owner's thread, or reject the call before returning a dangling one.
pub unsafe trait SignalI32 {
    fn qobject_ptr(&self) -> *mut ffi::QObject;
    /// Returns the callback id for [`unregister_callback`]; 0 = connect failed.
    fn connect_signal_i32(&self, signal: &str, f: impl FnMut(i32) + 'static) -> usize {
        let sender = self.qobject_ptr();
        let id = dtk_sys::register_cb_i32(f);
        if unsafe { ffi::relay_connect_i32(sender, signal, id) } {
            unsafe { dtk_sys::register_qt_relay(id) };
            id
        } else {
            dtk_sys::unregister_cb(id);
            0
        }
    }
}

/// Signal with one bool arg (e.g. checkedChanged(bool)).
/// # Safety
/// `qobject_ptr()` must return a live, correctly adjusted QObject pointer on
/// the Qt owner's thread, or reject the call before returning a dangling one.
pub unsafe trait SignalBool {
    fn qobject_ptr(&self) -> *mut ffi::QObject;
    /// Returns the callback id for [`unregister_callback`]; 0 = connect failed.
    fn connect_signal_bool(&self, signal: &str, f: impl FnMut(bool) + 'static) -> usize {
        let sender = self.qobject_ptr();
        let id = dtk_sys::register_cb_bool(f);
        if unsafe { ffi::relay_connect_bool(sender, signal, id) } {
            unsafe { dtk_sys::register_qt_relay(id) };
            id
        } else {
            dtk_sys::unregister_cb(id);
            0
        }
    }
}

/// Signal with two i32 args (e.g. tabMoved(int,int)).
/// # Safety
/// `qobject_ptr()` must return a live, correctly adjusted QObject pointer on
/// the Qt owner's thread, or reject the call before returning a dangling one.
pub unsafe trait SignalI32I32 {
    fn qobject_ptr(&self) -> *mut ffi::QObject;
    /// Returns the callback id for [`unregister_callback`]; 0 = connect failed.
    fn connect_signal_i32_i32(&self, signal: &str, f: impl FnMut(i32, i32) + 'static) -> usize {
        let sender = self.qobject_ptr();
        let id = dtk_sys::register_cb_i32_i32(f);
        if unsafe { ffi::relay_connect_i32_i32(sender, signal, id) } {
            unsafe { dtk_sys::register_qt_relay(id) };
            id
        } else {
            dtk_sys::unregister_cb(id);
            0
        }
    }
}

/// disconnect + unregister a connected signal callback (id from connect_signal*)
pub use dtk_sys::unregister_cb as unregister_callback;

// Application scope implementation lives in application.rs.

/// Preserve Linux argv bytes (including non-UTF-8 and control characters).
/// Qt needs writable, NUL-terminated `char*` entries that outlive QApplication.
fn env_args_raw() -> (Vec<u8>, Vec<usize>) {
    use std::os::unix::ffi::OsStrExt;
    let mut bytes = Vec::new();
    let mut lengths = Vec::new();
    for arg in std::env::args_os() {
        let arg = arg.as_bytes();
        lengths.push(arg.len());
        bytes.extend_from_slice(arg);
    }
    (bytes, lengths)
}

// ---- DMainWindow / DTitlebar ----

widget_wrapper!(DMainWindow, ffi::DMainWindow);
widget_wrapper!(DTitlebar, ffi::DTitlebar);

impl DMainWindow {
    pub fn new() -> Owned<DMainWindow> {
        require_application();
        Owned::new(Self::from_raw(unsafe { ffi::mainwindow_new() }))
    }
    /// with event callbacks: on_close returning false keeps the window open (event ignore)
    pub fn new_with_events(
        on_show: impl FnMut() + 'static,
        on_close: impl FnMut() -> bool + 'static,
    ) -> Owned<DMainWindow> {
        require_application();
        let show_id = dtk_sys::register_cb0(on_show);
        let close_id = dtk_sys::register_cb_guard(on_close);
        Owned::new(Self::from_raw(unsafe {
            ffi::mainwindow_new_ex(show_id, close_id)
        }))
    }
    pub fn titlebar(&self) -> DTitlebar {
        DTitlebar::from_raw(unsafe { ffi::mainwindow_titlebar(self.live_ptr()) })
    }
    pub fn set_central_widget(&self, w: &QWidget) {
        unsafe { ffi::mainwindow_set_central_widget(self.live_ptr(), w.live_ptr()) }
    }
    pub fn set_window_radius(&self, radius: i32) {
        unsafe { ffi::mainwindow_set_window_radius(self.live_ptr(), radius) }
    }
    pub fn set_enable_blur_window(&self, enable: bool) {
        unsafe { ffi::mainwindow_set_enable_blur(self.live_ptr(), enable) }
    }
    /// Take ownership from the window; Qt returns None if no central widget is
    /// set. The widget is unparented again, so it either needs a new home
    /// (`set_central_widget`, `layout.add_widget`, ...) or it gets deleted
    /// once the returned handle drops without ever being reparented.
    pub fn take_central_widget(&self) -> Option<Owned<QWidget>> {
        let ptr = unsafe { ffi::mainwindow_take_central_widget(self.live_ptr()) };
        (!ptr.is_null()).then(|| Owned::new(QWidget::from_raw(ptr)))
    }
}

impl DTitlebar {
    pub fn set_title(&self, title: &str) {
        unsafe { ffi::titlebar_set_title(self.live_ptr(), title) }
    }
    pub fn set_icon(&self, icon: &QIcon) {
        unsafe { ffi::titlebar_set_icon(self.live_ptr(), &*icon.ptr) }
    }
}

// ---- value types ----

/// Value-type wrapper: heap-allocated copy, always released in Drop.
macro_rules! value_wrapper {
    ($name:ident, $ffi:ty, $del:ident) => {
        pub struct $name {
            pub(crate) ptr: *mut $ffi,
            _not_send: PhantomData<*mut ()>,
        }
        impl $name {
            pub(crate) fn from_raw(ptr: *mut $ffi) -> Self {
                assert!(!ptr.is_null());
                Self {
                    ptr,
                    _not_send: PhantomData,
                }
            }
        }
        impl Drop for $name {
            fn drop(&mut self) {
                unsafe { ffi::$del(self.ptr) }
            }
        }
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, concat!(stringify!($name), "({:p})"), self.ptr)
            }
        }
    };
}

value_wrapper!(QColor, ffi::QColor, color_delete);
value_wrapper!(QFont, ffi::QFont, font_delete);
value_wrapper!(QPalette, ffi::QPalette, palette_delete);
value_wrapper!(DPalette, ffi::DPalette, dpalette_delete);
value_wrapper!(QPixmap, ffi::QPixmap, pixmap_delete);
value_wrapper!(QImage, ffi::QImage, image_delete);
value_wrapper!(QBrush, ffi::QBrush, brush_delete);
value_wrapper!(QPainterPath, ffi::QPainterPath, painter_path_delete);
value_wrapper!(QKeySequence, ffi::QKeySequence, key_sequence_delete);
value_wrapper!(QUrl, ffi::QUrl, url_delete);
value_wrapper!(QPoint, ffi::QPoint, point_delete);
value_wrapper!(QRect, ffi::QRect, rect_delete);
value_wrapper!(QSize, ffi::QSize, size_delete);
value_wrapper!(QMargins, ffi::QMargins, margins_delete);
value_wrapper!(QVariant, ffi::QVariant, variant_delete);
value_wrapper!(QVariantList, ffi::QVariantList, variant_list_delete);
value_wrapper!(DDciIcon, ffi::DDciIcon, ddci_icon_delete);

impl QVariant {
    /// An invalid/empty Qt QVariant.
    pub fn new() -> Self {
        Self::from_raw(unsafe { ffi::variant_new() })
    }
    pub fn from_bool(value: bool) -> Self {
        Self::from_raw(unsafe { ffi::variant_from_bool(value) })
    }
    pub fn from_i64(value: i64) -> Self {
        Self::from_raw(unsafe { ffi::variant_from_i64(value) })
    }
    pub fn from_f64(value: f64) -> Self {
        Self::from_raw(unsafe { ffi::variant_from_f64(value) })
    }
    pub fn from_string(value: &str) -> Self {
        Self::from_raw(unsafe { ffi::variant_from_string(value) })
    }
    pub fn is_valid(&self) -> bool {
        unsafe { ffi::variant_is_valid(self.ptr) }
    }
    pub fn to_bool(&self) -> bool {
        unsafe { ffi::variant_to_bool(self.ptr) }
    }
    /// Return None if Qt cannot convert this value to a 64-bit integer.
    pub fn to_i64(&self) -> Option<i64> {
        let mut ok = false;
        let value = unsafe { ffi::variant_to_i64(self.ptr, &mut ok) };
        ok.then_some(value)
    }
}

impl std::fmt::Display for QVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&unsafe { ffi::variant_to_string(self.ptr) })
    }
}

impl Clone for QVariant {
    fn clone(&self) -> Self {
        Self::from_raw(unsafe { ffi::variant_clone(self.ptr) })
    }
}

impl Default for QVariant {
    fn default() -> Self {
        Self::new()
    }
}

impl QVariantList {
    pub fn new() -> Self {
        Self::from_raw(unsafe { ffi::variant_list_new() })
    }
    /// Qt copies the implicitly shared QVariant into this list.
    pub fn push(&self, value: &QVariant) {
        unsafe { ffi::variant_list_push(self.ptr, value.ptr) }
    }
    pub fn len(&self) -> usize {
        unsafe { ffi::variant_list_len(self.ptr) }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Return an owned QVariant copy, or None for an out-of-range index.
    pub fn get(&self, index: usize) -> Option<QVariant> {
        let ptr = unsafe { ffi::variant_list_get(self.ptr, index) };
        (!ptr.is_null()).then(|| QVariant::from_raw(ptr))
    }
}

impl Clone for QVariantList {
    fn clone(&self) -> Self {
        Self::from_raw(unsafe { ffi::variant_list_clone(self.ptr) })
    }
}

impl Default for QVariantList {
    fn default() -> Self {
        Self::new()
    }
}

impl FromIterator<QVariant> for QVariantList {
    fn from_iter<T: IntoIterator<Item = QVariant>>(iter: T) -> Self {
        let list = Self::new();
        for value in iter {
            list.push(&value);
        }
        list
    }
}

impl QColor {
    pub fn rgb(r: i32, g: i32, b: i32) -> Self {
        Self::from_raw(unsafe { ffi::color_new_rgb(r, g, b, 255) })
    }
    pub fn rgba(r: i32, g: i32, b: i32, a: i32) -> Self {
        Self::from_raw(unsafe { ffi::color_new_rgb(r, g, b, a) })
    }
    /// packed 0xAARRGGBB
    pub fn rgba_u32(&self) -> u32 {
        unsafe { ffi::color_rgba(self.ptr) as u32 }
    }
}

impl QFont {
    pub fn new() -> Self {
        Self::from_raw(unsafe { ffi::font_new() })
    }
    pub fn set_point_size(&self, size: i32) {
        unsafe { ffi::font_set_point_size(self.ptr, size) }
    }
    pub fn set_bold(&self, bold: bool) {
        unsafe { ffi::font_set_bold(self.ptr, bold) }
    }
    /// italic on/off
    pub fn set_italic(&self, italic: bool) {
        unsafe { ffi::font_set_italic(self.ptr, italic) }
    }
    /// generic monospace family (terminals, code)
    pub fn set_monospace(&self) {
        unsafe { ffi::font_set_monospace(self.ptr) }
    }
    /// set the font family by name (e.g. "Fira Code")
    pub fn set_family(&self, name: &str) {
        unsafe { ffi::font_set_family(self.ptr, name) }
    }
    /// QFont::ForceIntegerMetrics: terminals need integer per-cell advances,
    /// otherwise shaped runs drift off the cell grid (fractional 9.85 vs cell 10)
    pub fn force_integer_metrics(&self) {
        unsafe { ffi::font_force_integer_metrics(self.ptr) }
    }
    /// shaped advance of a text run (kerning/ligatures included)
    pub fn advance(&self, text: &str) -> i32 {
        unsafe { ffi::fontmetrics_advance(self.ptr, text) }
    }
    /// cell geometry for grid rendering: (max char width, line height, ascent)
    pub fn metrics(&self) -> (i32, i32, i32) {
        unsafe {
            (
                ffi::fontmetrics_max_width(self.ptr),
                ffi::fontmetrics_height(self.ptr),
                ffi::fontmetrics_ascent(self.ptr),
            )
        }
    }
}

impl DPalette {
    pub fn new() -> Self {
        Self::from_raw(unsafe { ffi::dpalette_new() })
    }
    /// Copy the DTK title brush rather than exposing its internal const&.
    pub fn text_title(&self) -> QBrush {
        QBrush::from_raw(unsafe { ffi::dpalette_text_title(self.ptr) })
    }
}

impl QPalette {
    pub fn new() -> Self {
        Self::from_raw(unsafe { ffi::palette_new() })
    }
    /// use qt::palette_group / qt::palette_role constants
    pub fn set_color(&self, group: i32, role: i32, color: &QColor) {
        unsafe { ffi::palette_set_color(self.ptr, group, role, color.ptr) }
    }
    /// read a color (e.g. copy Active Highlight into the Inactive group)
    pub fn color(&self, group: i32, role: i32) -> QColor {
        QColor::from_raw(unsafe { ffi::palette_color(self.ptr, group, role) })
    }
}

impl QPixmap {
    /// file or qrc path
    pub fn new(path: &str) -> Self {
        require_application();
        Self::from_raw(unsafe { ffi::pixmap_new(path) })
    }
}

impl QImage {
    /// ARGB32 pixel buffer. Pixels start uninitialised, as in Qt's QImage;
    /// call `fill` before reading them.
    pub fn new(width: i32, height: i32) -> Self {
        Self::from_raw(unsafe { ffi::image_new(width, height) })
    }
    /// Load an image file. A failed load yields a null QImage, as in Qt.
    pub fn from_file(path: &str) -> Self {
        Self::from_raw(unsafe { ffi::image_from_file(path) })
    }
    pub fn is_null(&self) -> bool {
        unsafe { ffi::image_is_null(self.ptr) }
    }
    pub fn width(&self) -> i32 {
        unsafe { ffi::image_width(self.ptr) }
    }
    pub fn height(&self) -> i32 {
        unsafe { ffi::image_height(self.ptr) }
    }
    pub fn fill(&mut self, color: &QColor) {
        unsafe { ffi::image_fill(self.ptr, color.ptr) }
    }
    pub fn set_text(&mut self, key: &str, value: &str) {
        unsafe { ffi::image_set_text(self.ptr, key, value) }
    }
    pub fn text(&self, key: &str) -> String {
        unsafe { ffi::image_text(self.ptr, key) }
    }
    /// Save using Qt's encoder inferred from the file extension.
    pub fn save(&self, path: &str) -> bool {
        unsafe { ffi::image_save(self.ptr, path) }
    }
    /// Pixel color; x/y must be in bounds (Qt warns on invalid coordinates).
    pub fn pixel_color(&self, x: i32, y: i32) -> QColor {
        assert!(x >= 0 && x < self.width() && y >= 0 && y < self.height());
        QColor::from_raw(unsafe { ffi::image_pixel_color(self.ptr, x, y) })
    }
}

impl QBrush {
    pub fn from_color(color: &QColor) -> Self {
        Self::from_raw(unsafe { ffi::brush_from_color(color.ptr) })
    }
    pub fn color(&self) -> QColor {
        QColor::from_raw(unsafe { ffi::brush_color(self.ptr) })
    }
}

impl QPainterPath {
    pub fn new() -> Self {
        Self::from_raw(unsafe { ffi::painter_path_new() })
    }
    pub fn add_rect(&mut self, x: f64, y: f64, w: f64, h: f64) {
        unsafe { ffi::painter_path_add_rect(self.ptr, x, y, w, h) }
    }
    pub fn is_empty(&self) -> bool {
        unsafe { ffi::painter_path_is_empty(self.ptr) }
    }
    pub fn element_count(&self) -> i32 {
        unsafe { ffi::painter_path_element_count(self.ptr) }
    }
}

impl QKeySequence {
    /// Parse a Qt portable shortcut such as `Ctrl+Shift+K`.
    pub fn from_portable_text(text: &str) -> Self {
        Self::from_raw(unsafe { ffi::key_sequence_from_string(text) })
    }
}

impl std::fmt::Display for QKeySequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&unsafe { ffi::key_sequence_to_string(self.ptr) })
    }
}

impl QUrl {
    pub fn from_string(url: &str) -> Self {
        Self::from_raw(unsafe { ffi::url_from_string(url) })
    }
    pub fn is_valid(&self) -> bool {
        unsafe { ffi::url_is_valid(self.ptr) }
    }
}

impl std::fmt::Display for QUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&unsafe { ffi::url_to_string(self.ptr) })
    }
}

impl QPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self::from_raw(unsafe { ffi::point_new(x, y) })
    }
}

impl QRect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self::from_raw(unsafe { ffi::rect_new(x, y, w, h) })
    }
    pub fn width(&self) -> i32 {
        unsafe { ffi::rect_width(self.ptr) }
    }
    pub fn height(&self) -> i32 {
        unsafe { ffi::rect_height(self.ptr) }
    }
    pub fn x(&self) -> i32 {
        unsafe { ffi::rect_x(self.ptr) }
    }
    pub fn y(&self) -> i32 {
        unsafe { ffi::rect_y(self.ptr) }
    }
}

impl QSize {
    pub fn new(w: i32, h: i32) -> Self {
        Self::from_raw(unsafe { ffi::size_new(w, h) })
    }
}

impl QMargins {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self::from_raw(unsafe { ffi::q_margins_new(left, top, right, bottom) })
    }
}

impl DDciIcon {
    pub fn new() -> Self {
        Self::from_raw(unsafe { ffi::ddci_icon_new() })
    }
    /// load from a .dci file path
    pub fn from_file(path: &str) -> Self {
        Self::from_raw(unsafe { ffi::ddci_icon_from_file(path) })
    }
}

/// Qt enum/QFlags constants (for params mapped to i32), grouped by enum
pub mod qt {
    /// Qt::Alignment
    pub mod alignment {
        pub const LEFT: i32 = 0x1;
        pub const RIGHT: i32 = 0x2;
        pub const HCENTER: i32 = 0x4;
        pub const TOP: i32 = 0x20;
        pub const BOTTOM: i32 = 0x40;
        pub const VCENTER: i32 = 0x80;
        pub const CENTER: i32 = 0x84;
    }
    /// Qt::FocusPolicy
    pub mod focus {
        pub const NO_FOCUS: i32 = 0;
    }
    /// Qt::ItemDataRole
    pub mod item_role {
        pub const DISPLAY_ROLE: i32 = 0;
        pub const EDIT_ROLE: i32 = 2;
        pub const USER_ROLE: i32 = 0x0100;
    }
    /// Qt::KeyboardModifier
    pub mod modifier {
        pub const SHIFT: i32 = 0x02000000;
        pub const CONTROL: i32 = 0x04000000;
        pub const ALT: i32 = 0x08000000;
        pub const META: i32 = 0x10000000;
        pub const KEYPAD: i32 = 0x20000000;
    }
    /// Qt::MouseButton
    pub mod mouse_button {
        pub const LEFT: i32 = 1;
        pub const RIGHT: i32 = 2;
        pub const MIDDLE: i32 = 4;
        pub const BACK: i32 = 8;
        pub const FORWARD: i32 = 16;
    }
    /// PaintWidget mouse event kinds
    pub mod mouse_kind {
        pub const PRESS: i32 = 0;
        pub const RELEASE: i32 = 1;
        pub const MOVE: i32 = 2;
        pub const DOUBLE_CLICK: i32 = 3;
    }
    /// Qt::CursorShape subset
    pub mod cursor {
        pub const ARROW: i32 = 0;
        pub const IBEAM: i32 = 4;
        pub const SIZE_VER: i32 = 5; // vertical bar (drag top/bottom)
        pub const SIZE_HOR: i32 = 6; // horizontal bar (drag left/right)
        pub const POINTING_HAND: i32 = 13;
    }
    /// Qt::Key (letters/digits are ASCII); common subset for terminal input
    pub mod key {
        pub const ESCAPE: i32 = 0x01000000;
        pub const TAB: i32 = 0x01000001;
        pub const BACKTAB: i32 = 0x01000002;
        pub const BACKSPACE: i32 = 0x01000003;
        pub const RETURN: i32 = 0x01000004;
        pub const ENTER: i32 = 0x01000005; // keypad
        pub const INSERT: i32 = 0x01000006;
        pub const DELETE: i32 = 0x01000007;
        pub const PAUSE: i32 = 0x01000008;
        pub const PRINT: i32 = 0x01000009;
        pub const CLEAR: i32 = 0x0100000b;
        pub const HOME: i32 = 0x01000010;
        pub const END: i32 = 0x01000011;
        pub const LEFT: i32 = 0x01000012;
        pub const UP: i32 = 0x01000013;
        pub const RIGHT: i32 = 0x01000014;
        pub const DOWN: i32 = 0x01000015;
        pub const PAGE_UP: i32 = 0x01000016;
        pub const PAGE_DOWN: i32 = 0x01000017;
        pub const SHIFT: i32 = 0x01000020;
        pub const CONTROL: i32 = 0x01000021;
        pub const META: i32 = 0x01000022;
        pub const ALT: i32 = 0x01000023;
        pub const CAPS_LOCK: i32 = 0x01000024;
        pub const NUM_LOCK: i32 = 0x01000025;
        pub const SCROLL_LOCK: i32 = 0x01000026;
        pub const F1: i32 = 0x01000030;
        pub const F2: i32 = 0x01000031;
        pub const F3: i32 = 0x01000032;
        pub const F4: i32 = 0x01000033;
        pub const F5: i32 = 0x01000034;
        pub const F6: i32 = 0x01000035;
        pub const F7: i32 = 0x01000036;
        pub const F8: i32 = 0x01000037;
        pub const F9: i32 = 0x01000038;
        pub const F10: i32 = 0x01000039;
        pub const F11: i32 = 0x0100003a;
        pub const F12: i32 = 0x0100003b;
    }
    /// Qt::Orientation
    pub mod orientation {
        pub const HORIZONTAL: i32 = 1;
        pub const VERTICAL: i32 = 2;
    }
    /// QFrame::Shape
    pub mod frame {
        pub const STYLED_PANEL: i32 = 0x6;
    }
    /// QStyle::StandardPixmap
    pub mod standard_pixmap {
        pub const MESSAGE_BOX_WARNING: i32 = 10;
    }
    /// QHeaderView::ResizeMode
    pub mod header_resize {
        pub const INTERACTIVE: i32 = 0;
        pub const STRETCH: i32 = 1;
        pub const RESIZE_TO_CONTENTS: i32 = 3;
    }
    /// QPalette::ColorGroup
    pub mod palette_group {
        pub const ACTIVE: i32 = 0;
        pub const DISABLED: i32 = 1;
        pub const INACTIVE: i32 = 2;
        pub const CURRENT: i32 = 3;
        pub const ALL: i32 = 5;
    }
    /// QPalette::ColorRole
    pub mod palette_role {
        pub const WINDOW_TEXT: i32 = 0;
        pub const TEXT: i32 = 6;
        pub const BASE: i32 = 9;
        pub const WINDOW: i32 = 10;
        pub const HIGHLIGHT: i32 = 12;
        pub const HIGHLIGHTED_TEXT: i32 = 13;
    }
    /// Qt::TextElideMode
    pub mod elide {
        pub const LEFT: i32 = 0;
        pub const RIGHT: i32 = 1;
        pub const MIDDLE: i32 = 2;
        pub const NONE: i32 = 3;
    }
    /// QStyleOption::State
    pub mod state {
        pub const SELECTED: i32 = 0x8000;
        pub const MOUSE_OVER: i32 = 0x2000;
    }
    /// QMessageBox::Icon
    pub mod msg_icon {
        pub const NO_ICON: i32 = 0;
        pub const INFORMATION: i32 = 1;
        pub const WARNING: i32 = 2;
        pub const CRITICAL: i32 = 3;
        pub const QUESTION: i32 = 4;
    }
    /// QMessageBox::ButtonRole
    pub mod msg_role {
        pub const INVALID: i32 = -1;
        pub const ACCEPT: i32 = 0;
        pub const REJECT: i32 = 1;
        pub const DESTRUCTIVE: i32 = 2;
        pub const ACTION: i32 = 3;
        pub const HELP: i32 = 4;
        pub const YES: i32 = 5;
        pub const NO: i32 = 6;
        pub const RESET: i32 = 7;
        pub const APPLY: i32 = 8;
    }
    /// QMessageBox::StandardButton (QFlags-compatible bitmask)
    pub mod msg_btn {
        pub const NO_BUTTON: i32 = 0x00000000;
        pub const OK: i32 = 0x00000400;
        pub const SAVE: i32 = 0x00000800;
        pub const SAVE_ALL: i32 = 0x00001000;
        pub const OPEN: i32 = 0x00002000;
        pub const YES: i32 = 0x00004000;
        pub const YES_TO_ALL: i32 = 0x00008000;
        pub const NO: i32 = 0x00010000;
        pub const NO_TO_ALL: i32 = 0x00020000;
        pub const ABORT: i32 = 0x00040000;
        pub const RETRY: i32 = 0x00080000;
        pub const IGNORE: i32 = 0x00100000;
        pub const CLOSE: i32 = 0x00200000;
        pub const CANCEL: i32 = 0x00400000;
        pub const DISCARD: i32 = 0x00800000;
        pub const HELP: i32 = 0x01000000;
        pub const APPLY: i32 = 0x02000000;
        pub const RESET: i32 = 0x04000000;
        pub const RESTORE_DEFAULTS: i32 = 0x08000000;
        /// convenience: standard Yes|No button set
        pub const YES_NO: i32 = 0x00014000;
        /// convenience: standard Ok|Cancel button set
        pub const OK_CANCEL: i32 = 0x00400400;
    }
}

// ---- QIcon ----

value_wrapper!(QIcon, ffi::QIcon, icon_delete);

impl QIcon {
    pub fn from_theme(name: &str) -> Self {
        require_application();
        Self::from_raw(unsafe { ffi::icon_from_theme(name) })
    }
    /// like from_theme, but falls back to `fallback` when `name` is not in the icon theme.
    pub fn from_theme_with_fallback(name: &str, fallback: &QIcon) -> Self {
        require_application();
        Self::from_raw(unsafe { ffi::icon_from_theme_fallback(name, fallback.ptr) })
    }
    pub fn from_file(path: &str) -> Self {
        require_application();
        Self::from_raw(unsafe { ffi::icon_from_file(path) })
    }
}

// ---- DLabel ----

widget_wrapper!(DLabel, ffi::DLabel);

impl DLabel {
    pub fn new(text: &str) -> Owned<DLabel> {
        require_application();
        Owned::new(Self::from_raw(unsafe { ffi::label_new(text) }))
    }
    pub fn set_text(&self, text: &str) {
        unsafe { ffi::label_set_text(self.live_ptr(), text) }
    }
    pub fn set_word_wrap(&self, wrap: bool) {
        unsafe { ffi::label_set_word_wrap(self.live_ptr(), wrap) }
    }
    /// use qt::alignment::* constants for alignment
    pub fn set_alignment(&self, alignment: i32) {
        unsafe { ffi::label_set_alignment(self.live_ptr(), alignment) }
    }
    pub fn set_pixmap(&self, pm: &QPixmap) {
        unsafe { ffi::label_set_pixmap(self.live_ptr(), pm.ptr) }
    }
}

// ---- buttons ----

widget_wrapper!(DSuggestButton, ffi::DSuggestButton);
widget_wrapper!(DPushButton, ffi::DPushButton);

impl DSuggestButton {
    pub fn new(text: &str) -> Owned<DSuggestButton> {
        require_application();
        Owned::new(Self::from_raw(unsafe { ffi::suggest_button_new(text) }))
    }
    pub fn set_text(&self, text: &str) {
        unsafe { ffi::button_set_text(self.live_ptr().cast(), text) }
    }
    pub fn on_clicked(&self, f: impl FnMut() + 'static) -> usize {
        self.connect_signal("clicked(bool)", f)
    }
    /// programmatic click (emits clicked; useful for tests)
    pub fn click(&self) {
        unsafe { ffi::button_click(self.live_ptr().cast()) }
    }
}

impl DPushButton {
    pub fn new(text: &str) -> Owned<DPushButton> {
        require_application();
        Owned::new(Self::from_raw(unsafe { ffi::push_button_new(text) }))
    }
    pub fn set_text(&self, text: &str) {
        unsafe { ffi::button_set_text(self.live_ptr(), text) }
    }
    pub fn click(&self) {
        unsafe { ffi::button_click(self.live_ptr()) }
    }
    pub fn on_clicked(&self, f: impl FnMut() + 'static) -> usize {
        self.connect_signal("clicked(bool)", f)
    }
}

// ---- DMessageBox (DMessageBox = typedef QMessageBox) ----

widget_wrapper!(DMessageBox, ffi::QMessageBox);

impl DMessageBox {
    /// create an empty dialog; set title/text/icon/buttons then exec()
    pub fn new() -> Owned<DMessageBox> {
        require_application();
        Owned::new(Self::from_raw(unsafe { ffi::qmessagebox_new() }))
    }
    /// create a pre-configured dialog; use qt::msg_icon::* and qt::msg_btn::* constants
    pub fn with(
        icon: i32,
        title: &str,
        text: &str,
        buttons: i32,
        parent: Option<&QWidget>,
    ) -> Owned<DMessageBox> {
        let p = opt_ptr(parent);
        Owned::new(Self::from_raw(unsafe {
            ffi::qmessagebox_new_with(icon, title, text, buttons, p)
        }))
    }
    pub fn set_text(&self, text: &str) {
        unsafe { ffi::qmessagebox_set_text(self.live_ptr(), text) }
    }
    pub fn set_icon(&self, icon: i32) {
        unsafe { ffi::qmessagebox_set_icon(self.live_ptr(), icon) }
    }
    pub fn set_informative_text(&self, text: &str) {
        unsafe { ffi::qmessagebox_set_informative_text(self.live_ptr(), text) }
    }
    pub fn set_detailed_text(&self, text: &str) {
        unsafe { ffi::qmessagebox_set_detailed_text(self.live_ptr(), text) }
    }
    pub fn set_standard_buttons(&self, buttons: i32) {
        unsafe { ffi::qmessagebox_set_standard_buttons(self.live_ptr(), buttons) }
    }
    /// add a custom button with a text + ButtonRole; returns the button handle
    pub fn add_button(&self, text: &str, role: i32) -> DPushButton {
        DPushButton::from_raw(unsafe {
            ffi::qmessagebox_add_button_text(self.live_ptr(), text, role)
        })
    }
    /// add a standard button (qt::msg_btn::*); returns the button handle
    pub fn add_standard_button(&self, button: i32) -> DPushButton {
        DPushButton::from_raw(unsafe {
            ffi::qmessagebox_add_button_standard(self.live_ptr(), button)
        })
    }
    pub fn set_default_button(&self, button: i32) {
        unsafe { ffi::qmessagebox_set_default_button(self.live_ptr(), button) }
    }
    /// exec the dialog (blocking); returns the clicked StandardButton (qt::msg_btn::*)
    pub fn exec(&self) -> i32 {
        unsafe { ffi::qmessagebox_exec(self.live_ptr()) }
    }
    /// which standard button was clicked after exec returns
    pub fn clicked_button(&self) -> i32 {
        unsafe { ffi::qmessagebox_clicked_button(self.live_ptr()) }
    }
    pub fn text(&self) -> String {
        unsafe { ffi::qmessagebox_text(self.live_ptr()) }
    }

    // ---- static helpers ----
    /// information dialog; returns clicked StandardButton
    pub fn information(
        parent: Option<&QWidget>,
        title: &str,
        text: &str,
        buttons: i32,
        default_button: i32,
    ) -> i32 {
        let p = opt_ptr(parent);
        unsafe { ffi::qmessagebox_information(p, title, text, buttons, default_button) }
    }
    pub fn warning(
        parent: Option<&QWidget>,
        title: &str,
        text: &str,
        buttons: i32,
        default_button: i32,
    ) -> i32 {
        let p = opt_ptr(parent);
        unsafe { ffi::qmessagebox_warning(p, title, text, buttons, default_button) }
    }
    pub fn critical(
        parent: Option<&QWidget>,
        title: &str,
        text: &str,
        buttons: i32,
        default_button: i32,
    ) -> i32 {
        let p = opt_ptr(parent);
        unsafe { ffi::qmessagebox_critical(p, title, text, buttons, default_button) }
    }
    pub fn question(
        parent: Option<&QWidget>,
        title: &str,
        text: &str,
        buttons: i32,
        default_button: i32,
    ) -> i32 {
        let p = opt_ptr(parent);
        unsafe { ffi::qmessagebox_question(p, title, text, buttons, default_button) }
    }
    pub fn about(parent: Option<&QWidget>, title: &str, text: &str) {
        let p = opt_ptr(parent);
        unsafe { ffi::qmessagebox_about(p, title, text) }
    }
}

// ---- layouts ----

object_wrapper!(QLayout, ffi::QLayout);

impl QLayout {
    fn from_guarded(ptr: *mut ffi::QLayout, guard: Rc<dtk_sys::ObjectGuard>) -> Self {
        assert!(!ptr.is_null() && guard.is_alive());
        Self {
            ptr,
            qobject: ptr.cast(), // QLayout's primary Qt base is QObject
            _widget: (),
            guard,
            _not_send: PhantomData,
        }
    }
}

object_wrapper!(QBoxLayout, ffi::QBoxLayout);

impl QBoxLayout {
    /// Borrow QLayout without allocating another QPointer.
    pub fn as_layout(&self) -> QLayout {
        QLayout::from_guarded(self.live_ptr().cast(), self.guard.clone())
    }
}

macro_rules! layout_wrapper {
    ($name:ident, $ffi:ty, $new:ident) => {
        object_wrapper!($name, $ffi);
        impl $name {
            /// Borrow the QLayout base without allocating another QPointer.
            pub fn as_layout(&self) -> QLayout {
                QLayout::from_guarded(self.live_ptr().cast(), self.guard.clone())
            }
            /// When parent is Some, the layout installs directly on the parent widget.
            pub fn new(parent: Option<&QWidget>) -> Owned<$name> {
                require_application();
                Owned::new(Self::from_raw(unsafe { ffi::$new(opt_ptr(parent)) }))
            }
            pub fn add_widget(&self, w: &QWidget) {
                unsafe { ffi::layout_add_widget(self.live_ptr().cast(), w.live_ptr()) }
            }
            pub fn add_widget_ex(&self, w: &QWidget, stretch: i32, alignment: i32) {
                unsafe {
                    ffi::layout_add_widget_ex(
                        self.live_ptr().cast(),
                        w.live_ptr(),
                        stretch,
                        alignment,
                    )
                }
            }
            pub fn add_stretch(&self, stretch: i32) {
                unsafe { ffi::layout_add_stretch(self.live_ptr().cast(), stretch) }
            }
            pub fn add_layout<L: AsLayout>(&self, child: &L) {
                unsafe { ffi::layout_add_layout(self.live_ptr().cast(), child.layout_ptr()) }
            }
            pub fn set_spacing(&self, spacing: i32) {
                unsafe { ffi::layout_set_spacing(self.live_ptr().cast(), spacing) }
            }
            pub fn set_contents_margins(&self, l: i32, t: i32, r: i32, b: i32) {
                unsafe { ffi::layout_set_contents_margins(self.live_ptr().cast(), l, t, r, b) }
            }
        }
    };
}

/// A Qt layout that can be passed to another layout.
/// # Safety
/// `layout_ptr()` must return a live QLayout pointer on the owning GUI thread;
/// an invalid implementation makes safe `add_layout` dereference invalid memory.
pub unsafe trait AsLayout {
    fn layout_ptr(&self) -> *mut ffi::QLayout;
}

unsafe impl AsLayout for QLayout {
    fn layout_ptr(&self) -> *mut ffi::QLayout {
        self.live_ptr()
    }
}
unsafe impl AsLayout for QBoxLayout {
    fn layout_ptr(&self) -> *mut ffi::QLayout {
        self.live_ptr().cast()
    }
}

layout_wrapper!(QVBoxLayout, ffi::QVBoxLayout, vbox_new);
layout_wrapper!(QHBoxLayout, ffi::QHBoxLayout, hbox_new);

unsafe impl AsLayout for QVBoxLayout {
    fn layout_ptr(&self) -> *mut ffi::QLayout {
        self.live_ptr().cast()
    }
}
unsafe impl AsLayout for QHBoxLayout {
    fn layout_ptr(&self) -> *mut ffi::QLayout {
        self.live_ptr().cast()
    }
}

// ---- model/view ----

object_wrapper!(QAbstractItemModel, ffi::QAbstractItemModel);

impl QAbstractItemModel {
    fn from_guarded(ptr: *mut ffi::QAbstractItemModel, guard: Rc<dtk_sys::ObjectGuard>) -> Self {
        assert!(!ptr.is_null() && guard.is_alive());
        Self {
            ptr,
            qobject: ptr.cast(), // QAbstractItemModel's primary base is QObject
            _widget: (),
            guard,
            _not_send: PhantomData,
        }
    }
    /// Make this non-widget QObject a child of its view (setModel alone does
    /// not transfer ownership in Qt). The parent must stay on the GUI thread.
    pub fn set_parent(&self, parent: &impl Signal0) {
        unsafe { ffi::model_set_parent(self.live_ptr(), parent.qobject_ptr()) }
    }
    pub fn row_count(&self) -> i32 {
        unsafe { ffi::model_row_count(self.live_ptr()) }
    }
    pub fn column_count(&self) -> i32 {
        unsafe { ffi::model_column_count(self.live_ptr()) }
    }
    pub fn data(&self, row: i32, col: i32, role: i32) -> QVariant {
        QVariant::from_raw(unsafe { ffi::model_data(self.live_ptr(), row, col, role) })
    }
    pub fn set_data(&self, row: i32, col: i32, data: &QVariant, role: i32) -> bool {
        unsafe { ffi::model_set_data(self.live_ptr(), row, col, data.ptr, role) }
    }
    pub fn insert_rows(&self, row: i32, count: i32) -> bool {
        unsafe { ffi::model_insert_rows(self.live_ptr(), row, count) }
    }
    pub fn remove_rows(&self, row: i32, count: i32) -> bool {
        unsafe { ffi::model_remove_rows(self.live_ptr(), row, count) }
    }
}

object_wrapper!(QStandardItemModel, ffi::QStandardItemModel);

impl QStandardItemModel {
    /// The Qt parent owns this model; pass its view when appropriate.
    pub fn new(parent: &impl Signal0) -> Owned<QStandardItemModel> {
        require_application();
        Owned::new(Self::from_raw(unsafe {
            ffi::standard_item_model_new(parent.qobject_ptr())
        }))
    }
    pub fn set_column_count(&self, count: i32) {
        unsafe { ffi::standard_item_model_set_column_count(self.live_ptr(), count) }
    }
    pub fn as_model(&self) -> QAbstractItemModel {
        let ptr = unsafe { ffi::standard_item_model_as_model(self.live_ptr()) };
        QAbstractItemModel::from_guarded(ptr, self.guard.clone())
    }
}

impl widgets::DVariantListModel {
    /// Borrow the QAbstractItemModel base without allocating a second guard.
    pub fn as_model(&self) -> QAbstractItemModel {
        let ptr =
            unsafe { dtk_sys::gen_ffi::genffi::gen_d_variant_list_model_as_model(self.live_ptr()) }
                as *mut ffi::QAbstractItemModel;
        // Both bridges name the same C++ base; C++ performed the actual upcast.
        QAbstractItemModel::from_guarded(ptr, self.guard.clone())
    }
}

// ---- QTableWidget ----

widget_wrapper!(QTableWidget, ffi::QTableWidget);

impl QTableWidget {
    pub fn new(rows: i32, cols: i32) -> Owned<QTableWidget> {
        require_application();
        Owned::new(Self::from_raw(unsafe { ffi::table_new(rows, cols) }))
    }
    pub fn set_column_count(&self, cols: i32) {
        unsafe { ffi::table_set_column_count(self.live_ptr(), cols) }
    }
    pub fn set_row_count(&self, rows: i32) {
        unsafe { ffi::table_set_row_count(self.live_ptr(), rows) }
    }
    /// Set one header label per column; strings are passed verbatim to Qt,
    /// including separators and embedded NUL (no delimiter-encoded protocol).
    pub fn set_horizontal_header_labels<S: AsRef<str>>(&self, labels: &[S]) {
        let values = labels.iter().map(|s| s.as_ref().to_owned()).collect();
        unsafe { ffi::table_set_horizontal_header_labels(self.live_ptr(), values) }
    }
    /// Text of an existing horizontal header item; empty for a missing item.
    pub fn horizontal_header_label(&self, col: i32) -> String {
        unsafe { ffi::table_horizontal_header_label(self.live_ptr(), col) }
    }
    /// Set text without replacing an existing item; return false if out of bounds.
    pub fn set_cell_text(&self, row: i32, col: i32, text: &str) -> bool {
        unsafe { ffi::table_set_cell_text(self.live_ptr(), row, col, text) }
    }
    /// Store i64 UserRole data (e.g. a pid), creating an item when needed.
    /// Return false if out of bounds; no allocation occurs for invalid cells.
    pub fn set_cell_data(&self, row: i32, col: i32, data: i64) -> bool {
        unsafe { ffi::table_set_cell_data(self.live_ptr(), row, col, data) }
    }
    pub fn cell_data(&self, row: i32, col: i32) -> i64 {
        unsafe { ffi::table_cell_data(self.live_ptr(), row, col) }
    }
    pub fn cell_text(&self, row: i32, col: i32) -> String {
        unsafe { ffi::table_cell_text(self.live_ptr(), row, col) }
    }
    pub fn current_row(&self) -> i32 {
        unsafe { ffi::table_current_row(self.live_ptr()) }
    }
    pub fn row_count(&self) -> i32 {
        unsafe { ffi::table_row_count(self.live_ptr()) }
    }
    /// row selection, read-only, single-select
    pub fn select_rows_readonly(&self) {
        unsafe { ffi::table_select_rows_readonly(self.live_ptr()) }
    }
    pub fn header_stretch_last(&self, stretch: bool) {
        unsafe { ffi::table_header_stretch_last(self.live_ptr(), stretch) }
    }
    pub fn set_column_width(&self, col: i32, width: i32) {
        unsafe { ffi::table_set_column_width(self.live_ptr(), col, width) }
    }
    /// A coordinate view of the current item, or None if this cell is empty.
    /// The view re-resolves (row, col) on every call, so replacing the cell's
    /// item changes which item it operates on; it never holds a dangling item*.
    pub fn item(&self, row: i32, col: i32) -> Option<TableCell> {
        let p = unsafe { ffi::table_item(self.live_ptr(), row, col) };
        (!p.is_null()).then(|| TableCell {
            table: self.clone(),
            row,
            col,
        })
    }
    pub fn select_row(&self, row: i32) {
        unsafe { ffi::table_select_row(self.live_ptr(), row) }
    }
    pub fn hide_headers(&self, horizontal: bool, vertical: bool) {
        unsafe { ffi::table_hide_headers(self.live_ptr(), horizontal, vertical) }
    }
    /// use qt::header_resize::* constants for mode
    pub fn set_section_resize_mode(&self, col: i32, mode: i32) {
        unsafe { ffi::table_set_section_resize_mode(self.live_ptr(), col, mode) }
    }
    pub fn set_vertical_header_default_section_size(&self, size: i32) {
        unsafe { ffi::table_set_vertical_header_default_section_size(self.live_ptr(), size) }
    }
    pub fn set_show_grid(&self, show: bool) {
        unsafe { ffi::table_set_show_grid(self.live_ptr(), show) }
    }
    /// use qt::frame::* constants for shape
    pub fn set_frame_shape(&self, shape: i32) {
        unsafe { ffi::table_set_frame_shape(self.live_ptr(), shape) }
    }
    pub fn set_icon_size(&self, w: i32, h: i32) {
        unsafe { ffi::table_set_icon_size(self.live_ptr(), w, h) }
    }
    pub fn set_delegate_for_column(&self, col: i32, delegate: &PaintDelegate) {
        unsafe { ffi::table_set_delegate_for_column(self.live_ptr(), col, delegate.live_ptr()) }
    }
    /// Returns the callback id for [`unregister_callback`]; 0 = connect failed.
    pub fn on_selection_changed(&self, f: impl FnMut() + 'static) -> usize {
        self.connect_signal("itemSelectionChanged()", f)
    }
    /// Returns the callback id for [`unregister_callback`]; 0 = connect failed.
    pub fn on_current_row_changed(&self, f: impl FnMut(i32) + 'static) -> usize {
        self.connect_signal_i32("currentRowChanged(int)", f)
    }
}

// ---- QTimer ----

core_object_wrapper!(QTimer, ffi::QTimer);

impl QTimer {
    /// The parent owns the timer; use the scoped QtCore app, GUI app or a
    /// guarded QObject/widget as parent.
    pub fn new(parent: &impl Signal0) -> Owned<QTimer> {
        require_core_application();
        Owned::new(Self::from_raw(unsafe {
            ffi::timer_new(parent.qobject_ptr())
        }))
    }
    pub fn start(&self, msec: i32) {
        unsafe { ffi::timer_start(self.live_ptr(), msec) }
    }
    pub fn stop(&self) {
        unsafe { ffi::timer_stop(self.live_ptr()) }
    }
    /// Returns the callback id for [`unregister_callback`]; 0 = connect failed.
    pub fn on_timeout(&self, f: impl FnMut() + 'static) -> usize {
        self.connect_signal("timeout()", f)
    }
    /// One-shot timer; its closure is released after firing (even on panic),
    /// or when Qt cancels it because the application is destroyed.
    pub fn single_shot(msec: i32, f: impl FnOnce() + 'static) {
        require_core_application();
        let id = dtk_sys::register_cb_once(f);
        unsafe { ffi::timer_single_shot(msec, id) }
    }
}

// ---- table-cell coordinate view (not a borrowed QTableWidgetItem*) ----

#[derive(Clone)]
pub struct TableCell {
    table: QTableWidget,
    row: i32,
    col: i32,
}

impl TableCell {
    fn with_item<T>(&self, f: impl FnOnce(*mut ffi::QTableWidgetItem) -> T) -> Option<T> {
        let item = unsafe { ffi::table_item(self.table.live_ptr(), self.row, self.col) };
        (!item.is_null()).then(|| f(item))
    }
    /// Returns false if the cell is empty or its item has since been removed.
    pub fn set_icon(&self, icon: &QIcon) -> bool {
        self.with_item(|item| unsafe { ffi::item_set_icon(item, icon.ptr) })
            .is_some()
    }
    pub fn set_text_alignment(&self, alignment: i32) -> bool {
        self.with_item(|item| unsafe { ffi::item_set_text_alignment(item, alignment) })
            .is_some()
    }
    pub fn set_foreground(&self, color: &QColor) -> bool {
        self.with_item(|item| unsafe { ffi::item_set_foreground(item, color.ptr) })
            .is_some()
    }
    pub fn set_data_string(&self, role: i32, value: &str) -> bool {
        self.with_item(|item| unsafe { ffi::item_set_data_string(item, role, value) })
            .is_some()
    }
    pub fn data_string(&self, role: i32) -> Option<String> {
        self.with_item(|item| unsafe { ffi::item_data_string(item, role) })
    }
    pub fn set_data_bool(&self, role: i32, value: bool) -> bool {
        self.with_item(|item| unsafe { ffi::item_set_data_bool(item, role, value) })
            .is_some()
    }
    pub fn data_bool(&self, role: i32) -> Option<bool> {
        self.with_item(|item| unsafe { ffi::item_data_bool(item, role) })
    }
}

// ---- PaintDelegate: custom cell painting ----

object_wrapper!(PaintDelegate, ffi::QStyledItemDelegate);

/// painter handed to the paint callback (valid only inside it)
pub struct Painter {
    ptr: *mut ffi::QPainter,
}

impl Painter {
    /// draw text with the baseline at (x, y) — for cell-grid rendering
    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        unsafe { ffi::painter_draw_line(self.ptr, x1, y1, x2, y2) }
    }
    pub fn draw_text_at(&self, x: i32, y: i32, text: &str) {
        unsafe { ffi::painter_draw_text_at(self.ptr, x, y, text) }
    }
    pub fn save(&self) {
        unsafe { ffi::painter_save(self.ptr) }
    }
    pub fn restore(&self) {
        unsafe { ffi::painter_restore(self.ptr) }
    }
    pub fn set_pen_color(&self, color: &QColor) {
        unsafe { ffi::painter_set_pen_color(self.ptr, color.ptr) }
    }
    pub fn set_font(&self, font: &QFont) {
        unsafe { ffi::painter_set_font(self.ptr, font.ptr) }
    }
    /// combine qt::alignment::* for flags
    pub fn draw_text(&self, x: i32, y: i32, w: i32, h: i32, flags: i32, text: &str) {
        unsafe { ffi::painter_draw_text(self.ptr, x, y, w, h, flags, text) }
    }
    pub fn draw_pixmap(&self, x: i32, y: i32, w: i32, h: i32, pm: &QPixmap) {
        unsafe { ffi::painter_draw_pixmap(self.ptr, x, y, w, h, pm.ptr) }
    }
    pub fn draw_icon(&self, x: i32, y: i32, w: i32, h: i32, icon: &QIcon) {
        unsafe { ffi::painter_draw_icon(self.ptr, x, y, w, h, icon.ptr) }
    }
    pub fn fill_rect(&self, x: i32, y: i32, w: i32, h: i32, color: &QColor) {
        unsafe { ffi::painter_fill_rect(self.ptr, x, y, w, h, color.ptr) }
    }
    pub fn set_clip_rect(&self, x: i32, y: i32, w: i32, h: i32) {
        unsafe { ffi::painter_set_clip_rect(self.ptr, x, y, w, h) }
    }
    /// move the painter origin — compose with `scale` to draw scaled content
    pub fn translate(&self, x: f64, y: f64) {
        unsafe { ffi::painter_translate(self.ptr, x, y) }
    }
    /// scale the coordinate system about the current origin (uniform for glyphs)
    pub fn scale(&self, sx: f64, sy: f64) {
        unsafe { ffi::painter_scale(self.ptr, sx, sy) }
    }
    /// elide text to width using the painter's current font; use qt::elide::* for mode
    pub fn elided_text(&self, text: &str, mode: i32, width: i32) -> String {
        unsafe { ffi::painter_elided_text(self.ptr, text, mode, width) }
    }
}

/// model index handed to the paint callback (valid only inside it)
pub struct ModelIndex {
    ptr: *mut ffi::QModelIndex,
}

impl ModelIndex {
    pub fn data_string(&self, role: i32) -> String {
        unsafe { ffi::index_data_string(self.ptr, role) }
    }
    pub fn data_bool(&self, role: i32) -> bool {
        unsafe { ffi::index_data_bool(self.ptr, role) }
    }
    pub fn data_i64(&self, role: i32) -> i64 {
        unsafe { ffi::index_data_i64(self.ptr, role) }
    }
}

impl PaintDelegate {
    /// The parent owns this delegate; use its view/table as parent.
    /// f: (painter, index, x, y, w, h, state). test state against qt::state::*
    pub fn new(
        parent: &impl Signal0,
        f: impl FnMut(&Painter, &ModelIndex, i32, i32, i32, i32, i32) + 'static,
    ) -> Owned<PaintDelegate> {
        require_application();
        let parent_ptr = parent.qobject_ptr();
        let mut f = f;
        let id = dtk_sys::register_cb_paint(move |data| {
            f(
                &Painter { ptr: data.painter },
                &ModelIndex { ptr: data.index },
                data.x,
                data.y,
                data.w,
                data.h,
                data.state,
            );
        });
        Owned::new(Self::from_raw(unsafe {
            ffi::rust_delegate_new(id, parent_ptr)
        }))
    }
}

// ---- QSocketNotifier ----

core_object_wrapper!(QSocketNotifier, ffi::QSocketNotifier);

impl QSocketNotifier {
    /// Watch an fd for readability (Read type); the parent owns the notifier.
    pub fn new(fd: i32, parent: &impl Signal0) -> Owned<QSocketNotifier> {
        require_core_application();
        Owned::new(Self::from_raw(unsafe {
            ffi::socket_notifier_new(fd, parent.qobject_ptr())
        }))
    }
    /// Stop firing (e.g. a watched fd hit EOF and stays readable).
    pub fn set_enabled(&self, on: bool) {
        unsafe { ffi::socket_notifier_set_enabled(self.live_ptr(), on) }
    }
    pub fn on_activated(&self, f: impl FnMut() + 'static) -> usize {
        self.connect_signal("activated(QSocketDescriptor)", f)
    }
}

// ---- DProgressBar: base-class QProgressBar ops (generator only scans DTK headers) ----

impl widgets::DProgressBar {
    pub fn set_value(&self, value: i32) {
        unsafe { ffi::progressbar_set_value(self.live_ptr().cast(), value) }
    }
    /// (0, 0) = busy/indeterminate mode
    pub fn set_range(&self, minimum: i32, maximum: i32) {
        unsafe { ffi::progressbar_set_range(self.live_ptr().cast(), minimum, maximum) }
    }
    pub fn value(&self) -> i32 {
        unsafe { ffi::progressbar_value(self.live_ptr().cast()) }
    }
}

impl_default!(QFont, QPalette, DPalette, QPainterPath, DDciIcon);
impl_default_owned!(DMainWindow, DMessageBox);

// ---- typed signal helpers + base-class getters for generated widgets ----

impl widgets::DLineEdit {
    /// Returns the callback id for [`unregister_callback`]; 0 = connect failed.
    pub fn on_return_pressed(&self, f: impl FnMut() + 'static) -> usize {
        self.connect_signal("returnPressed()", f)
    }
}

impl widgets::DSwitchButton {
    /// checkedChanged(bool) with the bool delivered. Returns id; 0 = connect failed.
    pub fn on_checked_changed(&self, f: impl FnMut(bool) + 'static) -> usize {
        self.connect_signal_bool("checkedChanged(bool)", f)
    }
}

impl widgets::DSearchEdit {
    /// QLineEdit base-class getter, via the DLineEdit ancestry
    pub fn text(&self) -> String {
        unsafe { ffi::line_edit_text(self.live_ptr().cast()) }
    }
}

impl widgets::DComboBox {
    /// currentIndexChanged(int). Returns id; 0 = connect failed.
    pub fn on_current_index_changed(&self, f: impl FnMut(i32) + 'static) -> usize {
        self.connect_signal_i32("currentIndexChanged(int)", f)
    }
}

// generator output: bindings for the rest of dtkwidget
pub mod core;
pub mod gui;
pub mod widgets;

impl core::DConfig {
    // Both native factories explicitly transfer their returned QObject to
    // callers, regardless of whether the configuration metadata is valid.
    fn from_created(ptr: *mut ffi::DConfig) -> Option<Owned<Self>> {
        (!ptr.is_null()).then(|| Owned::new(Self::from_raw(ptr as _)))
    }

    /// Create an app-independent configuration. Null is None; check
    /// `is_valid()` separately for missing config metadata.
    /// Requires a scoped QCoreApplication or a GUI DApplication on its owning
    /// thread. Either application scope invalidates this object on exit even
    /// if its Rust owner or view escapes.
    pub fn create_generic(name: &str, subpath: &str) -> Option<Owned<Self>> {
        require_core_application();
        Self::from_created(unsafe { ffi::core_config_create_generic(name, subpath) })
    }

    /// Create an application-specific configuration with the given app ID.
    /// Native DCore transfers ownership even when `is_valid()` is false.
    pub fn create_for_app(app_id: &str, name: &str, subpath: &str) -> Option<Owned<Self>> {
        require_core_application();
        Self::from_created(unsafe { ffi::core_config_create(app_id, name, subpath) })
    }
}
