//! CXX-Qt integration layer for dtk-rs.
//!
//! This crate only imports existing Qt/DTK QObject classes (`extern "C++Qt"`).
//! It never constructs, owns, deletes or mutably borrows them: the safe
//! `dtk` crate supplies live, guarded QObject pointers inside a
//! `dtk_sys::NativeCall`. Typed signal functors own a
//! [`dtk_sys::native_signal::SignalToken`] and queue owned argument values
//! into the shared dtk-sys callback runtime.
//!
//! `bridge.rs`, `connectors.rs` and `include/dtk_cxxqt_casts.h` are generated
//! by `tools/signals.py` (run through `tools/gen.py`); do not edit them.

// Generated unsafe casts share one contract: `object` is null or a live
// QObject on its owner thread (see the connectors module header).
#[allow(clippy::missing_safety_doc)]
mod bridge;
pub use bridge::ffi;
pub mod connectors;
pub use cxx_qt_lib::QString;
