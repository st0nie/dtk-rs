//! Qt value-type ownership and DTK method roundtrips (brush, path, shortcut, URL).
//! QT_QPA_PLATFORM=offscreen cargo run --example value_types_selfcheck
use dtk::ColorExt;
use dtk::widgets::{DCrumbTextFormat, DFileChooserEdit, DGraphicsClipEffect, DKeySequenceEdit};
use dtk::{DApplication, QBrush, QColor, QKeySequence, QPainterPath, QTimer, QUrl};

fn main() {
    // QColor/QBrush/QPainterPath/QUrl are Qt value types, usable without a GUI app.
    let color = QColor::from_rgb(30, 80, 150);
    let brush = QBrush::from_color(&color);
    assert_eq!(brush.color().rgba_u32(), color.rgba_u32());
    let mut path = QPainterPath::new();
    assert!(path.is_empty());
    path.add_rect(1.0, 2.0, 30.0, 40.0);
    assert!(path.element_count() > 0);
    let url = QUrl::from("file:///tmp");
    assert!(url.is_valid());
    assert_eq!(url.to_string(), "file:///tmp");
    let shortcut = QKeySequence::from_portable_text("Ctrl+Shift+K");
    assert_eq!(shortcut.to_string(), "Ctrl+Shift+K");

    DApplication::with("dtk-value-types-selfcheck", |app| {
        DApplication::set_quit_on_last_window_closed(false);
        let format = DCrumbTextFormat::new();
        format.set_background(&brush);
        let effect = DGraphicsClipEffect::new();
        effect.set_clip_path(&path);
        let file_edit = DFileChooserEdit::new();
        file_edit.set_directory_url(&url);
        let key_edit = DKeySequenceEdit::new();
        assert!(key_edit.set_key_sequence(&shortcut));
        drop((color, brush, path, url, shortcut));

        assert_eq!(
            format.background().color().rgba_u32(),
            QColor::from_rgb(30, 80, 150).rgba_u32()
        );
        assert!(!effect.clip_path().is_empty());
        assert_eq!(file_edit.directory_url().to_string(), "file:///tmp");
        assert_eq!(key_edit.key_sequence().to_string(), "Ctrl+Shift+K");

        effect.delete_later();
        file_edit.delete_later();
        key_edit.delete_later();
        QTimer::single_shot(10, DApplication::quit);
        assert_eq!(app.exec(), 0);
        println!("value types selfcheck ok");
    });
}
