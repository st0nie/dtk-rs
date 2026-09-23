//! Generated QLayout* mapping and native DTK layout ownership.
//! QT_QPA_PLATFORM=offscreen cargo run --example layout_selfcheck
use dtk::widgets::{DBaseLine, DFlowLayout};
use dtk::{DApplication, QTimer, QVBoxLayout, QWidget};
use std::cell::Cell;
use std::rc::Rc;

fn main() {
    let app = DApplication::new("dtk-layout-selfcheck");
    DApplication::set_quit_on_last_window_closed(false);
    let flow = DFlowLayout::new();
    let child = QVBoxLayout::new(None);
    let base = child.as_layout();
    assert!(base.is_alive());
    assert_eq!(flow.count(), 0);
    flow.insert_layout(0, &base);
    assert_eq!(flow.count(), 1);
    flow.delete_later();

    let line = DBaseLine::new();
    let content = QWidget::new(None);
    content.set_parent(&line.as_widget());
    line.set_left_content(&content);
    let left = line
        .left_layout()
        .expect("DBaseLine must expose its left layout");
    assert!(left.as_layout().is_alive());
    line.delete_later();

    let observed = Rc::new(Cell::new((false, false, false, false, false, false)));
    let result = observed.clone();
    QTimer::single_shot(50, move || {
        result.set((
            !flow.is_alive(),
            !base.is_alive(),
            !child.is_alive(),
            !line.is_alive(),
            !left.is_alive(),
            !content.is_alive(),
        ));
        DApplication::quit();
    });
    assert_eq!(app.exec(), 0);
    assert_eq!(
        observed.get(),
        (true, true, true, true, true, true),
        "Qt must delete child layouts and content with their owners"
    );
    println!("layout selfcheck ok");
}
