//! Generated typed signals: inherited Qt base-class signals and value-type
//! arguments (by-value cxx-qt-lib types, owned heap copies, persistent indexes).
//! QT_QPA_PLATFORM=offscreen cargo run --example signal_values_selfcheck
use dtk::widgets::{DBlurEffectWidget, DFontComboBox, DImageViewer, DSwitchButton};
use dtk::{
    ColorExt, DApplication, PersistentModelIndex, QColor, QFont, QIcon, QImage, QStandardItemModel,
    QVariant, QWidget,
};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    DApplication::with("dtk-signal-values-selfcheck", |app| {
        inherited_signals();
        by_value_color();
        heap_value_copies();
        persistent_model_index(app);
    });
    assert!(dtk_sys::callback_registry_is_empty());
    println!("signal values selfcheck ok");
}

type Log<T> = Rc<RefCell<Vec<T>>>;

fn log<T: 'static>() -> (Log<T>, Log<T>) {
    let log = Rc::new(RefCell::new(Vec::new()));
    (log.clone(), log)
}

/// QAbstractButton::toggled and QWidget::windowTitleChanged on a DTK subclass:
/// the connector casts with qobject_cast to the declaring Qt class.
fn inherited_signals() {
    let button = DSwitchButton::new();
    let (sink, seen) = log();
    let _toggled = button.on_toggled(move |on| sink.borrow_mut().push(on));
    button.set_checked(true);
    button.set_checked(false);
    assert_eq!(*seen.borrow(), [true, false]);

    let (sink, titles) = log();
    let _title = button.on_window_title_changed(move |t| sink.borrow_mut().push(t));
    button.set_window_title("标题 ✓");
    assert_eq!(*titles.borrow(), ["标题 ✓"]);
}

/// QColor crosses by value: the emission copies it into the queued payload.
fn by_value_color() {
    let blur = DBlurEffectWidget::new();
    let (sink, seen) = log();
    let _c = blur.on_mask_color_changed(move |c: QColor| sink.borrow_mut().push(c.rgba_u32()));
    blur.set_mask_color(&QColor::from_rgba(10, 20, 30, 40));
    assert_eq!(*seen.borrow(), [0x280a141e]);
}

/// QFont / QImage / QIcon arguments are C++ copies owned by the Rust value:
/// the sender's own storage can change afterwards without affecting them.
fn heap_value_copies() {
    let combo = DFontComboBox::new();
    let (sink, fonts) = log();
    let _f = combo.on_current_font_changed(move |font: QFont| sink.borrow_mut().push(font));
    let font = QFont::new();
    font.set_family("DejaVu Sans");
    combo.set_current_font(&font);
    assert!(
        !fonts.borrow().is_empty(),
        "currentFontChanged not delivered"
    );

    let viewer = DImageViewer::new();
    let (sink, images) = log();
    let _i = viewer.on_image_changed(move |image: QImage| sink.borrow_mut().push(image));
    let mut image = QImage::new(3, 2);
    image.fill(&QColor::from_rgb(1, 2, 3));
    viewer.set_image(&image);
    drop(viewer);
    let images = images.borrow();
    let last = images.last().expect("imageChanged not delivered");
    assert_eq!((last.width(), last.height()), (3, 2));
    assert_eq!(last.pixel_color(2, 1).rgba_u32(), 0xff010203);

    let (sink, icons) = log();
    let _w = combo.on_window_icon_changed(move |icon: QIcon| sink.borrow_mut().push(icon));
    combo.set_window_icon(&QIcon::from_theme("document-open"));
    assert_eq!(icons.borrow().len(), 1);
}

/// rowsInserted(const QModelIndex &, int, int) queues a QPersistentModelIndex;
/// dataChanged carries QList<int> roles as Vec<i32>. Persistent indexes
/// follow row moves and become invalid when their row is removed.
fn persistent_model_index(app: &DApplication) {
    let owner = QWidget::new(None);
    let standard = QStandardItemModel::new(&*owner);
    standard.set_column_count(1);
    let model = standard.as_model();
    let (sink, inserted) = log();
    let _r = standard.on_rows_inserted(move |parent: PersistentModelIndex, first, last| {
        sink.borrow_mut().push((parent.is_valid(), first, last))
    });
    assert!(model.insert_rows(0, 2));
    assert_eq!(*inserted.borrow(), [(false, 0, 1)]);

    let (sink, changed) = log();
    let _d = standard.on_data_changed(move |top: PersistentModelIndex, bottom, roles| {
        sink.borrow_mut()
            .push((top.row(), bottom.row(), top.data(0).to_string(), roles))
    });
    let row1 = model.index(1, 0, None);
    assert!(row1.is_valid());
    assert!(model.set_index_data(&row1, &QVariant::from_string("row one"), 0));
    {
        let changed = changed.borrow();
        assert_eq!(changed.len(), 1);
        let (top, bottom, text, roles) = &changed[0];
        assert_eq!((*top, *bottom, text.as_str()), (1, 1, "row one"));
        assert!(roles.contains(&0), "DisplayRole in {roles:?}");
    }

    assert!(model.insert_rows(0, 1));
    assert_eq!(row1.row(), 2, "persistent index follows the inserted row");
    assert_eq!(row1.data(0).to_string(), "row one");
    let copy = row1.clone();
    assert!(model.remove_rows(2, 1));
    assert!(!row1.is_valid() && !copy.is_valid());
    let foreign = QStandardItemModel::new(&*owner);
    foreign.set_column_count(1);
    assert!(foreign.as_model().insert_rows(0, 1));
    let foreign_index = foreign.as_model().index(0, 0, None);
    assert!(!model.set_index_data(&foreign_index, &QVariant::from_i64(1), 0));
    assert!(!model.index(99, 0, None).is_valid());
    // Model destruction (with its owner) invalidates, never dangles, a held index.
    let held = foreign.as_model().index(0, 0, None);
    assert!(held.is_valid());
    owner.delete_later();
    dtk::QTimer::single_shot(0, DApplication::quit);
    app.exec();
    assert!(!foreign.is_alive());
    assert!(!held.is_valid() && held.row() == -1);
    drop(held); // destructor after model deletion: no access to the freed model
}
