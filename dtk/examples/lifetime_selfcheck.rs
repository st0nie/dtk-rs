//! Headless regression check for Qt-owned callback lifetimes and table item semantics.
//! Run with QT_QPA_PLATFORM=offscreen cargo run --example lifetime_selfcheck.
use dtk::widgets::{DGraphicsGlowEffect, DStackWidget};
use dtk::*;
#[path = "support/expected_panic.rs"]
mod expected_panic;
use expected_panic::expect_panic;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Captures = Rc<RefCell<Vec<Weak<()>>>>;

fn capture(captures: &Captures) -> Rc<()> {
    let value = Rc::new(());
    captures.borrow_mut().push(Rc::downgrade(&value));
    value
}

fn main() {
    DApplication::with("dtk-lifetime-selfcheck", |app| {
        DApplication::set_quit_on_last_window_closed(false);
        let captures: Captures = Rc::new(RefCell::new(Vec::new()));
        let marker = capture(&captures);
        expect_panic("only one Qt application", || {
            DApplication::with_quit_guard(
                "duplicate",
                move || {
                    let _ = &marker;
                    true
                },
                |_| {},
            )
        });
        let parent = QWidget::new(None);
        let stack = DStackWidget::new();
        assert!(
            stack.current_widget().is_none(),
            "empty stack must return None"
        );
        stack.delete_later();
        let glow = DGraphicsGlowEffect::new();
        let stale_glow = glow.clone();
        glow.delete_later();

        let button = DPushButton::new("test");
        button.as_widget().set_parent(&parent);
        let stale_parent = parent.clone();
        let stale_child = button.as_widget();
        let stale_button = button.clone();
        assert!(stale_parent.is_alive() && stale_child.is_alive());
        let marker = capture(&captures);
        assert_ne!(
            button.connect_signal("clicked(bool)", move || {
                let _ = &marker;
            }),
            0
        );
        let marker = capture(&captures);
        parent.add_shortcut("Ctrl+L", move || {
            let _ = &marker;
        });

        let menu = DMenu::new(&parent);
        let stale_menu = menu.clone();
        let marker = capture(&captures);
        let action_id = menu.add_action("item", move || {
            let _ = &marker;
        });
        assert_ne!(action_id, 0);

        let timer = QTimer::new(&parent);
        let marker = capture(&captures);
        assert_ne!(
            timer.on_timeout(move || {
                let _ = &marker;
            }),
            0
        );

        let marker = capture(&captures);
        let _paint = PaintWidget::new(Some(&parent), move |_| {
            let _ = &marker;
        });
        let marker = capture(&captures);
        let _delegate = PaintDelegate::new(&parent, move |_, _, _, _, _, _, _| {
            let _ = &marker;
        });

        let table = QTableWidget::new(1, 1);
        table.set_cell_text(0, 0, "old");
        table.set_cell_data(0, 0, 42);
        table.set_cell_text(0, 0, "new");
        assert_eq!(table.cell_text(0, 0), "new");
        assert_eq!(table.cell_data(0, 0), 42, "set_text must keep UserRole");
        let cell = table.item(0, 0).expect("cell has an item");
        let role = qt::item_role::USER_ROLE + 1;
        assert!(cell.set_data_string(role, "cell-data"));
        assert_eq!(cell.data_string(role).as_deref(), Some("cell-data"));
        table.set_row_count(0); // Qt destroys the QTableWidgetItem, but not TableCell.
        assert_eq!(cell.data_string(role), None);
        table.as_widget().set_parent(&parent);

        let marker_show = capture(&captures);
        let marker_close = capture(&captures);
        let window = DMainWindow::new_with_events(
            move || {
                let _ = &marker_show;
            },
            move || {
                let _ = &marker_close;
                true
            },
        );
        assert!(window.take_central_widget().is_none());
        let central = QWidget::new(None);
        window.set_central_widget(&central);
        let taken = window
            .take_central_widget()
            .expect("central widget must be returned");
        taken.set_parent(&parent); // takeCentralWidget transfers ownership to the caller
        window.delete_later();
        parent.delete_later();

        QTimer::single_shot(100, move || {
            assert!(!stale_parent.is_alive());
            assert!(!stale_child.is_alive());
            assert!(!stale_button.is_alive());
            assert!(!stale_glow.is_alive());
            let late_capture = Rc::new(());
            let late_weak = Rc::downgrade(&late_capture);
            expect_panic("DPushButton was destroyed", || {
                stale_button.connect_signal("clicked(bool)", move || {
                    let _ = &late_capture;
                })
            });
            assert!(
                late_weak.upgrade().is_none(),
                "dead sender must not register a closure"
            );
            let menu_capture = Rc::new(());
            let menu_weak = Rc::downgrade(&menu_capture);
            expect_panic("DMenu was destroyed", || {
                stale_menu.add_action("late", move || {
                    let _ = &menu_capture;
                })
            });
            assert!(
                menu_weak.upgrade().is_none(),
                "dead menu must not register an action"
            );
            expect_panic("QTableWidget was destroyed", || cell.data_string(role));
            expect_panic("DPushButton was destroyed", || stale_button.width());
            expect_panic("DGraphicsGlowEffect was destroyed", || {
                stale_glow.blur_radius()
            });
            expect_panic("QWidget was destroyed", || stale_child.width());
            assert!(
                captures
                    .borrow()
                    .iter()
                    .all(|weak| weak.upgrade().is_none()),
                "destroyed Qt owners must release every Rust closure"
            );
            println!("lifetime selfcheck ok");
            DApplication::quit();
        });
        assert_eq!(app.exec(), 0);
    });
}
