//! Typed signal connections through CXX-Qt imports of DTK QObjects.
//!
//! Each `on_*` method connects one statically typed Qt signal (no runtime
//! signature strings) and returns a [`Connection`] guard. The model is the
//! same as the string relay helpers:
//!
//! - The connection is made inside a `NativeCall` on a live, guarded handle.
//! - Qt arguments are converted to owned Rust values (`QString` -> `String`)
//!   inside the emission; the closure runs later, after every native frame
//!   has returned, and is never re-entered (nested emissions queue FIFO).
//! - The closure never receives the sender. Capture a clone of the handle if
//!   needed; its calls go through the ordinary guarded wrappers.
//! - Qt destroying the sender cancels the callback and releases its captures
//!   at a Rust cleanup boundary; application scope exit disconnects the rest.
//!
//! No API here creates `Pin<&mut T>` or `&mut T` for a native object.

use crate::widgets;
use dtk_sys::native_signal;
use std::marker::PhantomData;

/// RAII handle for one typed signal connection.
///
/// Dropping it disconnects and releases the closure. Use [`release`] to keep
/// the connection for the sender's lifetime instead (like the id-returning
/// string relay helpers); the returned id still works with
/// [`crate::unregister_callback`].
///
/// [`release`]: Connection::release
#[must_use = "dropping a Connection disconnects it; call release() to keep it"]
pub struct Connection {
    id: usize,
    _not_send: PhantomData<*mut ()>,
}

impl Connection {
    fn new(id: usize) -> Self {
        assert_ne!(id, 0, "typed Qt signal connection failed");
        Self {
            id,
            _not_send: PhantomData,
        }
    }
    /// Callback id shared with the string relay registry.
    pub fn id(&self) -> usize {
        self.id
    }
    /// False after Qt destroyed the sender (or its scope closed).
    pub fn is_connected(&self) -> bool {
        native_signal::is_connected(self.id)
    }
    /// Disconnect now (same as dropping).
    pub fn disconnect(self) {}
    /// Keep the connection until Qt destroys the sender or the application
    /// scope exits; returns the id for [`crate::unregister_callback`].
    pub fn release(self) -> usize {
        let id = self.id;
        std::mem::forget(self);
        id
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        crate::unregister_callback(self.id);
    }
}

impl std::fmt::Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Connection({})", self.id)
    }
}

macro_rules! typed_signals {
    ($ty:ident => $cxx:ident { $(
        $(#[$doc:meta])*
        fn $method:ident($($arg:ident: $aty:ty),*) => $connector:ident;
    )* }) => {
        impl widgets::$ty {
            $(
                $(#[$doc])*
                pub fn $method(&self, mut f: impl FnMut($($aty),*) + 'static) -> Connection {
                    let _call = dtk_sys::NativeCall::new();
                    let sender = self.live_ptr() as *const dtk_cxxqt::ffi::$cxx;
                    Connection::new(native_signal::connect(
                        move |($($arg,)*): ($($aty,)*)| f($($arg),*),
                        // SAFETY: live guarded pointer of this exact DTK type
                        // (both bridges name the same C++ class), on its Qt
                        // thread, inside this NativeCall.
                        |token| Some(unsafe { dtk_cxxqt::$connector(sender, token) }),
                    ))
                }
            )*
        }
    };
}

typed_signals!(DSwitchButton => DSwitchButton {
    /// `checkedChanged(bool)`.
    fn on_checked_changed(checked: bool) => d_switch_button_checked_changed;
});

typed_signals!(DLineEdit => DLineEdit {
    /// `textChanged(const QString &)`, delivered as an owned `String`.
    fn on_text_changed(text: String) => d_line_edit_text_changed;
    /// `textEdited(const QString &)`: user edits only.
    fn on_text_edited(text: String) => d_line_edit_text_edited;
    /// `returnPressed()`.
    fn on_return_pressed() => d_line_edit_return_pressed;
    /// `editingFinished()`.
    fn on_editing_finished() => d_line_edit_editing_finished;
    /// `alertChanged(bool)`.
    fn on_alert_changed(alert: bool) => d_line_edit_alert_changed;
    /// `focusChanged(bool)`.
    fn on_focus_changed(focused: bool) => d_line_edit_focus_changed;
});

typed_signals!(DSearchEdit => DSearchEdit {
    /// Inherited `DLineEdit::textChanged(const QString &)`.
    fn on_text_changed(text: String) => d_search_edit_text_changed;
    /// Inherited `DLineEdit::returnPressed()`.
    fn on_return_pressed() => d_search_edit_return_pressed;
    /// `searchAborted()`.
    fn on_search_aborted() => d_search_edit_search_aborted;
});

typed_signals!(DSlider => DSlider {
    /// `valueChanged(int)`.
    fn on_value_changed(value: i32) => d_slider_value_changed;
    /// `sliderPressed()`.
    fn on_slider_pressed() => d_slider_slider_pressed;
    /// `sliderReleased()`.
    fn on_slider_released() => d_slider_slider_released;
    /// `rangeChanged(int, int)`.
    fn on_range_changed(min: i32, max: i32) => d_slider_range_changed;
});

typed_signals!(DTabBar => DTabBar {
    /// `currentChanged(int)`.
    fn on_current_changed(index: i32) => d_tab_bar_current_changed;
    /// `tabCloseRequested(int)`.
    fn on_tab_close_requested(index: i32) => d_tab_bar_tab_close_requested;
    /// `tabMoved(int, int)`.
    fn on_tab_moved(from: i32, to: i32) => d_tab_bar_tab_moved;
    /// `tabAddRequested()`.
    fn on_tab_add_requested() => d_tab_bar_tab_add_requested;
});

typed_signals!(DDialog => DDialog {
    /// `buttonClicked(int, const QString &)`.
    fn on_button_clicked(index: i32, text: String) => d_dialog_button_clicked;
    /// `aboutToClose()`.
    fn on_about_to_close() => d_dialog_about_to_close;
    /// `closed()`.
    fn on_closed() => d_dialog_closed;
    /// `visibleChanged(bool)`.
    fn on_visible_changed(visible: bool) => d_dialog_visible_changed;
});

impl widgets::DSwitchButton {
    /// `QAbstractButton::isChecked() const`, called through CXX-Qt's shared
    /// `&T` receiver (cxx opaque types are interior-mutable; `&T` asserts no
    /// immutability or exclusivity of the C++ object).
    pub fn is_checked(&self) -> bool {
        let _call = dtk_sys::NativeCall::new();
        let button = self.live_ptr() as *const dtk_cxxqt::ffi::DSwitchButton;
        unsafe { dtk_cxxqt::d_switch_button_is_checked(button) }
    }
    /// `QAbstractButton::setChecked(bool)`: a non-const method, so it goes
    /// through a raw-pointer dtk-sys shim rather than a CXX-Qt `Pin<&mut T>`.
    pub fn set_checked(&self, checked: bool) {
        let _call = dtk_sys::NativeCall::new();
        unsafe { dtk_sys::ffi::abstract_button_set_checked(self.widget_ptr(), checked) }
    }
}
