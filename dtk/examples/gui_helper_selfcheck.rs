//! DGui helper maps Qt colors/images through a checked QObject singleton.
//! QT_QPA_PLATFORM=offscreen cargo run --example gui_helper_selfcheck
use dtk::gui::{DGuiApplicationHelper, GuiColorType, GuiSizeMode};
use dtk::{DApplication, DPalette, QColor, QImage, QTimer};

fn main() {
    let app = DApplication::new("dtk-gui-helper-selfcheck");
    DApplication::set_quit_on_last_window_closed(false);
    let helper = DGuiApplicationHelper::instance().expect("DTK GUI helper exists");
    let mut palette = DPalette::new();
    DGuiApplicationHelper::generate_palette(&mut palette, GuiColorType::LightType);
    let title_color = palette.text_title().color().rgba_u32();
    let own_palette = helper.application_palette();
    assert_ne!(title_color, 0);
    assert_ne!(own_palette.text_title().color().rgba_u32(), 0);
    let standard = DGuiApplicationHelper::standard_palette(GuiColorType::DarkType);
    assert_ne!(standard.text_title().color().rgba_u32(), 0);
    helper.set_application_palette(&palette);
    drop((standard, own_palette, palette)); // helper retains native palette data
    assert_ne!(
        helper.application_palette().text_title().color().rgba_u32(),
        0
    );
    let color = QColor::rgb(87, 122, 193);
    let same = DGuiApplicationHelper::adjust_color(&color, 0, 0, 0, 0, 0, 0, 0);
    assert_eq!(same.rgba_u32(), color.rgba_u32());
    let mut pixels = QImage::new(2, 3);
    pixels.fill(&color);
    let adjusted = DGuiApplicationHelper::adjust_color_2(&pixels, 0, 0, 0, 0, 0, 0, 0);
    assert_eq!((adjusted.width(), adjusted.height()), (2, 3));
    assert_eq!(adjusted.pixel_color(1, 2).rgba_u32(), color.rgba_u32());
    let _ = helper.theme_type(); // native getter preserves its C++ enum value
    helper.set_palette_type(GuiColorType::LightType);
    let native_light =
        dtk_sys::gui_ffi::genffi::gen_enum_d_gui_application_helper_color_type_light_type();
    assert_eq!(helper.palette_type(), native_light);
    helper.set_size_mode(GuiSizeMode::CompactMode);
    let native_compact =
        dtk_sys::gui_ffi::genffi::gen_enum_d_gui_application_helper_size_mode_compact_mode();
    assert_eq!(helper.size_mode(), native_compact);
    helper.reset_size_mode();
    QTimer::single_shot(10, DApplication::quit);
    assert_eq!(app.exec(), 0);
    println!("gui helper selfcheck ok");
}
