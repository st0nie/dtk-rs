//! CXX-Qt imports of existing DTK QObject classes.
//!
//! Borrowing rule (enforced by tools/audit_hand.py): every receiver is
//! `self: &T`. `Pin<&mut T>` is never declared, so no safe or unsafe wrapper
//! can be tempted to fabricate an exclusive native reference from a shared,
//! clonable dtk handle. Signals are `pub(self)`: CXX-Qt then emits only the
//! typed `on_*`/`connect_*` connection functions, not a Rust-callable emitter
//! (a `&self` emitter would also require a C++ `const` signal).
//! Mutating C++ methods stay in dtk-sys raw-pointer shims.

#[cxx_qt::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("dswitchbutton.h");
        include!("dlineedit.h");
        include!("dsearchedit.h");
        include!("dslider.h");
        include!("dtabbar.h");
        include!("ddialog.h");
    }

    unsafe extern "C++Qt" {
        #[namespace = "Dtk::Widget"]
        #[qobject]
        type DSwitchButton;

        #[qsignal]
        #[cxx_name = "checkedChanged"]
        pub(self) fn checked_changed(self: &DSwitchButton, checked: bool);

        /// Inherited `QAbstractButton::isChecked() const`.
        #[cxx_name = "isChecked"]
        fn is_checked(self: &DSwitchButton) -> bool;
    }

    unsafe extern "C++Qt" {
        #[namespace = "Dtk::Widget"]
        #[qobject]
        type DLineEdit;

        #[qsignal]
        #[cxx_name = "textChanged"]
        pub(self) fn text_changed(self: &DLineEdit, text: &QString);
        #[qsignal]
        #[cxx_name = "textEdited"]
        pub(self) fn text_edited(self: &DLineEdit, text: &QString);
        #[qsignal]
        #[cxx_name = "returnPressed"]
        pub(self) fn return_pressed(self: &DLineEdit);
        #[qsignal]
        #[cxx_name = "editingFinished"]
        pub(self) fn editing_finished(self: &DLineEdit);
        #[qsignal]
        #[cxx_name = "alertChanged"]
        pub(self) fn alert_changed(self: &DLineEdit, alert: bool);
        #[qsignal]
        #[cxx_name = "focusChanged"]
        pub(self) fn focus_changed(self: &DLineEdit, on_focus: bool);
    }

    unsafe extern "C++Qt" {
        // Inherited DLineEdit signals are redeclared on the subclass: the
        // generated `&DSearchEdit::textChanged` names the base member, so no
        // Rust-side base-pointer adjustment is needed.
        #[namespace = "Dtk::Widget"]
        #[qobject]
        type DSearchEdit;

        #[qsignal]
        #[cxx_name = "textChanged"]
        pub(self) fn text_changed(self: &DSearchEdit, text: &QString);
        #[qsignal]
        #[cxx_name = "returnPressed"]
        pub(self) fn return_pressed(self: &DSearchEdit);
        #[qsignal]
        #[cxx_name = "searchAborted"]
        pub(self) fn search_aborted(self: &DSearchEdit);
    }

    unsafe extern "C++Qt" {
        #[namespace = "Dtk::Widget"]
        #[qobject]
        type DSlider;

        #[qsignal]
        #[cxx_name = "valueChanged"]
        pub(self) fn value_changed(self: &DSlider, value: i32);
        #[qsignal]
        #[cxx_name = "sliderPressed"]
        pub(self) fn slider_pressed(self: &DSlider);
        #[qsignal]
        #[cxx_name = "sliderReleased"]
        pub(self) fn slider_released(self: &DSlider);
        #[qsignal]
        #[cxx_name = "rangeChanged"]
        pub(self) fn range_changed(self: &DSlider, min: i32, max: i32);
    }

    unsafe extern "C++Qt" {
        #[namespace = "Dtk::Widget"]
        #[qobject]
        type DTabBar;

        #[qsignal]
        #[cxx_name = "currentChanged"]
        pub(self) fn current_changed(self: &DTabBar, index: i32);
        #[qsignal]
        #[cxx_name = "tabCloseRequested"]
        pub(self) fn tab_close_requested(self: &DTabBar, index: i32);
        #[qsignal]
        #[cxx_name = "tabMoved"]
        pub(self) fn tab_moved(self: &DTabBar, from: i32, to: i32);
        #[qsignal]
        #[cxx_name = "tabAddRequested"]
        pub(self) fn tab_add_requested(self: &DTabBar);
    }

    unsafe extern "C++Qt" {
        #[namespace = "Dtk::Widget"]
        #[qobject]
        type DDialog;

        #[qsignal]
        #[cxx_name = "buttonClicked"]
        pub(self) fn button_clicked(self: &DDialog, index: i32, text: &QString);
        #[qsignal]
        #[cxx_name = "aboutToClose"]
        pub(self) fn about_to_close(self: &DDialog);
        #[qsignal]
        #[cxx_name = "closed"]
        pub(self) fn closed(self: &DDialog);
        #[qsignal]
        #[cxx_name = "visibleChanged"]
        pub(self) fn visible_changed(self: &DDialog, visible: bool);
    }
}
