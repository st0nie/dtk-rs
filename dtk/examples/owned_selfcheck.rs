//! `Owned<T>` lifetime: a freshly constructed top-level object deletes itself
//! when dropped unparented, but never when Qt has since taken ownership.
//! QT_QPA_PLATFORM=offscreen cargo run --example owned_selfcheck
use dtk::widgets::DListView;
use dtk::{
    DApplication, DMainWindow, DMessageBox, DPushButton, Owned, QTimer, QVBoxLayout, QWidget,
};
use std::cell::Cell;
use std::rc::Rc;

fn main() {
    DApplication::with("dtk-owned-selfcheck", |app| {
        DApplication::set_quit_on_last_window_closed(false);

        // Case 1: constructed, never parented, never explicitly deleted — Owned<T>
        // must ask Qt to delete it once dropped, with no help from the caller.
        let forgotten = {
            let widget = QWidget::new(None);
            let view = widget.clone(); // plain, non-owning view for the later check
            drop(widget); // Owned<QWidget> drops here, unparented -> schedules delete
            view
        };

        // Case 2: constructed, then parented into a layout on a live widget —
        // dropping the Owned<T> handle immediately must be a safe no-op.
        let central = QWidget::new(None);
        let vbox = QVBoxLayout::new(Some(&central));
        let kept = {
            let button = DPushButton::new("kept");
            vbox.add_widget(&button.as_widget());
            let view = button.clone();
            drop(button); // Owned<DPushButton> drops here, but Qt now owns it
            view
        };

        // Case 3: `take_central_widget()` hands ownership back; if the caller
        // does not reparent it, the returned `Owned<QWidget>` must clean it up.
        // A dedicated window avoids `setCentralWidget`'s "replaces (deletes) the
        // previous central widget" semantics interacting with the other cases.
        let taken_win = DMainWindow::new();
        taken_win.set_central_widget(&QWidget::new(None));
        let taken_view = {
            let taken: Owned<QWidget> = taken_win
                .take_central_widget()
                .expect("central widget was set");
            let view = taken.clone();
            drop(taken); // never reparented -> schedules delete
            view
        };
        taken_win.delete_later();

        // Case 4: Default::default() for Owned<T> must behave exactly like new().
        let default_box: Owned<DMessageBox> = Default::default();
        let default_view = default_box.clone();
        drop(default_box); // unparented default -> schedules delete

        // Case 5: `Owned<T>` itself satisfies the same generic bounds (`Signal0`,
        // `AsLayout`) as the plain view, without an explicit deref at the call site.
        let list_view = DListView::new();
        let timer = QTimer::new(&list_view); // &Owned<DListView> as &impl Signal0
        let row = QVBoxLayout::new(None);
        vbox.add_layout(&row); // &Owned<QVBoxLayout> as &impl AsLayout
        assert!(row.as_layout().is_alive());
        timer.stop();

        let win = DMainWindow::new();
        win.set_central_widget(&central);
        win.show();

        let observed = Rc::new(Cell::new((false, false, false, false, false, false)));
        let result = observed.clone();
        QTimer::single_shot(50, move || {
            result.set((
                !forgotten.is_alive(),
                kept.is_alive(),
                !taken_view.is_alive(),
                !default_view.is_alive(),
                list_view.is_alive(),
                row.as_layout().is_alive(),
            ));
            DApplication::quit();
        });
        assert_eq!(app.exec(), 0);
        let (
            forgotten_deleted,
            kept_alive,
            taken_deleted,
            default_deleted,
            list_view_alive,
            row_alive,
        ) = observed.get();
        assert!(
            forgotten_deleted,
            "unparented Owned<T> must self-delete when dropped"
        );
        assert!(
            kept_alive,
            "Owned<T> must not delete an object Qt has since parented"
        );
        assert!(
            taken_deleted,
            "take_central_widget() result must self-delete if never reparented"
        );
        assert!(
            default_deleted,
            "Default::default() must behave exactly like new()"
        );
        assert!(
            list_view_alive,
            "list_view is kept alive by this scope, unaffected by Owned<QTimer>'s drop"
        );
        assert!(
            row_alive,
            "row is kept alive by this scope, unaffected by nested add_layout"
        );
        println!("owned selfcheck ok");
    });
}
