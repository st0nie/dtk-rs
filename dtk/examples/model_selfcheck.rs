//! QAbstractItemModel bridge, generated DListView::set_model, and Qt ownership.
//! QT_QPA_PLATFORM=offscreen cargo run --example model_selfcheck
use dtk::widgets::{DListView, DVariantListModel};
use dtk::{DApplication, QStandardItemModel, QTimer, QVariant, QVariantList, qt};
use std::cell::Cell;
use std::rc::Rc;

fn main() {
    DApplication::with("dtk-model-selfcheck", |app| {
        DApplication::set_quit_on_last_window_closed(false);
        let view = DListView::new();
        let model = DVariantListModel::new();
        let base = model.as_model();
        // QListView::setModel does not take ownership. Give the model a QObject
        // parent explicitly so deleting the view also destroys the model.
        base.set_parent(&view);
        view.set_model(&base);
        assert_eq!(base.row_count(), 0);
        assert_eq!(base.column_count(), 1);
        let missing = QVariant::from_string("missing");
        assert!(!base.data(0, 0, qt::item_role::DISPLAY_ROLE).is_valid());
        assert!(!base.set_data(0, 0, &missing, qt::item_role::EDIT_ROLE));

        let value = QVariant::from_string("model data");
        assert!(view.add_item(&value));
        assert_eq!(base.row_count(), 1);
        let read = base.data(0, 0, qt::item_role::DISPLAY_ROLE);
        assert!(read.is_valid());
        assert_eq!(read.to_string(), "model data");

        // The same DListView APIs must also work with a non-DTK Qt model.
        let custom_view = DListView::new();
        let standard = QStandardItemModel::new(&custom_view);
        let custom_base = standard.as_model();
        assert_eq!(custom_base.column_count(), 0);
        custom_view.set_model(&custom_base);
        let values: QVariantList = [QVariant::from_string("custom data")].into_iter().collect();
        assert!(
            !custom_view.add_items(&values),
            "zero-column models cannot store row data"
        );
        assert_eq!(custom_base.row_count(), 0);
        standard.set_column_count(1); // QListView addresses column 0.
        assert_eq!(custom_base.column_count(), 1);
        assert!(custom_view.add_items(&values));
        assert_eq!(custom_view.count(), 1);
        assert_eq!(
            custom_base
                .data(0, 0, qt::item_role::DISPLAY_ROLE)
                .to_string(),
            "custom data"
        );

        view.delete_later();
        custom_view.delete_later();
        let observed = Rc::new(Cell::new((false, false, false, false, false, false)));
        let result = observed.clone();
        QTimer::single_shot(50, move || {
            result.set((
                !view.is_alive(),
                !base.is_alive(),
                !model.is_alive(),
                !custom_view.is_alive(),
                !custom_base.is_alive(),
                !standard.is_alive(),
            ));
            DApplication::quit();
        });
        assert_eq!(app.exec(), 0);
        assert_eq!(observed.get(), (true, true, true, true, true, true));
        println!("model selfcheck ok");
    });
}
