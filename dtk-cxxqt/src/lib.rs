//! CXX-Qt integration layer for dtk-rs.
//!
//! This crate only imports existing DTK QObject classes (`extern "C++Qt"`).
//! It never constructs, owns, deletes or mutably borrows them: the safe
//! `dtk` crate supplies live, guarded pointers inside a `dtk_sys::NativeCall`.
//! Typed signal functors own a [`SignalToken`] and queue owned argument
//! values into the shared dtk-sys callback runtime.
#![allow(clippy::missing_safety_doc)] // one shared contract, documented below

mod bridge;
pub use bridge::ffi;
pub use cxx_qt_lib::QString;
use dtk_sys::native_signal::SignalToken;
use std::any::Any;

macro_rules! connectors {
    ($( $name:ident: $ty:ident :: $on:ident ( $($p:ident : $pty:ty),* ) -> $arg:ty = $conv:expr; )*) => {$(
        /// Connect with Qt `AutoConnection` and the sender as context, so the
        /// functor dies with the sender. The functor never exposes the sender
        /// reference: it converts arguments into owned values in the emission
        /// frame and queues them via `token`. Dropping the returned guard
        /// disconnects (a no-op once Qt already destroyed the connection).
        ///
        /// # Safety
        /// `sender` must point to a live object of this exact C++ type, on its
        /// Qt owner thread, while the caller holds a dtk-sys `NativeCall`.
        pub unsafe fn $name(sender: *const ffi::$ty, token: SignalToken<$arg>) -> Box<dyn Any> {
            let sender = unsafe { &*sender };
            Box::new(sender.$on(move |_sender, $($p: $pty),*| token.emit($conv)))
        }
    )*};
}

connectors! {
    d_switch_button_checked_changed: DSwitchButton::on_checked_changed(checked: bool) -> (bool,) = (checked,);

    d_line_edit_text_changed: DLineEdit::on_text_changed(text: &QString) -> (String,) = (String::from(text),);
    d_line_edit_text_edited: DLineEdit::on_text_edited(text: &QString) -> (String,) = (String::from(text),);
    d_line_edit_return_pressed: DLineEdit::on_return_pressed() -> () = ();
    d_line_edit_editing_finished: DLineEdit::on_editing_finished() -> () = ();
    d_line_edit_alert_changed: DLineEdit::on_alert_changed(alert: bool) -> (bool,) = (alert,);
    d_line_edit_focus_changed: DLineEdit::on_focus_changed(focus: bool) -> (bool,) = (focus,);

    d_search_edit_text_changed: DSearchEdit::on_text_changed(text: &QString) -> (String,) = (String::from(text),);
    d_search_edit_return_pressed: DSearchEdit::on_return_pressed() -> () = ();
    d_search_edit_search_aborted: DSearchEdit::on_search_aborted() -> () = ();

    d_slider_value_changed: DSlider::on_value_changed(value: i32) -> (i32,) = (value,);
    d_slider_slider_pressed: DSlider::on_slider_pressed() -> () = ();
    d_slider_slider_released: DSlider::on_slider_released() -> () = ();
    d_slider_range_changed: DSlider::on_range_changed(min: i32, max: i32) -> (i32, i32) = (min, max);

    d_tab_bar_current_changed: DTabBar::on_current_changed(index: i32) -> (i32,) = (index,);
    d_tab_bar_tab_close_requested: DTabBar::on_tab_close_requested(index: i32) -> (i32,) = (index,);
    d_tab_bar_tab_moved: DTabBar::on_tab_moved(from: i32, to: i32) -> (i32, i32) = (from, to);
    d_tab_bar_tab_add_requested: DTabBar::on_tab_add_requested() -> () = ();

    d_dialog_button_clicked: DDialog::on_button_clicked(index: i32, text: &QString) -> (i32, String) = (index, String::from(text));
    d_dialog_about_to_close: DDialog::on_about_to_close() -> () = ();
    d_dialog_closed: DDialog::on_closed() -> () = ();
    d_dialog_visible_changed: DDialog::on_visible_changed(visible: bool) -> (bool,) = (visible,);
}

/// Inherited `QAbstractButton::isChecked() const` through a shared `&T`.
///
/// # Safety
/// Same contract as the connectors: live sender, owner thread, `NativeCall`.
pub unsafe fn d_switch_button_is_checked(button: *const ffi::DSwitchButton) -> bool {
    unsafe { &*button }.is_checked()
}
