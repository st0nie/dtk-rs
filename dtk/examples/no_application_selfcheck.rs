//! Constructing Qt widgets before QApplication must fail in Rust, not crash Qt.
//! QT_QPA_PLATFORM=offscreen cargo run --example no_application_selfcheck
use dtk::gui::DImageHandler;
use dtk::widgets::{DListView, DMessageManager};
use dtk::{
    Clipboard, DApplication, QPixmap, QTableWidget, QTimer, QVariant, QVariantList, QWidget,
};
#[path = "support/expected_panic.rs"]
mod expected_panic;
use expected_panic::expect_panic;
use std::rc::Rc;

fn main() {
    // QtCore value types remain usable without an event loop.
    assert!(!QVariant::new().is_valid());
    assert!(QVariantList::new().is_empty());
    expect_panic("create DApplication", || QWidget::new(None));
    expect_panic("create DApplication", DListView::new);
    expect_panic("create DApplication", DImageHandler::new);
    expect_panic("create DApplication", DMessageManager::instance);
    expect_panic("create DApplication", || QTableWidget::new(0, 0));
    expect_panic("create DApplication", Clipboard::text);
    expect_panic("create DApplication", || QPixmap::new(""));
    let capture = Rc::new(());
    let weak = Rc::downgrade(&capture);
    expect_panic("create DApplication", || {
        QTimer::single_shot(0, move || {
            let _ = &capture;
        })
    });
    assert!(
        weak.upgrade().is_none(),
        "rejected timer must drop its callback"
    );

    let app = DApplication::new("dtk-no-application-selfcheck");
    DApplication::set_quit_on_last_window_closed(false);
    let widget = QWidget::new(None);
    assert!(widget.is_alive());
    std::thread::spawn(|| {
        // A process-global QApplication existing on the GUI thread is NOT
        // permission for a worker to allocate a QWidget or QObject there.
        expect_panic("GUI thread", || QWidget::new(None));
        expect_panic("GUI thread", DListView::new);
        expect_panic("GUI thread", DImageHandler::new);
        expect_panic("GUI thread", || QTimer::single_shot(0, || {}));
    })
    .join()
    .unwrap();
    widget.delete_later();
    QTimer::single_shot(10, DApplication::quit);
    assert_eq!(app.exec(), 0);
    assert!(!widget.is_alive());
    println!("no-application selfcheck ok");
}
