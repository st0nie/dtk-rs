//! Tour of common DTK controls and how to wire their signals.
//!
//! Run:  cargo run --example controls
//!
//! Signals connect by name string: any signal whose arguments you don't need
//! works with `connect_signal` (e.g. "checkedChanged(bool)"); signals with one
//! int argument work with `connect_signal_i32` (e.g. "currentRowChanged(int)").

use dtk::widgets::*;
use dtk::*;

use std::rc::Rc;

fn main() {
    let app = DApplication::new("dtk-rs-controls");
    app.load_translator();

    let win = DMainWindow::new();
    win.set_window_title("Controls");
    win.titlebar().set_title("DTK Controls");

    let central = Rc::new(QWidget::new(None));
    let vbox = QVBoxLayout::new(Some(&central));
    vbox.set_contents_margins(24, 24, 24, 24);
    vbox.set_spacing(12);

    // --- line edit with placeholder + alert state ---
    let edit = DLineEdit::new();
    edit.set_placeholder_text("Type something, press Enter");
    vbox.add_widget(&edit.as_widget());
    edit.connect_signal("returnPressed()", || {
        // no text getter yet (base-class method) — signal itself is the demo
        println!("return pressed");
    });

    // --- search edit (built-in clear button) ---
    let search = DSearchEdit::new();
    search.set_place_holder("Search…");
    vbox.add_widget(&search.as_widget());

    // --- switch toggling a spinner ---
    let row = QHBoxLayout::new(None);
    let switch = DSwitchButton::new();
    let spinner = Rc::new(DSpinner::new());
    spinner.set_fixed_size(24, 24);
    row.add_widget(&switch.as_widget());
    row.add_widget(&spinner.as_widget());
    row.add_stretch(1);
    vbox.add_layout(&row);

    // DSwitchButton::checkedChanged(bool) — the bool relay delivers the arg
    {
        let spinner = spinner.clone();
        switch.on_checked_changed(move |on| if on { spinner.start() } else { spinner.stop() });
    }

    // --- floating message (DDE-style toast) ---
    let toast_btn = DSuggestButton::new("Show floating message");
    vbox.add_widget(&toast_btn.as_widget());
    {
        let central = central.clone();
        toast_btn.on_clicked(move || {
            DMessageManager::instance()
                .expect("DTK message manager unavailable")
                .send_message_2(
                    &central,
                    &QIcon::from_theme("dialog-ok"),
                    "Saved successfully",
                );
        });
    }

    win.set_central_widget(&central);
    win.resize(420, 240);
    win.show();

    std::process::exit(app.exec());
}
