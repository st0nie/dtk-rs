fn main() {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // run moc on relay.h (Q_OBJECT)
    let moc = ["/usr/lib/qt6/libexec/moc", "/usr/lib/qt6/bin/moc", "moc"]
        .iter()
        .find(|p| {
            std::process::Command::new(p)
                .arg("-v")
                .output()
                .is_ok_and(|o| o.status.success())
        })
        .expect("Qt6 moc not found");
    let moc_out = out_dir.join("moc_relay.cpp");
    let status = std::process::Command::new(moc)
        .args(["include/relay.h", "-o"])
        .arg(&moc_out)
        .status()
        .unwrap();
    assert!(status.success(), "moc failed");

    let mut build = cxx_build::bridges([
        "src/lib.rs",
        "src/gen_ffi.rs",
        "src/core_ffi.rs",
        "src/gui_ffi.rs",
    ]);
    build
        .file("cpp/shim.cpp")
        .file("cpp/application_runtime.cpp")
        .file("cpp/runtime_borrow.cpp")
        .file("cpp/object_relations.cpp")
        .file("cpp/anchor_relations.cpp")
        .file("cpp/runtime_worker_probe.cpp")
        .file("cpp/relay.cpp")
        .file("cpp/dtk_gen_shim.cpp")
        .file("cpp/dtk_core_shim.cpp")
        .file("cpp/dtk_gui_shim.cpp")
        .file(&moc_out)
        .include("include")
        .include("compat") // compat dir for alias headers missing from DTK6 packaging
        .include(cxx_qt_lib_include())
        .std("c++17")
        .flag_if_supported("-fPIC"); // Qt6 requires PIC

    // locate Qt6 + DTK6 via pkg-config; include paths go to cxx_build, link info is emitted by the pkg-config crate
    for pkg in [
        "Qt6Widgets",
        "Qt6Gui",
        "Qt6Core",
        "dtk6widget",
        "dtk6gui",
        "dtk6core",
    ] {
        let lib = pkg_config::Config::new()
            .probe(pkg)
            .unwrap_or_else(|e| panic!("pkg-config {pkg}: {e}"));
        for inc in &lib.include_paths {
            build.include(inc);
        }
        for (k, v) in &lib.defines {
            build.define(k, v.as_deref());
        }
    }

    build.compile("dtkshim");
    println!("cargo:rerun-if-env-changed=DEP_CXX_QT_LIB_CXX_QT_MANIFEST_PATH");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/gen_ffi.rs");
    println!("cargo:rerun-if-changed=cpp/shim.cpp");
    println!("cargo:rerun-if-changed=cpp/application_runtime.cpp");
    println!("cargo:rerun-if-changed=include/application_runtime.h");
    println!("cargo:rerun-if-changed=cpp/runtime_borrow.cpp");
    println!("cargo:rerun-if-changed=include/runtime_borrow.h");
    println!("cargo:rerun-if-changed=cpp/object_relations.cpp");
    println!("cargo:rerun-if-changed=include/object_relations.h");
    println!("cargo:rerun-if-changed=cpp/anchor_relations.cpp");
    println!("cargo:rerun-if-changed=include/anchor_relations.h");
    println!("cargo:rerun-if-changed=cpp/runtime_worker_probe.cpp");
    println!("cargo:rerun-if-changed=include/runtime_worker_probe.h");
    println!("cargo:rerun-if-changed=cpp/relay.cpp");
    println!("cargo:rerun-if-changed=cpp/dtk_gen_shim.cpp");
    println!("cargo:rerun-if-changed=include/dtk_shim.h");
    println!("cargo:rerun-if-changed=include/dtk_gen_shim.h");
    println!("cargo:rerun-if-changed=include/relay.h");
    println!("cargo:rerun-if-changed=compat"); // alias headers, recursive watch
}

/// cxx-qt-lib exports its C++ headers (`cxx-qt-lib/qcolor.h`, ... with the
/// `rust::IsRelocatable` specializations required for by-value types) next to
/// the manifest it announces through `links = "cxx-qt-lib"` metadata.
fn cxx_qt_lib_include() -> std::path::PathBuf {
    let manifest = std::env::var_os("DEP_CXX_QT_LIB_CXX_QT_MANIFEST_PATH")
        .expect("cxx-qt-lib did not export its CXX-Qt manifest");
    std::path::Path::new(&manifest)
        .parent()
        .expect("manifest directory")
        .join("include")
}
