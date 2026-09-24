//! QTableWidget uses a real Qt QStringList for headers, not delimiter joining.
//! QT_QPA_PLATFORM=offscreen cargo run --example table_header_selfcheck
use dtk::{DApplication, QTableWidget, QTimer};

fn main() {
    let app = DApplication::new("dtk-table-header-selfcheck");
    DApplication::set_quit_on_last_window_closed(false);
    let table = QTableWidget::new(1, 3);
    let labels = ["App|Memory", "中文\0NUL", ""];
    table.set_horizontal_header_labels(&labels);
    for (i, label) in labels.into_iter().enumerate() {
        assert_eq!(table.horizontal_header_label(i as i32), label);
    }
    let dynamic = vec![
        String::from("plain"),
        String::from("a|b"),
        String::from("é"),
    ];
    table.set_horizontal_header_labels(&dynamic);
    for (i, label) in dynamic.into_iter().enumerate() {
        assert_eq!(table.horizontal_header_label(i as i32), label);
    }
    assert!(!table.set_cell_text(-1, 0, "invalid"));
    assert!(!table.set_cell_text(1, 0, "invalid"));
    assert!(!table.set_cell_data(0, 3, 99));
    assert!(table.item(0, 0).is_none());
    assert!(table.set_cell_data(0, 0, 42)); // missing cell creates a Qt-owned item
    assert_eq!(table.cell_data(0, 0), 42);
    assert!(table.set_cell_text(0, 0, "preserve UserRole"));
    assert_eq!(table.cell_data(0, 0), 42);
    assert_eq!(table.cell_text(0, 0), "preserve UserRole");
    table.delete_later();
    QTimer::single_shot(10, DApplication::quit);
    assert_eq!(app.exec(), 0);
    println!("table header selfcheck ok");
}
