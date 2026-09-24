//! DCore configuration creation has an audited caller-owned result.
//! QT_QPA_PLATFORM=offscreen cargo run --example config_selfcheck
use dtk::core::DConfig;
use dtk::{DApplication, QTimer, QVariant};

fn main() {
    // Native DConfig requires setAppId before any QCoreApplication exists.
    // Calling it concurrently with another thread creating an app is unsafe.
    unsafe { DConfig::set_app_id("org.dtk.rs.selfcheck") };
    let app = DApplication::new("dtk-config-selfcheck");
    DApplication::set_quit_on_last_window_closed(false);
    let config = DConfig::create_generic("dtk-rs-nonexistent-meta", "")
        .expect("DCore allocated a configuration object");
    assert_eq!(config.name(), "dtk-rs-nonexistent-meta");
    assert!(!config.is_valid());
    assert!(config.key_list().is_empty());
    let fallback = QVariant::from_string("fallback");
    assert_eq!(
        config.value("absent-key", &fallback).to_string(),
        "fallback"
    );
    let view = config.clone();
    drop(config); // unique caller owner queues deletion
    let app_config = DConfig::create_for_app("org.dtk.rs.selfcheck", "dtk-rs-nonexistent-meta", "")
        .expect("DCore allocated an app-specific configuration");
    assert_eq!(app_config.name(), "dtk-rs-nonexistent-meta");
    let app_view = app_config.clone();
    drop(app_config);
    QTimer::single_shot(15, move || {
        assert!(!view.is_alive(), "generic config owner must release it");
        assert!(!app_view.is_alive(), "app config owner must release it");
        DApplication::quit();
    });
    assert_eq!(app.exec(), 0);
    println!("config selfcheck ok");
}
