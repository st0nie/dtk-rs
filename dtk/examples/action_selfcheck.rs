//! QAction/QMenu ownership and typed triggered(bool) delivery.
//! QT_QPA_PLATFORM=offscreen cargo run --example action_selfcheck
use dtk::{DApplication, DMenu, QAction, QTimer, QWidget};
use std::cell::Cell;
use std::rc::Rc;

fn main() {
    DApplication::with("dtk-action-selfcheck", |app| {
        DApplication::set_quit_on_last_window_closed(false);
        let parent = QWidget::new(None);
        let menu = DMenu::new(&parent);
        let action = QAction::new("Open", &menu);
        menu.add_action_object(&action);
        assert_eq!(action.text(), "Open");
        action.set_text("Reopen");
        assert_eq!(action.text(), "Reopen");
        action.set_enabled(false);
        assert!(!action.is_enabled());
        action.set_enabled(true);
        action.set_visible(true);
        assert!(action.is_visible());

        let hits = Rc::new(Cell::new(0));
        let observed = hits.clone();
        let marker = Rc::new(());
        let weak = Rc::downgrade(&marker);
        assert_ne!(
            action
                .on_triggered(move |checked| {
                    assert!(!checked);
                    let _ = &marker;
                    observed.set(observed.get() + 1);
                })
                .release(),
            0
        );
        action.trigger();
        assert_eq!(hits.get(), 1);
        let stale = action.clone();
        parent.delete_later();
        let cleaned = Rc::new(Cell::new(false));
        let result = cleaned.clone();
        QTimer::single_shot(50, move || {
            result.set(!stale.is_alive() && weak.upgrade().is_none());
            DApplication::quit();
        });
        assert_eq!(app.exec(), 0);
        assert!(
            cleaned.get(),
            "Qt action and Rust callback must die with menu parent"
        );
        println!("action selfcheck ok");
    });
}
