//! PaintWidget self-check: custom painting + input events reach the Rust handler.
//! Run: QT_QPA_PLATFORM=offscreen ./target/debug/examples/paint_widget --smoke
use dtk::*;
use std::cell::Cell;
use std::rc::Rc;

fn main() {
    let code = DApplication::with("paint_widget", |app| {
        let win = DMainWindow::new();

        let painted = Rc::new(Cell::new(false));
        let resized = Rc::new(Cell::new(false));
        let keyed = Rc::new(Cell::new(false));
        let tabbed = Rc::new(Cell::new(false));

        let pw = PaintWidget::new(None, {
            let painted = painted.clone();
            let resized = resized.clone();
            let keyed = keyed.clone();
            let tabbed = tabbed.clone();
            move |ev| match ev {
                PaintWidgetEvent::Paint(p, w, h) => {
                    p.fill_rect(0, 0, w, h, &QColor::from_rgb(30, 30, 30));
                    let font = QFont::new();
                    font.set_point_size(14);
                    p.set_font(&font);
                    p.set_pen_color(&QColor::from_rgb(230, 230, 230));
                    p.draw_text(10, 10, 200, 30, qt::alignment::LEFT, "hello paint");
                    painted.set(true);
                }
                PaintWidgetEvent::Resize { w, h } => {
                    assert!(w > 0 && h > 0);
                    resized.set(true);
                }
                PaintWidgetEvent::Key(k) => {
                    assert!(k.press);
                    if k.key == qt::key::TAB {
                        // Tab must reach the handler, not the focus framework
                        tabbed.set(true);
                        return;
                    }
                    assert_eq!(k.key, i32::from(b'A'));
                    assert_eq!(k.text, "a");
                    assert_eq!(k.mods, qt::modifier::CONTROL);
                    keyed.set(true);
                }
                _ => {}
            }
        });
        win.set_central_widget(&pw.as_widget());
        win.resize(300, 200);
        win.show();

        if std::env::args().any(|a| a == "--smoke") {
            // clipboard round-trip (offscreen platform keeps it in-process)
            Clipboard::set_text("dtk-clip-test");
            assert_eq!(Clipboard::text(), "dtk-clip-test");
            // shortcut registration (fires only with a real window focus; creation is the check)
            let _sc = pw.as_widget().add_shortcut("Ctrl+Shift+C", || {});
            pw.inject_key(i32::from(b'A'), qt::modifier::CONTROL, "a");
            pw.inject_key(qt::key::TAB, 0, "\t");
            QTimer::single_shot(500, move || {
                assert!(painted.get(), "paint callback never fired");
                assert!(resized.get(), "resize event never fired");
                assert!(keyed.get(), "key event never fired");
                assert!(tabbed.get(), "tab key never fired");
                println!("smoke ok");
                DApplication::quit();
            });
        }
        app.exec()
    });
    std::process::exit(code);
}
