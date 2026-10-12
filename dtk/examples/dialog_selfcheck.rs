// ponytail: compile-time self-check for the three dialog classes.
// Verifies DDialog + DMessageManager (generated) and DMessageBox (hand-written) link + have the API surface
// the user asked for (exec / addButton / setTitle / setMessage / setIcon for confirmation dialogs).
use dtk::widgets::{DDialog, DMessageManager};
use dtk::{DApplication, DDciIcon, DMessageBox, QIcon, QMargins, QWidget, qt};

fn _ddialog_usage(show: bool) {
    let d = DDialog::new();
    d.set_title("Confirm");
    d.set_message("Proceed?");
    d.set_icon(&QIcon::from_theme("dialog-warning"));
    d.add_button("OK", true, 0); // ButtonNormal
    d.add_button("Cancel", false, 1); // ButtonWarning
    if show {
        let _r: i32 = d.exec();
    }
}

fn _dmessagebox_usage(show: bool) {
    // static helpers (most common confirmation path)
    if show {
        let _r = DMessageBox::information(None, "Info", "Done", qt::msg_btn::OK, qt::msg_btn::OK);
        let _r = DMessageBox::question(
            None,
            "Ask",
            "Yes or no?",
            qt::msg_btn::YES_NO,
            qt::msg_btn::YES,
        );
    }

    // full instance API
    let mb = DMessageBox::with(
        qt::msg_icon::QUESTION,
        "Ask",
        "Sure?",
        qt::msg_btn::YES_NO,
        None,
    );
    mb.set_informative_text("This cannot be undone.");
    mb.set_default_button(qt::msg_btn::NO);
    let _btn = mb.add_button("Custom", qt::msg_role::ACTION);
    if show {
        let _clicked: i32 = mb.exec();
    }
}

fn _messagemanager_usage() {
    let _mm = DMessageManager::instance().expect("DTK message manager unavailable");
    let _margins = QMargins::new(10, 10, 10, 10);
    let _dci = DDciIcon::new();
    let _dci2 = DDciIcon::from_file("/usr/share/dsg/icons/example.dci");
}

fn main() {
    // smoke: methods exist + types resolve; exec() only with --show (blocks offscreen otherwise)
    let show = std::env::args().any(|a| a == "--show");
    DApplication::with("selfcheck", |_| {
        _ddialog_usage(show);
        _dmessagebox_usage(show);
        _messagemanager_usage();
        let _w = QWidget::new(None);
        println!("selfcheck ok");
    });
}
