//! Verify generated non-QObject constructors own and destroy their C++ storage.
//! QT_QPA_PLATFORM=offscreen cargo run --example opaque_lifetime_selfcheck
use dtk::widgets::{DFileIconProvider, DPrinter, DStyleOptionButton};
use dtk::{DApplication, OwnedOpaque};

fn main() {
    let _app = DApplication::new("dtk-opaque-lifetime-selfcheck");
    assert!(std::mem::needs_drop::<OwnedOpaque<DStyleOptionButton>>());
    assert!(std::mem::needs_drop::<OwnedOpaque<DFileIconProvider>>());
    assert!(std::mem::needs_drop::<OwnedOpaque<DPrinter>>());

    for _ in 0..64 {
        let style = DStyleOptionButton::new();
        let provider = DFileIconProvider::new();
        let printer = DPrinter::new();
        // Deref gives methods their normal borrowed DTK receiver without
        // duplicating the unique owner or freeing borrowed singleton pointers.
        let _ = &*style;
        let _ = &*provider;
        let _ = &*printer;
        drop((style, provider, printer));
    }
    println!("opaque lifetime selfcheck ok");
}
