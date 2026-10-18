//! Generated typed signals with pointer, item, list and QFlags arguments.
//! QObject pointers arrive as guarded views (or `None`), table/standard
//! items as positions/persistent indexes, index lists as owned vectors.
//! QT_QPA_PLATFORM=offscreen cargo run --example signal_pointer_selfcheck
use dtk::widgets::DStackWidget;
use dtk::{
    DApplication, DMenu, PersistentModelIndex, QAction, QStandardItemModel, QTableWidget, QVariant,
    QWidget,
};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    DApplication::with("dtk-signal-pointer-selfcheck", |app| {
        action_pointer(app);
        widget_pointer_and_null();
        table_items();
        standard_item_and_index_list();
    });
    assert!(dtk_sys::callback_registry_is_empty());
    println!("signal pointer selfcheck ok");
}

type Log<T> = Rc<RefCell<Vec<T>>>;

fn log<T: 'static>() -> (Log<T>, Log<T>) {
    let log = Rc::new(RefCell::new(Vec::new()));
    (log.clone(), log)
}

/// QMenu::triggered(QAction *): the closure gets a guarded QAction view that
/// shares nothing with the sender and goes dead when Qt deletes the action.
fn action_pointer(app: &DApplication) {
    let owner = QWidget::new(None);
    let menu = DMenu::new(&owner);
    let action = QAction::new("打开 ✓", &*menu);
    menu.add_action_object(&action);
    let (sink, seen) = log::<Option<QAction>>();
    let _t = menu.on_triggered(move |a| sink.borrow_mut().push(a));
    action.trigger();
    let delivered = seen.borrow_mut().pop().expect("triggered not delivered");
    let delivered = delivered.expect("live action must arrive as Some");
    assert!(delivered.is_alive());
    assert_eq!(delivered.text(), "打开 ✓");
    delivered.set_text("renamed");
    assert_eq!(
        action.text(),
        "renamed",
        "view refers to the emitted action"
    );

    // Qt deletes the action through its parent menu (and the menu through
    // its owner): the delivered view's own guard observes it; never dangles.
    owner.delete_later();
    dtk::QTimer::single_shot(0, DApplication::quit);
    app.exec();
    assert!(!action.is_alive());
    assert!(!delivered.is_alive());
}

/// DStackWidget::currentWidgetChanged(QWidget *): a page arrives as a
/// QWidget view; `clear()` deletes every page with qDeleteAll and then emits
/// nullptr, delivered as `None`. The earlier view observes the deletion.
fn widget_pointer_and_null() {
    let stack = DStackWidget::new();
    let page = QWidget::new(None);
    let (sink, seen) = log::<Option<QWidget>>();
    let _c = stack.on_current_widget_changed(move |w| sink.borrow_mut().push(w));
    stack.push_widget(&page, false);
    {
        let seen = seen.borrow();
        let first = seen[0].as_ref().expect("pushed page");
        assert!(first.is_alive());
        assert!(first.width() >= 0, "view is usable while the page lives");
    }
    stack.clear();
    let seen = seen.borrow();
    assert_eq!(seen.len(), 2, "{seen:?}");
    assert!(
        !seen[0].as_ref().unwrap().is_alive(),
        "qDeleteAll invalidates the view"
    );
    assert!(!page.is_alive());
    assert!(seen[1].is_none(), "cleared stack emits nullptr");
}

/// QTableWidget item signals never expose QTableWidgetItem*: the emission
/// converts the item to its (row, column).
fn table_items() {
    let table = QTableWidget::new(3, 2);
    let (sink, changed) = log();
    let _c = table.on_item_changed(move |cell| sink.borrow_mut().push(cell));
    assert!(table.set_cell_text(2, 1, "x"));
    assert!(table.set_cell_text(2, 1, "y"));
    assert!(table.set_cell_text(0, 0, "z"));
    let changed = changed.borrow();
    assert!(changed.contains(&Some((2, 1))), "{changed:?}");
    assert_eq!(changed.last(), Some(&Some((0, 0))), "{changed:?}");
}

/// QStandardItemModel::itemChanged(QStandardItem *) arrives as a persistent
/// index; layoutChanged(const QList<QPersistentModelIndex> &, hint) as an
/// owned Vec of persistent indexes.
fn standard_item_and_index_list() {
    let owner = QWidget::new(None);
    let standard = QStandardItemModel::new(&*owner);
    standard.set_column_count(1);
    let model = standard.as_model();
    assert!(model.insert_rows(0, 3));
    for (row, text) in ["b", "c", "a"].into_iter().enumerate() {
        assert!(model.set_data(row as i32, 0, &QVariant::from_string(text), 0));
    }

    let (sink, items) = log::<Option<PersistentModelIndex>>();
    let _i = standard.on_item_changed(move |index| sink.borrow_mut().push(index));
    let row1 = model.index(1, 0, None);
    assert!(model.set_index_data(&row1, &QVariant::from_string("changed"), 0));
    {
        let items = items.borrow();
        let index = items.last().and_then(Option::as_ref).expect("itemChanged");
        assert_eq!(
            (index.row(), index.data(0).to_string()),
            (1, "changed".to_owned())
        );
    }

    let (sink, layouts) = log::<(Vec<PersistentModelIndex>, i32)>();
    let _l =
        standard.on_layout_changed(move |parents, hint| sink.borrow_mut().push((parents, hint)));
    model.sort(0, false);
    let layouts = layouts.borrow();
    let (parents, _hint) = layouts.last().expect("sort emits layoutChanged");
    // A flat model sorts the invisible root: an empty list or one invalid
    // parent, depending on the Qt version.
    assert!(parents.iter().all(|p| !p.is_valid()));
    let texts: Vec<_> = (0..3).map(|r| model.data(r, 0, 0).to_string()).collect();
    assert_eq!(texts, ["a", "b", "changed"]);
}
