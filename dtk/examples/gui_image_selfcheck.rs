//! DGui image transforms and guarded, owned DImageHandler lifetime.
//! QT_QPA_PLATFORM=offscreen cargo run --example gui_image_selfcheck
use dtk::ColorExt;
use dtk::gui::DImageHandler;
use dtk::{DApplication, QColor, QImage, QTimer};

fn main() {
    DApplication::with("dtk-gui-image-selfcheck", |app| {
        DApplication::set_quit_on_last_window_closed(false);
        let handler = DImageHandler::new();
        let mut image = QImage::new(3, 2);
        image.fill(&QColor::from_rgb(200, 50, 20));
        let gray = DImageHandler::gray_scale_color_filter(&image);
        assert_eq!((gray.width(), gray.height()), (3, 2));
        let argb = gray.pixel_color(1, 1).rgba_u32();
        assert_eq!((argb >> 16) & 0xff, (argb >> 8) & 0xff);
        assert_eq!((argb >> 8) & 0xff, argb & 0xff);
        assert!(handler.rotate_image(&mut image, 90));
        assert_eq!((image.width(), image.height()), (2, 3));
        image.set_text("Comment", "Rust DTK metadata");
        let file =
            std::env::temp_dir().join(format!("dtk-image-metadata-{}.png", std::process::id()));
        let file_name = file.to_str().expect("UTF-8 temporary path");
        assert!(image.save(file_name));
        handler.set_file_name(file_name);
        assert!(!handler.read_image().is_null());
        // DTK metadata depends on its optional FreeImage backend; Qt PNG text
        // is not necessarily included in this map. Either way the map is owned.
        let _metadata = handler.find_all_meta_data();
        assert_eq!(
            dtk::QImage::from_file(file_name).text("Comment"),
            "Rust DTK metadata"
        );
        std::fs::remove_file(&file).expect("remove image test file");
        handler.set_file_name("/this/file/does/not/exist.png");
        // DTK isReadable() reports whether the image format is supported, not
        // necessarily whether this specific file exists. readImage() is null.
        assert!(handler.read_image().is_null());
        handler.delete_later();
        QTimer::single_shot(10, DApplication::quit);
        assert_eq!(app.exec(), 0);
        println!("gui image selfcheck ok");
    });
}
