//! Painting and model APIs: Rust-defined list model, persistent indexes,
//! generated QModelIndex/QPainter/QStyle/QStyleOption methods and painting
//! onto a Rust-owned QImage.
//! QT_QPA_PLATFORM=offscreen cargo run --example paint_model_selfcheck
use dtk::widgets::{DListView, DStyle, DVariantListModel};
use dtk::{
    ColorExt, DApplication, ListModel, Painter, PersistentModelIndex, QColor, QImage, QRect,
    QStyleOption, QStyleOptionViewItem, QVariant, QWidget,
};
use dtk::{PaintDelegate, QTableWidget};
use std::cell::RefCell;
use std::rc::Rc;

const DISPLAY: i32 = 0;
const EDIT: i32 = 2;
const USER: i32 = 256;

fn main() {
    DApplication::with("dtk-paint-model-selfcheck", |app| {
        rust_list_model(app);
        generated_index_methods();
        painting_on_image();
        style_options();
        styled_delegate(app);
    });
    assert!(dtk_sys::callback_registry_is_empty());
    println!("paint model selfcheck ok");
}

#[derive(Debug)]
struct Song {
    title: String,
    plays: i64,
}

fn rust_list_model(app: &DApplication) {
    let owner = QWidget::new(None);
    let songs = vec![
        Song {
            title: "a".into(),
            plays: 1,
        },
        Song {
            title: "b".into(),
            plays: 2,
        },
    ];
    let list = ListModel::new_editable(
        &*owner,
        songs,
        |s: &Song, role| match role {
            DISPLAY | EDIT => Some(QVariant::from_string(&s.title)),
            USER => Some(QVariant::from_i64(s.plays)),
            _ => None,
        },
        |s: &mut Song, role, value| {
            if role != EDIT {
                return false;
            }
            s.title = value.to_string();
            true
        },
    );
    let model = list.as_model();
    assert_eq!((model.row_count(), model.column_count()), (2, 1));
    assert_eq!(model.data(1, 0, DISPLAY).to_string(), "b");
    assert_eq!(model.data(0, 0, USER).to_i64(), Some(1));
    assert!(!model.data(0, 0, 99).is_valid());
    assert!(!model.data(5, 0, DISPLAY).is_valid(), "out of range");

    // Generated typed signals on the Rust model's QAbstractItemModel view.
    let log = Rc::new(RefCell::new(Vec::new()));
    let sink = log.clone();
    let _ins = model.on_rows_inserted(move |_p: PersistentModelIndex, a, b| {
        sink.borrow_mut().push(("ins", a, b))
    });
    let sink = log.clone();
    let _rem = model.on_rows_removed(move |_p, a, b| sink.borrow_mut().push(("rem", a, b)));
    let sink = log.clone();
    let _chg = model.on_data_changed(move |top: PersistentModelIndex, _b, _roles| {
        sink.borrow_mut().push(("chg", top.row(), top.row()))
    });

    // A persistent index follows Rust-side inserts and removals.
    let b = model.index(1, 0, None);
    list.insert(
        0,
        Song {
            title: "z".into(),
            plays: 0,
        },
    );
    assert_eq!(b.row(), 2);
    assert_eq!(b.data(DISPLAY).to_string(), "b");
    list.push(Song {
        title: "c".into(),
        plays: 3,
    });
    assert_eq!(list.remove(0).unwrap().title, "z");
    assert!(list.remove(10).is_none());
    assert_eq!(b.row(), 1);

    // Editing through Qt (setData) mutates the Rust item.
    assert!(model.set_index_data(&b, &QVariant::from_string("B!"), EDIT));
    assert!(!model.set_index_data(&b, &QVariant::from_string("x"), DISPLAY));
    assert_eq!(list.with(1, |s| s.title.clone()).unwrap(), "B!");
    assert!(list.update(1, |s| s.plays += 10));
    assert_eq!(model.data(1, 0, USER).to_i64(), Some(12));
    assert_eq!(
        *log.borrow(),
        [
            ("ins", 0, 0),
            ("ins", 3, 3),
            ("rem", 0, 0),
            ("chg", 1, 1),
            ("chg", 1, 1)
        ]
    );

    // A view drives synchronous rowCount/data queries while painting.
    let view = DListView::new();
    view.set_model(&model);
    view.resize(200, 200);
    view.show();
    dtk::QTimer::single_shot(50, DApplication::quit);
    app.exec();
    assert_eq!(view.count(), 3);

    // A data hook that tries to enter Qt is contained: no data, no reentry.
    let probe = model.clone();
    let hostile = ListModel::new(&*owner, vec![0], move |_: &i32, _| {
        let _ = probe.row_count(); // rejected inside the synchronous borrow
        Some(QVariant::from_i64(7))
    });
    assert!(!hostile.as_model().data(0, 0, DISPLAY).is_valid());

    list.reset(vec![]);
    assert_eq!(model.row_count(), 0);
    assert!(!b.is_valid());
    owner.delete_later();
    dtk::QTimer::single_shot(0, DApplication::quit);
    app.exec();
    assert!(!model.is_alive(), "parent owns the Rust model's QObject");
}

/// Generated DVariantListModel methods take/return PersistentModelIndex.
fn generated_index_methods() {
    let owner = QWidget::new(None);
    let model = DVariantListModel::new();
    model.as_model().set_parent(&*owner);
    let root = model.as_model().index(-1, -1, None);
    assert!(!root.is_valid());
    assert!(model.insert_rows(0, 2, &root));
    assert_eq!(model.row_count(&root), 2);
    let first = model.as_model().index(0, 0, None);
    assert!(model.set_data(&first, &QVariant::from_string("x"), DISPLAY));
    assert_eq!(model.data(&first, DISPLAY).to_string(), "x");
}

fn painting_on_image() {
    let mut image = QImage::new(8, 8);
    image.fill(&QColor::from_rgb(0, 0, 0));
    Painter::on_image(&mut image, |p| {
        p.fill_rect(2, 2, 4, 4, &QColor::from_rgb(10, 200, 30));
    });
    assert_eq!(image.pixel_color(3, 3).rgba_u32(), 0xff0ac81e);
    assert_eq!(image.pixel_color(0, 0).rgba_u32(), 0xff000000);
}

/// QStyle/DStyle through borrowed views; owned options never keep widgets.
fn style_options() {
    let widget = QWidget::new(None);
    widget.resize(40, 20);
    let style = widget.style();
    let opt = QStyleOption::new();
    opt.init_from(&widget);
    opt.set_rect(&QRect::new(0, 0, 40, 20));
    assert_eq!(opt.rect().width(), 40);
    let copy = opt.clone();
    assert_eq!(copy.state(), opt.state());

    let mut image = QImage::new(40, 20);
    image.fill(&QColor::from_rgb(255, 255, 255));
    // PE_FrameFocusRect = 3 in QStyle::PrimitiveElement; drawn through the
    // generated static DStyle helper on any QStyle.
    Painter::on_image(&mut image, |p| {
        DStyle::draw_primitive(&style, 3, &opt, p, &widget)
    });
    if let Some(dstyle) = style.as_dstyle() {
        let _ = dstyle.pixel_metric_3(0, &opt, &widget);
        let _ = DStyle::get_state(&opt);
    }

    let item = QStyleOptionViewItem::new();
    item.set_text("row");
    item.set_rect(&QRect::new(0, 0, 10, 10));
    let owned = item.clone();
    drop(item);
    assert_eq!(owned.text(), "row");
    assert_eq!(owned.rect().height(), 10);
    drop(widget);
    // `opt` held no QObject pointer: still usable after the widget is gone.
    assert_eq!(opt.rect().height(), 20);
}

/// PaintDelegate::new_styled lends the initialized item option; style
/// queries and drawing are admitted inside the paint borrow, QObject
/// operations are not.
fn styled_delegate(app: &DApplication) {
    let table = QTableWidget::new(2, 1);
    table.set_cell_text(0, 0, "cell text");
    let seen = Rc::new(RefCell::new(Vec::new()));
    let sink = seen.clone();
    let view = table.as_widget();
    let delegate = PaintDelegate::new_styled(&*table, move |painter, option, index| {
        sink.borrow_mut().push((
            option.text(),
            option.rect().width() > 0,
            index.data_string(0),
        ));
        let copy = option.clone(); // owned, widget/index pointers cleared
        painter.fill_rect(
            copy.rect().x(),
            copy.rect().y(),
            1,
            1,
            &QColor::from_rgb(1, 2, 3),
        );
        // Entering Qt here is rejected (contained panic), never reentrant.
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| view.width())).is_err());
    });
    table.set_delegate_for_column(0, &delegate);
    table.resize(200, 100);
    table.show();
    dtk::QTimer::single_shot(80, DApplication::quit);
    app.exec();
    let seen = seen.borrow();
    assert!(!seen.is_empty(), "delegate painted");
    assert!(
        seen.iter()
            .any(|(text, has_rect, data)| text == "cell text" && *has_rect && data == "cell text")
    );
}
