<p align="center"><img src="assets/logo.svg" width="240" alt="dtk-rs logo: Ferris holding the deepin logo with pliers"></p>

# dtk-rs

Rust bindings for DTK6 (dtkwidget). Goal: write deepin apps with DTK UIs in pure Rust.

- Rust-facing wrappers over a `cxx` FFI layer; QObject handles are guarded, but reentrant native calls and non-QObject borrowed lifetimes are **not yet fully memory-safe** (see Design).
- **~100 DTK classes** bound by a header-scanning generator (`DComboBox`, `DSpinner`, `DDialog`, `DListView`, …) plus hand-written core (`DApplication`, `DMainWindow`, layouts, `QTableWidget`, timers, painting).
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
dtk/       safe wrappers: hand-written core classes + widgets.rs (generator output)
tools/gen.py  header-scanning generator; rescans /usr/include/dtk6/DWidget/*.h and regenerates bindings
GEN_REPORT.md coverage report: what was generated, what was skipped (with reasons)
TODO.md      known gaps and follow-ups
```

## Usage

Not on crates.io yet; add from git:

```sh
cargo add dtk --git https://github.com/st0nie/dtk-rs.git
```

```rust
use dtk::*;

let app = DApplication::new("my-app");
let win = DMainWindow::new();
win.titlebar().set_title("Hello");
let btn = DSuggestButton::new("Click me");
btn.on_clicked(|| println!("clicked"));
btn.show();
win.resize(400, 300);
win.show();
std::process::exit(app.exec());
```

Any signal can be connected (runtime connect by name):

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

The generator-covered classes live under `dtk::widgets` (DComboBox, DSpinner, DDialog…).

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
| [`threading_selfcheck.rs`](dtk/examples/threading_selfcheck.rs) | Typed worker → GUI delivery, ordering, destruction | `QT_QPA_PLATFORM=offscreen cargo run --example threading_selfcheck` |
| [`opaque_lifetime_selfcheck.rs`](dtk/examples/opaque_lifetime_selfcheck.rs) | Owned non-QObject DTK allocations and generated C++ deleters | `QT_QPA_PLATFORM=offscreen cargo run --example opaque_lifetime_selfcheck` |
| [`guard_selfcheck.rs`](dtk-sys/examples/guard_selfcheck.rs) | QPointer clears when Qt deletes an object | `QT_QPA_PLATFORM=offscreen cargo run -p dtk-sys --example guard_selfcheck` |
| [`guard_bench.rs`](dtk/examples/guard_bench.rs) | Raw vs guarded Qt/DTK getter cost | `QT_QPA_PLATFORM=offscreen cargo run --release --example guard_bench` |
| [`layout_selfcheck.rs`](dtk/examples/layout_selfcheck.rs) | Generated `QLayout*` binding and native parent-child lifetime | `QT_QPA_PLATFORM=offscreen cargo run --example layout_selfcheck` |
| [`variant_selfcheck.rs`](dtk/examples/variant_selfcheck.rs) | QVariant/QVariantList roundtrip through DTK tabs and list items | `QT_QPA_PLATFORM=offscreen cargo run --example variant_selfcheck` |
| [`action_selfcheck.rs`](dtk/examples/action_selfcheck.rs) | QAction/QMenu state, triggered(bool), Qt-owned callback cleanup | `QT_QPA_PLATFORM=offscreen cargo run --example action_selfcheck` |
| [`model_selfcheck.rs`](dtk/examples/model_selfcheck.rs) | Guarded QAbstractItemModel, DListView, QVariant data and parent ownership | `QT_QPA_PLATFORM=offscreen cargo run --example model_selfcheck` |
| [`no_application_selfcheck.rs`](dtk/examples/no_application_selfcheck.rs) | Reject GUI object creation before DApplication, keep QtCore values usable | `QT_QPA_PLATFORM=offscreen cargo run --example no_application_selfcheck` |

Headless smoke tests (CI-friendly, no display needed):

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

The owner owns the Qt notifier and Rust callback; dropping the owner disconnects the receiver. `send()` only guarantees enqueue, not delivery if the owner is destroyed first. Internally Linux `eventfd` wakes Qt's event loop, writes are coalesced and messages are processed in bounded batches to keep input responsive. Compare [`threading_selfcheck.rs`](dtk/examples/threading_selfcheck.rs) for an offscreen test and [`mpsc_eventfd.rs`](dtk/examples/mpsc_eventfd.rs) for the low-level pattern. `QSocketNotifier` remains available for eventfd/signalfd/pipes and other externally owned fds.

## Design

Deep dive (layers, full binding walkthrough, signal mechanism, lifetime rules): [ARCHITECTURE.md](ARCHITECTURE.md).

- **Application**: construct one `DApplication` **before** QWidget/QObject, timers, clipboard, pixmaps and themed icons; these safe entry points now reject calls made without it **or from another thread**, instead of entering native Qt from the wrong context. QtCore-only values (`QVariant`, `QVariantList`) can be made before it. Trying to create a second application fails before Qt's singleton constructor and cleans up the pending Rust callback. Arguments pass as raw Linux argv bytes rather than lossy delimiter-joined strings.
- **Lifetime — three wrapper cases.** QObject/widget constructors return `Owned<T>`; their dereferenced `T` is a non-owning `Clone` view guarded by `QPointer` (`!Send`, GUI thread). Drop of an owner *never adopted by Qt* queues `deleteLater()`; after the first Qt parent change its original owner permanently relinquishes deletion, even if an audited `take` method later produces a **new** owner. This requires a running event loop to execute deferred deletion; dropping after shutdown is not deterministic cleanup. Pointer getters remain borrowed `Option<T>`, not owned based on method names—this version of DTK schedules deletion after `DListView::take_header_widget`. Non-QObject constructors such as `DPrinter::new()` return `OwnedOpaque<DPrinter>` with a generated C++ deleter. Generated non-QObject pointer getters require `unsafe` because their native allocation lifetime and ownership (borrowed versus transferred) have not been verified; their views are not `Copy`/`Clone`. Value types (`QColor`, `QImage`, `QBrush`, `QPainterPath`, `QKeySequence`, `QUrl`, …) own copies freed in `Drop`; `WaterMarkData` returned by DTK is copy-constructed into `OwnedOpaque<WaterMarkData>`. Native reentrant destruction and opaque borrows still need auditing; this is **not yet a fully memory-safe binding**.
- **Callbacks are `'static`** — share widgets/state into them with `Rc`/`RefCell`/`Cell` (never `Arc<Mutex>`: everything runs on one thread anyway).
- **Signals**: `DtkRelay` (Q_OBJECT + SLOT) string-connects any signal -> Rust callback id -> closure in a thread_local registry. Qt-object destruction releases its Rust callbacks; `dtk::unregister_callback(id)` disconnects early. `QTimer::single_shot` runs a `FnOnce` and cleans up even on panic or Qt cancellation.
- **Event overrides**: `DtkAppEx` (QEvent::Quit guard) / `DtkMainWindowEx` (showEvent/closeEvent) shim subclasses -> Rust callbacks. Entry points: `DApplication::new_with_quit_guard`, `DMainWindow::new_with_events`.
- **Custom painting**: `PaintDelegate` (QStyledItemDelegate subclass) forwards paint to Rust, with `Painter` primitives + `ModelIndex::data_*`. `PaintWidgetEvent::Paint` only borrows a `Painter` within its callback; storing it afterward is a compile error.
- **Type mapping**: QString<->&str/String, numbers direct, enums/QFlags->i32 (`dtk::qt` constants module); QColor/QImage/QBrush/QPainterPath/QKeySequence/QUrl/QFont/QPalette/QPixmap/QSize/QPoint/QRect/QVariant/QVariantList values -> owning wrappers, QWidget* and DTK class pointers <-> guarded views. QImage is usable before QApplication; QPixmap is not.
- **Qt ownership**: `QTimer::new(&owner)`, `QSocketNotifier::new(fd, &owner)` and `PaintDelegate::new(&owner, f)` require a QObject parent; the returned `Owned<T>` can be cloned as a non-owning view for callbacks. `DMainWindow::take_central_widget()` has verified ownership transfer and returns `Option<Owned<QWidget>>`, whereas generated `take_*` getters do **not** assume transfer. See [`ownership_transfer_selfcheck.rs`](dtk/examples/ownership_transfer_selfcheck.rs) for the native difference and [`lifetime_selfcheck.rs`](dtk/examples/lifetime_selfcheck.rs) for callback cleanup.
- **QSocketNotifier**: `new(fd, &owner)` + `on_activated`; for custom eventfd/signalfd/pipe sources, not required for ordinary worker → GUI messages.
- **Generator**: regex-parses DTK headers and classifies verified Qt ancestry as QWidget, QObject or opaque (non-QObject). C++ `static_cast` computes actual base pointers once per wrapper (cached for QWidget operations); generated QObject/Widget wrappers share a QPointer guard. Pointer getters return `Option<T>` because Qt may return `nullptr` (e.g. an empty `DStackWidget::current_widget()`). Only mapped signatures are generated; skips land in GEN_REPORT.md. Base-class Qt methods (e.g. `QProgressBar::setValue`) are added to the shim when needed.
- **Table items**: `QTableWidget::item(row,col)` returns an optional `TableCell` **coordinate view**, not an owning or borrowed `QTableWidgetItem*`. Each method resolves the current item again and returns `None`/`false` if the cell became empty. If the item is replaced, the view operates on the replacement at the same coordinates. Header labels use `&[impl AsRef<str>]` and Qt `QStringList`, so `|`, NUL and Unicode are preserved (no delimiter encoding). Cell text/data setters return `false` for invalid indices without allocating; setting UserRole data on an empty valid cell creates a Qt-owned item.
- **Layouts**: `QVBoxLayout::as_layout()`/`QHBoxLayout::as_layout()` share their QPointer guard with a `QLayout` view. DTK signatures such as `DFlowLayout::insert_layout` and `DBackgroundGroup::set_layout` accept this view and leave Qt's native parent/child ownership intact. `DBaseLine::left_layout/right_layout` return optional guarded `QBoxLayout` views.
- **Actions**: `QAction::new(text, &owner)` requires a Qt owner; `DMenu::add_action_object(&action)` adds an existing action without changing its QObject parent. `on_triggered` receives the checked `bool`, and Qt destruction releases registered callbacks.
- **Models**: `DVariantListModel::as_model()` and `QStandardItemModel::as_model()` share their guards with a `QAbstractItemModel` view for `DListView::set_model(&model)`. Qt views do **not** adopt models automatically: call `model.set_parent(&view)` if the view should own it, or construct `QStandardItemModel::new(&view)`. Qt's default standard model has zero columns; call `set_column_count(1)` before adding list items. `row_count`, `data`, `set_data`, and row insertion/removal use short-lived `QModelIndex` values inside C++ and Rust-owned `QVariant` values. `DListView::new()` starts with no model; install one before item operations. The Rust shim returns `0`/`false` for `count`/item mutators without a model instead of entering native DTK code that dereferences null.

## Regenerating

```
python3 tools/gen.py && cargo build
```

Generated files (don't hand-edit): `dtk/src/widgets.rs`, `dtk-sys/src/gen_ffi.rs`, `dtk-sys/cpp/dtk_gen_shim.cpp`, `dtk-sys/include/dtk_gen_shim.h`.

## License

MIT — see [LICENSE](LICENSE).
