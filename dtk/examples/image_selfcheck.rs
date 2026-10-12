//! Owned QImage mapping through DTK image viewer and watermark APIs.
//! QT_QPA_PLATFORM=offscreen cargo run --example image_selfcheck
use dtk::widgets::{DImageViewer, DWaterMarkHelper, DWaterMarkWidget, WaterMarkData};
use dtk::{DApplication, QColor, QImage, QTimer};

fn main() {
    // QImage is an ordinary QtGui value, unlike QPixmap: no QApplication needed.
    let mut pixels = QImage::new(2, 3);
    assert!(!pixels.is_null());
    assert_eq!((pixels.width(), pixels.height()), (2, 3));
    let color = QColor::rgba(34, 87, 205, 128);
    pixels.fill(&color);
    pixels.set_text("Comment", "Rust|中文\0tail");
    assert_eq!(pixels.text("Comment"), "Rust|中文\0tail");
    assert_eq!(pixels.pixel_color(1, 2).rgba_u32(), color.rgba_u32());
    assert!(QImage::from_file("/this/file/does/not/exist.png").is_null());

    DApplication::with("dtk-image-selfcheck", |app| {
        DApplication::set_quit_on_last_window_closed(false);
        let viewer = DImageViewer::new();
        let watermark = DWaterMarkWidget::new();
        viewer.set_image(&pixels);
        let watermark_data = WaterMarkData::new();
        watermark_data.set_image(&pixels);
        watermark.set_data(&watermark_data);
        let helper = DWaterMarkHelper::instance().expect("DTK watermark helper exists");
        helper.set_data(&watermark_data);
        drop(watermark_data);
        drop(pixels); // Qt holds its own implicitly shared value copy
        let from_viewer = viewer.image();
        assert_eq!((from_viewer.width(), from_viewer.height()), (2, 3));
        assert_eq!(from_viewer.pixel_color(1, 2).rgba_u32(), color.rgba_u32());
        let from_watermark = watermark.data().image();
        assert_eq!((from_watermark.width(), from_watermark.height()), (2, 3));
        assert_eq!(
            from_watermark.pixel_color(1, 2).rgba_u32(),
            color.rgba_u32()
        );
        assert_eq!(
            helper.data().image().pixel_color(1, 2).rgba_u32(),
            color.rgba_u32()
        );

        viewer.delete_later();
        watermark.delete_later();
        QTimer::single_shot(10, DApplication::quit);
        assert_eq!(app.exec(), 0);
        println!("image selfcheck ok");
    });
}
