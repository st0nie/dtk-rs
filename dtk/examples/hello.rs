//! Minimal dtk-rs app: window, label, button, message box.
//!
//! Run:  cargo run --example hello
//!
//! Rules shown here:
//! - all Qt objects live on the GUI thread; wrappers are non-owning and `!Send`
//! - callbacks must be `'static` → share widgets via `Rc`
//! - Qt's parent-child tree owns the objects; Rust wrapper handles can be
//!   forgotten once only callbacks reference them

use dtk::*;
use std::cell::Cell;
use std::rc::Rc;

fn main() {
    let code = DApplication::with("dtk-rs-hello", |app| {
        app.load_translator(); // DTK built-in translations (zh_CN etc.)

        let win = DMainWindow::new();
        win.set_window_title("Hello DTK");
        win.titlebar().set_title("Hello DTK");
        win.titlebar()
            .set_icon(&QIcon::from_theme("preferences-system"));

        let central = QWidget::new(None);
        let vbox = QVBoxLayout::new(Some(&central)); // installs on `central` directly
        vbox.set_contents_margins(24, 24, 24, 24);
        vbox.set_spacing(12);

        let label = Rc::new(DLabel::new("Not clicked yet."));
        label.set_alignment(qt::alignment::CENTER);
        vbox.add_widget(&label.as_widget());

        let btn = DSuggestButton::new("Click me");
        vbox.add_widget(&btn.as_widget());

        // Rc + Cell: share state with the 'static callback (no Send/Sync needed,
        // everything stays on the GUI thread).
        let count = Rc::new(Cell::new(0i32));
        {
            let label = label.clone();
            let count = count.clone();
            btn.on_clicked(move |_| {
                let n = count.get() + 1;
                count.set(n);
                label.set_text(&format!("Clicked {n} time(s)."));
            })
            .release();
        }

        let about_btn = DPushButton::new("About");
        vbox.add_widget(&about_btn.as_widget());
        about_btn
            .on_clicked(|_| {
                DMessageBox::about(None, "About", "Hello from dtk-rs!"); // modal, blocks
            })
            .release();

        win.set_central_widget(&central);
        win.resize(360, 180);
        win.show();

        if DApplication::has_arg("--smoke") {
            QTimer::single_shot(100, DApplication::quit);
        }
        app.exec()
    });
    std::process::exit(code);
}
