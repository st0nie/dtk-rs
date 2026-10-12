//! Ownership handoff: an old constructor owner must not delete a Qt object
//! after Qt parents it and transfers it via `takeCentralWidget()`. DTK's
//! `takeHeaderWidget()` behaves differently: its result remains Qt-owned.
//! QT_QPA_PLATFORM=offscreen cargo run --example ownership_transfer_selfcheck
use dtk::widgets::DListView;
use dtk::{DApplication, DMainWindow, QTimer, QWidget};

fn main() {
    DApplication::with("dtk-ownership-transfer-selfcheck", |app| {
        DApplication::set_quit_on_last_window_closed(false);
        let window = DMainWindow::new();
        let original = QWidget::new(None);
        window.set_central_widget(&original);
        let taken = window
            .take_central_widget()
            .expect("central widget must be returned");
        let survivor = taken.clone();
        drop(original); // historical Owned must not delete what Qt already adopted

        let list_view = DListView::new();
        let header = QWidget::new(None);
        let index = list_view.add_header_widget(&header);
        assert!(index >= 0);
        let taken_header = list_view
            .take_header_widget(index)
            .expect("header widget must be returned");
        let header_view = taken_header.clone();
        assert!(header_view.is_alive());
        drop(header);

        let footer = QWidget::new(None);
        let footer_index = list_view.add_footer_widget(&footer);
        assert!(footer_index >= 0);
        let footer_view = list_view
            .take_footer_widget(footer_index)
            .expect("footer widget must be returned");
        drop(footer);
        QTimer::single_shot(20, move || {
            assert!(
                survivor.is_alive(),
                "original owner deleted transferred widget"
            );
            assert!(
                !header_view.is_alive(),
                "DTK takeHeaderWidget returns a borrowed view (native schedules deletion)"
            );
            assert!(
                !footer_view.is_alive(),
                "DTK takeFooterWidget returns a borrowed view (native schedules deletion)"
            );
            drop(taken_header);
            drop(taken); // current central-widget owner releases it
            QTimer::single_shot(20, move || {
                assert!(!survivor.is_alive(), "new owner must release widget");
                DApplication::quit();
            });
        });
        assert_eq!(app.exec(), 0);
        println!("ownership transfer selfcheck ok");
    });
}
