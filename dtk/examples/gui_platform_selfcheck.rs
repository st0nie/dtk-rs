//! DGui coverage beyond the pilot: platform themes, native settings, the
//! window-manager helper and desktop services. Values depend on the platform
//! plugin, so this checks ownership/lifetime and typed signals, not themes.
//! QT_QPA_PLATFORM=offscreen cargo run --example gui_platform_selfcheck
use dtk::gui::{
    DDesktopServices, DGuiApplicationHelper, DPlatformTheme, DWindowManagerHelper,
    SystemSoundEffect,
};
use dtk::{DApplication, QPoint};

fn main() {
    let mut escaped: Option<DPlatformTheme> = None;
    DApplication::with("dtk-gui-platform-selfcheck", |_app| {
        let helper = DGuiApplicationHelper::instance().expect("helper singleton");
        let system = helper.system_theme().expect("system theme");
        let app_theme = helper.application_theme().expect("application theme");
        // DTK6: the application theme is the system theme unless a window
        // leader theme was created; both are helper-owned borrowed views.
        assert!(system.parent_theme().is_none());
        if let Some(parent) = app_theme.parent_theme() {
            assert_eq!(parent.theme_name(), system.theme_name());
        }
        let _ = (
            system.theme_name(),
            system.icon_theme_name(),
            system.font_point_size(),
        );
        let _ = (
            system.double_click_time(),
            system.window_radius_2(8),
            system.active_color(),
        );
        let _ = DGuiApplicationHelper::fetch_palette(&system);

        // DPlatformTheme -> DNativeSettings base view and its settings API.
        let native = system.as_d_native_settings();
        let _ = native.is_valid();
        let _ = native.get_setting("Net/ThemeName").is_valid();

        // Typed signals generated from DPlatformTheme/DNativeSettings headers.
        // (No setters here: on a real session the system theme writes the
        // desktop's XSETTINGS.)
        let _r = system.on_window_radius_changed(|_radius| {});
        let _t = system.on_theme_name_changed(|_name: Vec<u8>| {});
        let _a = system.on_active_color_changed(|_color: dtk::QColor| {});

        let wm = DWindowManagerHelper::instance().expect("Q_GLOBAL_STATIC helper");
        let _ = (
            wm.has_composite(),
            wm.has_blur_window(),
            wm.window_manager_name_string(),
        );
        let _ = wm.window_from_point(&QPoint::new(0, 0));
        let _c = wm.on_has_composite_changed(|| {});

        assert_eq!(
            DDesktopServices::get_name_by_effect_type(SystemSoundEffect::SseDeviceAdded),
            "device-added"
        );
        assert_eq!(
            DDesktopServices::get_name_by_effect_type(SystemSoundEffect::SseNotifications),
            "message"
        );
        escaped = Some(system);
    });
    // Helper-owned theme views expire with the scope even though the native
    // singletons may survive; access would panic rather than dereference.
    assert!(!escaped.as_ref().unwrap().is_alive());
    assert!(dtk_sys::callback_registry_is_empty());

    // Repeated scope: fresh borrowed views of the singletons.
    DApplication::with("dtk-gui-platform-selfcheck", |_app| {
        let helper = DGuiApplicationHelper::instance().unwrap();
        assert!(helper.system_theme().unwrap().is_alive());
        assert!(DWindowManagerHelper::instance().unwrap().is_alive());
    });
    println!("gui platform selfcheck ok");
}
