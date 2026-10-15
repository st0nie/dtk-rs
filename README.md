<p align="center"><img src="assets/logo.svg" width="240" alt="dtk-rs logo: Ferris holding the deepin logo with pliers"></p>

# dtk-rs

Rust bindings for DTK6 (dtkwidget). Goal: write deepin apps with DTK UIs in pure Rust.

- Rust-facing wrappers over a `cxx` FFI layer; QObject handles are guarded, but reentrant native calls and non-QObject borrowed lifetimes are **not yet fully memory-safe** (see Design).
- **146 DTK Widget classes** bound by a header-scanning generator (`DComboBox`, `DSpinner`, `DDialog`, `DListView`, …), with first generated DCore/DGui pilots. The core also has hand-written `DApplication`, `DMainWindow`, layouts, `QTableWidget`, timers and painting.
- **Any signal** can be connected at runtime by name — no per-signal glue code.
- Linux + Qt6 + DTK6 only. Not cross-platform (DTK isn't either).

## Requirements

A deepin/UOS (or any) Linux with Qt6 and DTK6 development packages; `pkg-config` must find `Qt6Widgets`, `dtk6widget`, etc. Build:

```sh
cargo build
```

## Layout

```
dtk-sys/   FFI layer: C++ shim (cpp/) + cxx::bridge + build.rs (locates Qt6/DTK6 via pkg-config, moc for the signal relay)
dtk/       Rust wrappers: hand-written core + generated widgets.rs/core.rs/gui.rs
dtk-cxxqt/ CXX-Qt imports of DTK QObjects (typed signals, const getters); no ownership
tools/gen.py  scans DWidget headers and selected DCore/DGui pilot headers; regenerates all bridges
GEN_REPORT.md / GEN_CORE_REPORT.md / GEN_GUI_REPORT.md: coverage (DCore/DGui reports are pilots only)
TODO.md      known gaps and follow-ups
```

## Usage

Not on crates.io yet; add from git:

```sh
cargo add dtk --git https://github.com/st0nie/dtk-rs.git
```

```rust
use dtk::*;

let code = DApplication::with("my-app", |app| {
    let win = DMainWindow::new();
    win.titlebar().set_title("Hello");
    let btn = DSuggestButton::new("Click me");
    btn.on_clicked(|| println!("clicked"));
    btn.show();
    win.resize(400, 300);
    win.show();
    app.exec()
}); // windows, pending timers, connections and callbacks are cleaned up here
std::process::exit(code);
```

### Scoped GUI lifetime

`DApplication::with(name, |app| ...)` replaces the old owning-looking `new` API.
The borrowed token cannot escape, be cloned into an owner, or enter a `'static`
callback. `DApplication::with_quit_guard(name, guard, |app| ...)` replaces
`new_with_quit_guard`; a false guard still vetoes Qt's quit event. Use static
`DApplication::quit` from callbacks, not an owning application handle.

Scope exit (normal return, no `exec`, or panic unwinding) first blocks
application-dependent object access/construction, nested application scopes,
and callback registration/dispatch, then joins DTK's native configuration
worker and synchronously deletes GUI top-level windows, constructed QObjects
and application children. It frees pending timers,
connections and Rust captures without another event-loop turn. `exec()` exit
alone leaves the scope live. Returned widget/object views and owners become
invalid at the outer boundary; later access panics, and their `Drop` is a no-op.
Borrowed DTK singletons are **not** assumed to be ours to delete: even those
that native code keeps alive lose their old guards and binding-owned relays.
Non-QObject singleton borrows (e.g. `DFontSizeManager`, `DFileIconProvider`)
also expire at the scope boundary; their unsafe native-lifetime contracts
within an open scope are unchanged. Independent owned opaque values/copies
remain owned by Rust and are not confused with application-owned QObjects.
Independent hand-written Qt values (`QVariant`, `QColor`, `QFont`, `QImage`, …)
may still be constructed/accessed during capture cleanup: they do not reenter
an application or register callbacks. Generated opaque methods/constructors
conservatively reject closing-scope access.
A later scope creates fresh guards and cannot revive an earlier handle.

Only one GUI or headless application can exist at a time, on Linux's process
main thread. Scopes may be created again sequentially. DTK's native-owned
configuration QThread finishes accepted work through a FIFO worker-affinity
stop marker, then is joined before application deletion (Qt6 requires its
event/notify frames to return). Direct quit must not strand accepted config
updates. The native singleton is not deleted; it restarts only
after the next GUI/Core application is fully constructed. During a repeated
native DTK constructor you may see `Warning: The provided thread is not
running.`: its generated configuration builder only queues initialization;
the worker is intentionally paused until the application is fully ready, then
runs the queued work. It is not restarted while the app pointer/vtable is being
constructed. Native QObject parents
and adoption/ownership-transfer history are preserved. Cleanup disables user
close/quit/event callbacks rather than allowing a close veto to leak windows.
Callback panics are caught at the FFI boundary; ordinary scope panics propagate
**after** native cleanup. As with Rust RAII generally, `process::exit`, abort
and forced termination bypass destructors: call `exit` only **after** `with`
returns. User-created `Rc` cycles are not broken automatically.

Imported DTK signals are statically typed through CXX-Qt and return an RAII
`dtk::Connection` (drop disconnects; `release()` keeps it and returns an id):

```rust
let _c = line_edit.on_text_changed(|text: String| { ... });
switch.on_checked_changed(|on| { ... }).release();
```

Arguments are converted to owned values inside the emission and the closure
runs after the emitting native call returns. See [CXXQT_INTEGRATION.md](CXXQT_INTEGRATION.md).
Any other signal can be connected at runtime by name:

```rust
widget.connect_signal("windowRadiusChanged()", || { ... });
widget.connect_signal_i32("currentRowChanged(int)", |row| { ... });
```

When a callback synchronously emits the same signal, Rust queues its copied
arguments until the outer `FnMut` call returns rather than aliasing a mutable
closure or dropping the emission. Destroying the sender or unregistering the
callback discards pending invocations; see `signal_reentrancy_selfcheck`.
`unregister_callback` for a GUI connection must be called on its GUI thread;
a worker call returns `false` without touching Qt's relay map.

Generated Widget classes live under `dtk::widgets` (DComboBox, DSpinner, DDialog…). `dtk::core::{DStandardPaths, DSysInfo, DConfig}` and `dtk::gui::{DGuiApplicationHelper, DImageHandler, DFontManager}` are **pilot classes only**; they do not imply that all DCore/DGui types are bound. DConfig's caller-owned `create_generic`/`create_for_app` results use `Owned<DConfig>` and work with either a scoped headless `QCoreApplication` or GUI `DApplication`; native `set_app_id` is `unsafe` because it must precede all QCoreApplication creation. Core path locations/options and GUI palette/size modes use generated Rust enums with their discriminants read from the compiled native headers; remaining raw `i32` enum inputs (e.g. `set_mode`, `set_attribute`) require `unsafe` pending a typed/synchronization audit. `DImageHandler::find_all_meta_data()` returns an owned `BTreeMap<String, String>`; native DTK may return an empty map when its optional FreeImage backend is absent, even if Qt's QImage retains PNG text.

## Headless QtCore services

`QCoreApplication::with` lends a non-owning application token on the process
main thread. It needs neither a display nor a Qt platform plugin:

```rust
use dtk::{QCoreApplication, QTimer};
use dtk::core::DConfig;

QCoreApplication::with("service", |app| {
    let config = DConfig::create_generic("service-settings", "").unwrap();
    // Missing metadata may still yield an object; check config.is_valid().
    QTimer::single_shot(0, QCoreApplication::quit);
    assert_eq!(app.exec(), 0);
});
```

The native owner cannot be moved into a `'static` callback. On normal return
or panic, the scope blocks reentry, destroys constructed QtCore objects and
unfired timers, and releases the application's singleton. Escaping guarded
handles become invalid; owned values such as `QVariant` remain usable. This
cleanup also works if `exec()` never ran or already returned. Native QObject
parents are unchanged—scope cleanup uses a registry of **constructor owners**,
not borrowed getters. GUI types still require `DApplication::with`; both
application types share one native scoped runtime and shutdown protocol.
Qt/DTK dev packages are still needed for linking, even in headless services.

## Examples

All examples live in [`dtk/examples/`](dtk/examples/) and build with `cargo build --examples`.

| Example | Shows | Run |
|---|---|---|
| [`hello.rs`](dtk/examples/hello.rs) | Minimal app: `DMainWindow`, titlebar, `DLabel`, buttons, `DMessageBox`, sharing state into callbacks via `Rc<Cell<_>>` | `cargo run --example hello` |
| [`controls.rs`](dtk/examples/controls.rs) | Control tour: `DLineEdit`, `DSearchEdit`, `DSwitchButton`, `DSpinner`, floating messages via `DMessageManager`, connecting arbitrary signals by name | `cargo run --example controls` |
| [`mpsc_eventfd.rs`](dtk/examples/mpsc_eventfd.rs) | **Background thread → GUI** with `std::sync::mpsc` + `eventfd` + `QSocketNotifier` (see below) | `cargo run --example mpsc_eventfd` |
| [`demo.rs`](dtk/examples/demo.rs) | Binding self-checks: real argv, palette round-trip, `PaintDelegate` custom cell painting, `QSocketNotifier` on a pipe | `cargo run --example demo` |
| [`dialog_selfcheck.rs`](dtk/examples/dialog_selfcheck.rs) | `DDialog` checks | `cargo run --example dialog_selfcheck` |
| [`lifetime_selfcheck.rs`](dtk/examples/lifetime_selfcheck.rs) | Qt callback cleanup, table metadata, central widget ownership | `QT_QPA_PLATFORM=offscreen cargo run --example lifetime_selfcheck` |
| [`cxxqt_signal_selfcheck.rs`](dtk/examples/cxxqt_signal_selfcheck.rs) | CXX-Qt typed DTK signals: RAII/release, deferred FIFO delivery, sender destruction, scope cleanup | `QT_QPA_PLATFORM=offscreen cargo run --example cxxqt_signal_selfcheck` |
| [`threading_selfcheck.rs`](dtk/examples/threading_selfcheck.rs) | Typed worker → GUI delivery, ordering, destruction | `QT_QPA_PLATFORM=offscreen cargo run --example threading_selfcheck` |
| [`opaque_lifetime_selfcheck.rs`](dtk/examples/opaque_lifetime_selfcheck.rs) | Owned non-QObject DTK allocations and generated C++ deleters | `QT_QPA_PLATFORM=offscreen cargo run --example opaque_lifetime_selfcheck` |
| [`guard_selfcheck.rs`](dtk-sys/examples/guard_selfcheck.rs) | QPointer clears when Qt deletes an object | `QT_QPA_PLATFORM=offscreen cargo run -p dtk-sys --example guard_selfcheck` |
| [`guard_bench.rs`](dtk/examples/guard_bench.rs) | Raw vs guarded Qt/DTK getter cost | `QT_QPA_PLATFORM=offscreen cargo run --release --example guard_bench` |
| [`layout_selfcheck.rs`](dtk/examples/layout_selfcheck.rs) | Generated `QLayout*` binding and native parent-child lifetime | `QT_QPA_PLATFORM=offscreen cargo run --example layout_selfcheck` |
| [`variant_selfcheck.rs`](dtk/examples/variant_selfcheck.rs) | QVariant/QVariantList roundtrip through DTK tabs and list items | `QT_QPA_PLATFORM=offscreen cargo run --example variant_selfcheck` |
| [`action_selfcheck.rs`](dtk/examples/action_selfcheck.rs) | QAction/QMenu state, triggered(bool), Qt-owned callback cleanup | `QT_QPA_PLATFORM=offscreen cargo run --example action_selfcheck` |
| [`model_selfcheck.rs`](dtk/examples/model_selfcheck.rs) | Guarded QAbstractItemModel, DListView, QVariant data and parent ownership | `QT_QPA_PLATFORM=offscreen cargo run --example model_selfcheck` |
| [`no_application_selfcheck.rs`](dtk/examples/no_application_selfcheck.rs) | Reject GUI object creation before DApplication or on a worker thread; keep QtCore values usable | `QT_QPA_PLATFORM=offscreen cargo run --example no_application_selfcheck` |
| [`core_paths_selfcheck.rs`](dtk/examples/core_paths_selfcheck.rs) | DCore path lookup and system info without QApplication | `cargo run --example core_paths_selfcheck` |
| [`gui_helper_selfcheck.rs`](dtk/examples/gui_helper_selfcheck.rs) | DGui image/color adjustments, DPalette ownership and checked singleton | `QT_QPA_PLATFORM=offscreen cargo run --example gui_helper_selfcheck` |
| [`gui_image_selfcheck.rs`](dtk/examples/gui_image_selfcheck.rs) | DImageHandler QImage transforms, rotation, saved-image text and optional native metadata | `QT_QPA_PLATFORM=offscreen cargo run --example gui_image_selfcheck` |
| [`gui_font_selfcheck.rs`](dtk/examples/gui_font_selfcheck.rs) | DFontManager's typed size table, native base-font offset and guarded lifetime | `QT_QPA_PLATFORM=offscreen cargo run --example gui_font_selfcheck` |
| [`config_selfcheck.rs`](dtk/examples/config_selfcheck.rs) | DConfig factory ownership, QVariant fallback and deferred deletion | `QT_QPA_PLATFORM=offscreen cargo run --example config_selfcheck` |
| [`core_application_selfcheck.rs`](dtk/examples/core_application_selfcheck.rs) | Headless scope, pending callback cleanup, panic/reentry, invalidated handles and repeated application scopes | `env -u DISPLAY -u WAYLAND_DISPLAY -u QT_QPA_PLATFORM cargo run --example core_application_selfcheck` |
| [`gui_application_selfcheck.rs`](dtk/examples/gui_application_selfcheck.rs) | GUI scope cleanup, native deletion witnesses, singleton invalidation, callback/reentry safety, no-exec/panic, repeated scopes and latched DConfig queue completion | `QT_QPA_PLATFORM=offscreen cargo run --example gui_application_selfcheck` |

Full local validation (generator reproducibility/tests, fmt/clippy/check,
workspace tests including compile-fail doctests, and **every** example in
Debug and Release, plus 32 fresh-process native shutdown stress runs in each
profile; GUI uses offscreen, QtCore runs without display variables):

```sh
bash tools/check.sh
```

Individual smoke tests (CI-friendly, no display needed):

```sh
QT_QPA_PLATFORM=offscreen ./target/debug/examples/demo --smoke
QT_QPA_PLATFORM=offscreen ./target/debug/examples/mpsc_eventfd --smoke
QT_QPA_PLATFORM=offscreen ./target/debug/examples/lifetime_selfcheck
QT_QPA_PLATFORM=offscreen ./target/debug/examples/threading_selfcheck
QT_QPA_PLATFORM=offscreen ./target/debug/examples/opaque_lifetime_selfcheck
QT_QPA_PLATFORM=offscreen ./target/debug/examples/layout_selfcheck
QT_QPA_PLATFORM=offscreen ./target/debug/examples/variant_selfcheck
QT_QPA_PLATFORM=offscreen ./target/debug/examples/action_selfcheck
QT_QPA_PLATFORM=offscreen ./target/debug/examples/model_selfcheck
QT_QPA_PLATFORM=offscreen ./target/debug/examples/no_application_selfcheck
QT_QPA_PLATFORM=offscreen cargo run -p dtk-sys --example guard_selfcheck
env -u DISPLAY -u WAYLAND_DISPLAY -u QT_QPA_PLATFORM ./target/debug/examples/core_application_selfcheck
```

## Threading: typed worker → GUI messages

Qt widgets are `!Send` and must remain on the GUI thread. `GuiSender<T>` delivers only `Send` values to a handler bound to a Qt owner; application code needs no file descriptors, unsafe blocks or timer polling:

```rust
let to_gui = GuiSender::new(&window, move |text: String| label.set_text(&text))?;
std::thread::spawn(move || {
    let result = do_work();
    let _ = to_gui.send(result); // returns BrokenPipe if the window has been destroyed
});
```

A borrowed `QCoreApplication` scope token can also own a `GuiSender` for
headless worker-to-event-loop messages; scope exit disconnects its receiver.

The owner owns the Qt notifier and Rust callback; dropping the owner disconnects the receiver. `send()` only guarantees enqueue, not delivery if the owner is destroyed first. Internally Linux `eventfd` wakes Qt's event loop, writes are coalesced and messages are processed in bounded batches to keep input responsive. Compare [`threading_selfcheck.rs`](dtk/examples/threading_selfcheck.rs) for an offscreen test and [`mpsc_eventfd.rs`](dtk/examples/mpsc_eventfd.rs) for the low-level pattern. `QSocketNotifier` remains available for eventfd/signalfd/pipes and other externally owned fds.

## Design

Deep dive (layers, full binding walkthrough, signal mechanism, lifetime rules): [ARCHITECTURE.md](ARCHITECTURE.md).

- **Application**: create one application on the **process main thread**. QWidget/DGui/clipboard/pixmap/themed-icon APIs need `DApplication`; DConfig, QTimer and QSocketNotifier need either it or scoped `QCoreApplication::with`. Entry points reject missing runtimes, wrong threads and application-scope teardown reentry before accessing native objects. QtCore-only values (`QVariant`, `QVariantList`) can be made before it. Trying to create a second application fails before Qt's singleton constructor and cleans up the pending Rust callback. Arguments pass as raw Linux argv bytes rather than lossy delimiter-joined strings.
- **Lifetime — three wrapper cases.** QObject/widget constructors return `Owned<T>`; their dereferenced `T` is a non-owning `Clone` view guarded by `QPointer` (`!Send`, GUI thread). Drop of an owner *never adopted by Qt* queues `deleteLater()`; after the first Qt parent change its original owner permanently relinquishes deletion, even if an audited `take` method later produces a **new** owner. During a live scope deferred deletion requires a running event loop. Both `DApplication::with` and `QCoreApplication::with` additionally destroy registered constructor/transfer-owned objects at scope end, even if the loop never ran or already stopped; escaped guards (including native-surviving DTK singleton views) remain dead. Pointer getters remain borrowed `Option<T>`, not owned based on method names—this DTK version leaves a taken header parented to a deferred-deleted container. Verified central-widget and feature-item releases return fresh `Owned<T>`. Non-QObject constructors/copies and print-setting factories return `OwnedOpaque<T>` with generated matching deleters; audited process-static getters are safe borrowed scope-checked views, never deletion capabilities. Arbitrary foreign virtual factories remain unsafe with their concrete ownership contract. `DPrinter`/blur-group operations require GUI; independent opaque values retain no-app construction but reject shutdown reentry. Qt values and `WaterMarkData` own copies independent of their original object. Neither owner exposes `DerefMut` or a pinned exclusive native reference through a cloned view.
- **Runtime borrowing**: native transactions include arguments, calls and result wrapping. Ordinary callbacks execute after their native frames return; synchronous painting/close/quit guards reject QObject reentry. Suspended modal loops admit only audited timer/quit/dialog-close controls; deferred deletion waits for borrowed frames to return. Retained native pointer, duplicate/cycle and transfer contracts are enforced, not inferred from QPointer checks. See [RUNTIME_SAFETY_AUDIT.md](RUNTIME_SAFETY_AUDIT.md), [`runtime_borrow_selfcheck.rs`](dtk/examples/runtime_borrow_selfcheck.rs) and [`pointer_contract_selfcheck.rs`](dtk/examples/pointer_contract_selfcheck.rs).
- **Callbacks are `'static`** — share widgets/state into them with `Rc`/`RefCell`/`Cell` (never `Arc<Mutex>`: everything runs on one thread anyway).
- **Signals**: `DtkRelay` (Q_OBJECT + SLOT) string-connects any signal -> Rust callback id -> closure in a thread_local registry. Qt-object destruction cancels callbacks immediately and retires capture destructors to a safe Rust boundary; `dtk::unregister_callback(id)` disconnects early. `QTimer::single_shot` runs a `FnOnce` and cleans up even on panic or Qt cancellation.
- **Event overrides**: `ScopedDApplication` (QEvent::Quit guard) / `DtkMainWindowEx` (showEvent/closeEvent) shim subclasses -> Rust callbacks. Entry points: `DApplication::with_quit_guard`, `DMainWindow::new_with_events`.
- **Custom painting**: `PaintDelegate` (QStyledItemDelegate subclass) forwards paint to Rust, with `Painter` primitives + `ModelIndex::data_*`. `PaintWidgetEvent::Paint` only borrows a `Painter` within its callback; storing it afterward is a compile error.
- **Type mapping**: QString<->&str/String, numbers direct, enums/QFlags->i32 (`dtk::qt` constants module); QColor/QImage/QBrush/QPainterPath/QKeySequence/QUrl/DPalette/QFont/QPalette/QPixmap/QSize/QPoint/QRect/QVariant/QVariantList values -> owning wrappers, QWidget* and DTK class pointers <-> guarded views. QImage is usable before QApplication; QPixmap is not.
- **Qt ownership**: `QTimer::new(&owner)`, `QSocketNotifier::new(fd, &owner)` and `PaintDelegate::new(&owner, f)` require a QObject parent; the returned `Owned<T>` can be cloned as a non-owning view for callbacks. `DMainWindow::take_central_widget()` has verified ownership transfer and returns `Option<Owned<QWidget>>`, whereas generated `take_*` getters do **not** assume transfer. See [`ownership_transfer_selfcheck.rs`](dtk/examples/ownership_transfer_selfcheck.rs) for the native difference and [`lifetime_selfcheck.rs`](dtk/examples/lifetime_selfcheck.rs) for callback cleanup.
- **QSocketNotifier**: readiness callbacks (including legacy typed fd signals) are coalesced per pending/running id and native polling pauses until completion, honoring explicit disable, so delayed delivery cannot replay an already-drained fd. `new(fd, &owner)` + `on_activated`; for custom eventfd/signalfd/pipe sources, not required for ordinary worker → GUI messages.
- **Generator**: regex-parses DTK headers and classifies verified Qt ancestry as QWidget, QObject or opaque (non-QObject). C++ `static_cast` computes actual base pointers once per wrapper (cached for QWidget operations); generated QObject/Widget wrappers share a QPointer guard. Pointer getters return `Option<T>` because Qt may return `nullptr` (e.g. an empty `DStackWidget::current_widget()`). Only mapped signatures are generated; skips land in GEN_REPORT.md. Base-class Qt methods (e.g. `QProgressBar::setValue`) are added to the shim when needed.
- **Table items**: `QTableWidget::item(row,col)` returns an optional `TableCell` **coordinate view**, not an owning or borrowed `QTableWidgetItem*`. Each method resolves the current item again and returns `None`/`false` if the cell became empty. If the item is replaced, the view operates on the replacement at the same coordinates. Header labels use `&[impl AsRef<str>]` and Qt `QStringList`, so `|`, NUL and Unicode are preserved (no delimiter encoding). Cell text/data setters return `false` for invalid indices without allocating; setting UserRole data on an empty valid cell creates a Qt-owned item.
- **Layouts**: `QVBoxLayout::as_layout()`/`QHBoxLayout::as_layout()` share their QPointer guard with a `QLayout` view. DTK signatures such as `DFlowLayout::insert_layout` and `DBackgroundGroup::set_layout` accept this view and leave Qt's native parent/child ownership intact. `DBaseLine::left_layout/right_layout` return optional guarded `QBoxLayout` views.
- **Actions**: `QAction::new(text, &owner)` requires a Qt owner; `DMenu::add_action_object(&action)` adds an existing action without changing its QObject parent. `on_triggered` receives the checked `bool`, and Qt destruction releases registered callbacks.
- **Models**: `DVariantListModel::as_model()` and `QStandardItemModel::as_model()` share their guards with a `QAbstractItemModel` view for `DListView::set_model(&model)`. Qt views do **not** adopt models automatically: call `model.set_parent(&view)` if the view should own it, or construct `QStandardItemModel::new(&view)`. Qt's default standard model has zero columns; call `set_column_count(1)` before adding list items. `row_count`, `data`, `set_data`, and row insertion/removal use short-lived `QModelIndex` values inside C++ and Rust-owned `QVariant` values. `DListView::new()` starts with no model; install one before item operations. The Rust shim returns `0`/`false` for `count`/item mutators without a model instead of entering native DTK code that dereferences null.

## Regenerating

```
uv run --no-project python tools/gen.py && cargo build
```

Generated files (don't hand-edit): `dtk/src/{widgets,core,gui}.rs`, `dtk-sys/src/{gen,core,gui}_ffi.rs`, `dtk-sys/cpp/dtk_{gen,core,gui}_shim.cpp`, matching headers, `GEN*REPORT.md` and `OWNERSHIP_{WIDGET,CORE,GUI}_REPORT.md`. `uv run --no-project python tools/audit_hand.py` regenerates `OWNERSHIP_HAND_REPORT.md`; `bash tools/check.sh` verifies all inventories and complete Debug/Release behavior.

## License

MIT — see [LICENSE](LICENSE).
