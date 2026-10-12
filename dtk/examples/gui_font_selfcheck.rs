//! DGui font manager uses a guarded QObject and owned QFont value copies.
//! QT_QPA_PLATFORM=offscreen cargo run --example gui_font_selfcheck
use dtk::gui::{DFontManager, DGuiApplicationHelper, FontSizeType};
use dtk::{DApplication, QFont, QTimer};

fn main() {
    DApplication::with("dtk-gui-font-selfcheck", |app| {
        DApplication::set_quit_on_last_window_closed(false);
        let helper = DGuiApplicationHelper::instance().expect("DTK GUI helper exists");
        let native_manager = helper
            .font_manager()
            .expect("DTK helper font manager exists");
        assert!(native_manager.font_pixel_size(FontSizeType::T6) > 0);
        let manager = DFontManager::new();
        let view = manager.clone();
        let base = QFont::new();
        manager.set_base_font(&base);
        // The native getter adds a base-font pixel-size offset. Reset before
        // checking the per-size setter's exact roundtrip.
        manager.reset_base_font();
        let size = manager.font_pixel_size(FontSizeType::T6);
        assert!(size > 0, "DTK must return a usable pixel size");
        manager.set_font_pixel_size(FontSizeType::T6, size + 1);
        assert_eq!(manager.font_pixel_size(FontSizeType::T6), size + 1);
        assert!(manager.t6().metrics().1 > 0);
        manager.delete_later();
        QTimer::single_shot(10, DApplication::quit);
        assert_eq!(app.exec(), 0);
        assert!(!view.is_alive());
        println!("gui font selfcheck ok");
    });
}
