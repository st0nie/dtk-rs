//! DCore coverage beyond the pilot: DSettings (JSON + QSettings file backend),
//! file watchers, DLogManager, DTrashManager and DSGApplication, all headless
//! (no display needed).
//! env -u DISPLAY -u WAYLAND_DISPLAY -u QT_QPA_PLATFORM cargo run --example core_extras_selfcheck
use dtk::core::{
    DFileSystemWatcher, DFileWatcher, DFileWatcherManager, DLogManager, DSGApplication, DSettings,
    DTrashManager,
};
use dtk::{QCoreApplication, QTimer, QVariant};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

const JSON: &str = r#"{"groups": [{"key": "base", "name": "Base", "groups": [
    {"key": "font", "name": "Font", "options": [
        {"key": "size", "type": "spinbutton", "name": "Size", "default": 12},
        {"key": "family", "type": "lineedit", "name": "Family", "default": "Noto Sans"}
    ]}]}]}"#;

fn main() {
    let dir = std::env::temp_dir().join(format!("dtk-rs-core-extras-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let ini = dir.join("settings.conf");
    let ini = ini.to_str().unwrap().to_owned();

    QCoreApplication::with("dtk-core-extras-selfcheck", |app| {
        settings_values_and_signal();
        // Persist through the native QSettings backend on its worker thread.
        let settings = DSettings::from_json(JSON);
        assert!(settings.set_file_backend(&ini));
        assert!(!settings.set_file_backend(&ini), "second backend rejected");
        settings.set_option("base.font.size", &QVariant::from_i64(20));
        settings.sync();
        drop(settings); // deleteLater: worker joined, backend deleted on its thread
        run_until(app, || false, Duration::from_millis(50));

        watchers(app, &dir);
        log_manager(&dir);
        trash_and_app_id();
    });
    // A fresh scope reads the persisted value back through a new backend.
    QCoreApplication::with("dtk-core-extras-selfcheck", |app| {
        // An owned DFileSystemWatcher works in any scope. DTK's process-global
        // DFileWatcher backend stays bound to the first scope (documented).
        owned_watcher_only(app, &dir);
        let settings = DSettings::from_json(JSON);
        assert_eq!(
            settings.value("base.font.size").to_i64(),
            Some(12),
            "default before backend"
        );
        assert!(settings.set_file_backend(&ini));
        assert_eq!(
            settings.value("base.font.size").to_i64(),
            Some(20),
            "loaded from file"
        );
    });
    assert!(dtk_sys::callback_registry_is_empty());
    std::fs::remove_dir_all(&dir).ok();
    println!("core extras selfcheck ok");
}

fn run_until(app: &QCoreApplication, mut done: impl FnMut() -> bool, limit: Duration) {
    let start = Instant::now();
    while !done() && start.elapsed() < limit {
        QTimer::single_shot(10, QCoreApplication::quit);
        app.exec();
    }
}

fn settings_values_and_signal() {
    let settings = DSettings::from_json(JSON);
    let mut keys = settings.keys();
    keys.sort();
    assert_eq!(keys, ["base.font.family", "base.font.size"]);
    assert_eq!(settings.group_keys(), ["base"]);
    assert_eq!(settings.value("base.font.family").to_string(), "Noto Sans");
    assert!(!settings.value("missing").is_valid());

    let seen = Rc::new(RefCell::new(Vec::new()));
    let sink = seen.clone();
    let _c = settings.on_value_changed(move |key, value: QVariant| {
        sink.borrow_mut().push((key, value.to_i64()))
    });
    settings.set_option("base.font.size", &QVariant::from_i64(14));
    assert_eq!(*seen.borrow(), [("base.font.size".to_owned(), Some(14))]);
    // Native setOption dereferences a null option for unknown keys: rejected.
    settings.set_option("no.such.key", &QVariant::from_i64(1));
    settings.reset();
    assert_eq!(settings.get_option("base.font.size").to_i64(), Some(12));
    // An unreadable file gives empty settings rather than an error.
    assert!(
        DSettings::from_json_file("/nonexistent/settings.json")
            .keys()
            .is_empty()
    );
}

fn watchers(app: &QCoreApplication, dir: &std::path::Path) {
    let file = dir.join("watched.txt");
    std::fs::write(&file, "a").unwrap();
    let path = file.to_str().unwrap();

    // Owned inotify watcher with QPrivateSignal signals.
    let fs = DFileSystemWatcher::new();
    assert!(fs.add_path(path));
    assert_eq!(fs.files(), [path]);
    let modified = Rc::new(RefCell::new(Vec::new()));
    let sink = modified.clone();
    let _m = fs.on_file_modified(move |p, name| sink.borrow_mut().push((p, name)));

    // DFileWatcher (QUrl signals) and the manager's string signals.
    let watcher = DFileWatcher::new(path);
    let base = watcher.as_d_base_file_watcher();
    assert_eq!(
        base.file_url().to_local_file().map(|s| s.to_string()),
        Some(path.to_owned())
    );
    assert!(base.start_watcher());
    let urls = Rc::new(RefCell::new(Vec::new()));
    let sink = urls.clone();
    let _u = watcher.on_file_modified(move |url| sink.borrow_mut().push(url));
    let manager = DFileWatcherManager::new();
    let child = manager.add(path).expect("manager creates a child watcher");
    assert!(child.is_alive());
    assert_eq!(manager.watched_files(), [path]);
    let managed = Rc::new(RefCell::new(Vec::new()));
    let sink = managed.clone();
    let _w = manager.on_file_modified(move |p| sink.borrow_mut().push(p));

    std::fs::write(&file, "changed").unwrap();
    run_until(
        app,
        || {
            !modified.borrow().is_empty()
                && !urls.borrow().is_empty()
                && !managed.borrow().is_empty()
        },
        Duration::from_secs(5),
    );
    assert!(
        !modified.borrow().is_empty(),
        "DFileSystemWatcher::fileModified"
    );
    assert!(!urls.borrow().is_empty(), "DFileWatcher::fileModified");
    assert_eq!(
        managed.borrow()[0],
        path,
        "DFileWatcherManager::fileModified"
    );

    // remove(): native deleteLater of the borrowed child; its view dies.
    manager.remove(path);
    run_until(app, || !child.is_alive(), Duration::from_secs(1));
    assert!(!child.is_alive());
    assert!(manager.watched_files().is_empty());
    assert!(base.stop_watcher());
}

fn owned_watcher_only(app: &QCoreApplication, dir: &std::path::Path) {
    let file = dir.join("second.txt");
    std::fs::write(&file, "a").unwrap();
    let fs = DFileSystemWatcher::new();
    assert!(fs.add_path(file.to_str().unwrap()));
    let modified = Rc::new(RefCell::new(0));
    let sink = modified.clone();
    let _m = fs.on_file_modified(move |_, _| *sink.borrow_mut() += 1);
    std::fs::write(&file, "b").unwrap();
    run_until(app, || *modified.borrow() > 0, Duration::from_secs(5));
    assert!(
        *modified.borrow() > 0,
        "owned DFileSystemWatcher in a later scope"
    );
}

fn log_manager(dir: &std::path::Path) {
    let log = dir.join("app.log");
    DLogManager::setlog_file_path(log.to_str().unwrap());
    assert_eq!(DLogManager::getlog_file_path(), log.to_str().unwrap());
    DLogManager::set_log_format("%{message}\n");
}

/// DTrashManager::instance() is a borrowed Q_GLOBAL_STATIC view; repeated
/// calls see the same object. Only read-only queries run here: moving a
/// file would write to the user's real trash. DSGApplication::id() falls
/// back to the application name when no DSG id is configured.
fn trash_and_app_id() {
    let trash = DTrashManager::instance().expect("process-wide trash manager");
    let again = DTrashManager::instance().unwrap();
    assert!(trash.is_alive() && again.is_alive());
    let _ = trash.trash_is_empty();
    assert!(!trash.move_to_trash("/nonexistent/dtk-rs-selfcheck", false));
    let id = DSGApplication::id();
    assert!(!id.is_empty(), "fallback app id");
}
