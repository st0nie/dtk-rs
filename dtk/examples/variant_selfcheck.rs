//! QVariant value mapping and generated DTK tab-data roundtrip.
//! QT_QPA_PLATFORM=offscreen cargo run --example variant_selfcheck
use dtk::widgets::{DListView, DTabBar, DVariantListModel};
use dtk::{DApplication, QTimer, QVariant, QVariantList};

fn main() {
    DApplication::with("dtk-variant-selfcheck", |app| {
        DApplication::set_quit_on_last_window_closed(false);

        let empty = QVariant::new();
        assert!(!empty.is_valid());
        assert_eq!(empty.to_i64(), None);
        assert!(QVariant::from_bool(true).to_bool());
        assert!(QVariant::from_f64(2.5).is_valid());

        let number = QVariant::from_i64(4_294_967_300);
        let copy = number.clone();
        drop(number);
        assert_eq!(copy.to_i64(), Some(4_294_967_300));

        let text = "DTK 中文\0Rust";
        let tab = DTabBar::new();
        let index = tab.add_tab("test");
        assert_eq!(index, 0);
        let value = QVariant::from_string(text);
        tab.set_tab_data(index, &value);
        drop(value);
        assert_eq!(tab.tab_data(index).to_string(), text);
        tab.set_tab_data(index, &copy);
        assert_eq!(tab.tab_data(index).to_i64(), Some(4_294_967_300));

        let list: QVariantList = [QVariant::from_string("first"), QVariant::from_i64(42)]
            .into_iter()
            .collect();
        assert_eq!(list.len(), 2);
        assert_eq!(list.get(0).expect("first").to_string(), "first");
        assert_eq!(list.get(1).expect("second").to_i64(), Some(42));
        assert!(list.get(2).is_none());
        let copied_list = list.clone();
        drop(list);
        let view = DListView::new();
        // Native DListView dereferences a null model in addItems/count. Rust must
        // reject that precondition rather than crashing the process.
        assert_eq!(view.count(), 0);
        assert!(!view.add_items(&copied_list));
        let model = DVariantListModel::new();
        let base = model.as_model();
        base.set_parent(&view);
        view.set_model(&base);
        assert!(view.add_items(&copied_list));
        assert_eq!(view.count(), 2);

        view.delete_later();
        tab.delete_later();
        QTimer::single_shot(10, DApplication::quit);
        assert_eq!(app.exec(), 0);
        println!("variant selfcheck ok");
    });
}
