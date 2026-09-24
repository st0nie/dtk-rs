//! DCore paths and system information work without QApplication.
//! cargo run --example core_paths_selfcheck
use dtk::core::{
    DStandardPaths, DSysInfo, DsgDirectory, LocateOption, StandardLocation, XdgDirectory,
};
use std::path::Path;

fn main() {
    let home = DStandardPaths::home_path();
    assert!(!home.is_empty());
    assert!(Path::new(&home).is_dir());
    let executable = DStandardPaths::find_executable("sh", vec!["/bin".to_owned()]);
    assert!(Path::new(&executable).is_file());
    let config = DStandardPaths::path(XdgDirectory::ConfigHome);
    assert!(Path::new(&config).is_absolute());
    let nested = DStandardPaths::file_path(XdgDirectory::ConfigHome, "settings.json");
    assert!(nested.starts_with(&config));
    assert!(nested.ends_with("settings.json"));
    for path in DStandardPaths::paths(DsgDirectory::DataDir) {
        assert!(Path::new(&path).is_absolute());
    }
    let temp = DStandardPaths::writable_location(StandardLocation::TempLocation);
    assert!(Path::new(&temp).is_dir());
    let file_name = format!("dtk-rs-core-test-{}", std::process::id());
    let temp_file = Path::new(&temp).join(&file_name);
    std::fs::write(&temp_file, b"test").expect("create test file");
    let located = DStandardPaths::locate(
        StandardLocation::TempLocation,
        &file_name,
        LocateOption::LocateFile,
    );
    assert!(Path::new(&located).is_file());
    std::fs::remove_file(&temp_file).expect("remove test file");
    assert!(!DSysInfo::operating_system_name().is_empty());
    assert!(DSysInfo::memory_total_size() > 0);
    assert!(!DSysInfo::computer_name().is_empty());
    println!("core paths selfcheck ok");
}
