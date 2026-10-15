fn main() {
    // Headers only: dtk-sys already links Qt6/DTK6 through pkg-config.
    let mut includes = Vec::new();
    let mut defines = Vec::new();
    for pkg in ["dtk6widget", "dtk6gui", "dtk6core"] {
        let lib = pkg_config::Config::new()
            .cargo_metadata(false)
            .probe(pkg)
            .unwrap_or_else(|e| panic!("pkg-config {pkg}: {e}"));
        includes.extend(lib.include_paths);
        defines.extend(lib.defines);
    }
    let builder = cxx_qt_build::CxxQtBuilder::new()
        .file("src/bridge.rs")
        .qt_module("Widgets");
    // SAFETY: only adds DTK include paths/defines; no cc::Build state that
    // CXX-Qt relies on (standard, PIC, Qt flags) is altered.
    let builder = unsafe {
        builder.cc_builder(move |cc| {
            for include in &includes {
                cc.include(include);
            }
            for (name, value) in &defines {
                cc.define(name, value.as_deref());
            }
        })
    };
    builder.build();
}
