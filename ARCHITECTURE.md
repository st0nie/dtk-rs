# dtk-rs architecture

How Rust bindings for DTK6 (dtkwidget) are implemented. For usage see [README.md](README.md);
for known gaps see [TODO.md](TODO.md).

## Big picture

Three layers plus a header-scanning generator:

```
┌────────────────────────────────────────────────────────────────┐
│ dtk/            Rust-facing API (raw-handle safety unfinished)  │
│   src/lib.rs      hand-written: core classes, macros, signals  │
│   src/widgets.rs  GENERATED: 146 DTK class wrappers            │
│   ↑ guarded QObject views, non-owning, !Send                   │
├────────────────────────────────────────────────────────────────┤
│ cxx::bridge #1  dtk-sys/src/lib.rs      hand-written core      │
│ cxx::bridge #2  dtk-sys/src/gen_ffi.rs  GENERATED (~100 types) │
│   ↑ cxx generates the Rust↔C++ ABI glue                        │
├────────────────────────────────────────────────────────────────┤
│ dtk-sys/cpp/    C++ shim (the unsafe FFI layer)                │
│   shim.cpp           hand-written: core + event-override       │
│                      subclasses + painter primitives           │
│   dtk_gen_shim.cpp   GENERATED: one free fn per DTK method     │
│   relay.cpp          DtkRelay (Q_OBJECT) signal forwarding     │
│   ↓ calls the real DTK6/Qt6 API                                │
├────────────────────────────────────────────────────────────────┤
│ dtk6widget / Qt6   (pkg-config link, moc for relay.h)          │
└────────────────────────────────────────────────────────────────┘

tools/gen.py  scans /usr/include/dtk6/DWidget/*.h, classifies Qt ancestry,
              and regenerates the four GENERATED files (+ GEN_REPORT.md)
```

## Why a C++ shim at all

cxx can only bridge types it knows: primitives, `rust::String`/`rust::Str`,
opaque C++ types behind pointers, and a few std containers. Qt types — `QString`,
`QList`, enums, overloaded methods — cannot cross. So every DTK method is
flattened into a **free function in namespace `dtkrs`** whose signature uses
only mappable types:

```cpp
// dtk_gen_shim.cpp — generated, one line per method
DComboBox *gen_d_combo_box_new() { return new DComboBox; }
void gen_d_combo_box_show_popup(DComboBox *self) { self->showPopup(); }
rust::String gen_d_about_dialog_product_name(DAboutDialog *self) { return to_rust_string(self->productName()); }
void gen_d_alert_control_set_message_alignment(DAlertControl *self, int32_t a) { self->setMessageAlignment(Qt::Alignment::fromInt(a)); }
QColor * gen_d_alert_control_alert_color(DAlertControl *self) { return new QColor(self->alertColor()); }
```

- `this` becomes the first parameter (`self`).
- Value-type returns are heap copies (`new QColor(...)`) — ownership passes to Rust.
- Enum/flags cross as `int32_t` with `static_cast` / `fromInt()`/`toInt()` at the boundary.
- `QString` crosses as `rust::Str`/`rust::String` via `from_rust_str`/`to_rust_string` helpers.

## End-to-end walkthrough: DComboBox

### 1. The DTK header

```cpp
// /usr/include/dtk6/DWidget/dcombobox.h
class DComboBox : public QComboBox, public DObject {
    Q_OBJECT
public:
    explicit DComboBox(QWidget *parent = nullptr);
    void showPopup() override;
    bool eventFilter(QObject *watched, QEvent *event) override;  // QEvent* unmapped → skipped
};
```

### 2. gen.py decides what to emit

Each public method's parameter/return types are checked against the type map:

| Method | Verdict |
|---|---|
| `DComboBox(QWidget *parent = nullptr)` | ctor with default args → emit `new()` |
| `void showPopup()` | clean → emit |
| `bool eventFilter(QObject*, QEvent*)` | `QEvent*` unmapped → skip, record in GEN_REPORT.md |

GEN_REPORT.md entry:

```
## DComboBox — 1 methods generated, 1 skipped
- `void addComboBox(const QString &text, const DComboBoxOptions &options);`
  ← unsupported param type: const DComboBoxOptions &
```

Skipped ≠ lost forever: the fix is usually adding a type mapping or declaring
another Qt class in both bridges (see TODO.md).

### 3. The four generated artifacts

```cpp
// dtk-sys/include/dtk_gen_shim.h — declaration
using Dtk::Widget::DComboBox;                 // DTK namespace flattened
DComboBox *gen_d_combo_box_new();
void gen_d_combo_box_show_popup(DComboBox *self);
```

```rust
// dtk-sys/src/gen_ffi.rs — cxx::bridge #2
#[cxx::bridge(namespace = "dtkrs")]
pub mod genffi {
    extern "C++" {
        include!("dtk_gen_shim.h");
        type DComboBox;                                      // opaque
        unsafe fn gen_d_combo_box_new() -> *mut DComboBox;
        unsafe fn gen_d_combo_box_show_popup(self_: *mut DComboBox);
    }
}
```

```rust
// dtk/src/widgets.rs — safe wrapper
widget_wrapper!(DComboBox, genffi::DComboBox);
impl DComboBox {
    pub fn new() -> Self {
        Self::from_raw(unsafe { genffi::gen_d_combo_box_new() })
    }
    pub fn show_popup(&self) {
        unsafe { genffi::gen_d_combo_box_show_popup(self.ptr) }
    }
}
impl Default for DComboBox { fn default() -> Self { Self::new() } }
```

`widget_wrapper!` (in `dtk/src/lib.rs`) expands to the struct plus all common
QWidget operations:

```rust
pub struct DComboBox {
    pub(crate) ptr: *mut genffi::DComboBox,   // non-owning raw pointer
    _not_send: PhantomData<*mut ()>,           // !Send/!Sync — GUI thread only
}
impl DComboBox {
    pub(crate) fn from_raw(ptr: *mut genffi::DComboBox) -> Self { ... }
    pub fn as_widget(&self) -> QWidget { ... }   // base-class view, for layouts
    pub fn show(&self)        { unsafe { ffi::widget_show(self.ptr.cast()) } }
    pub fn resize(&self, ...) { ... }
    // ... every common QWidget op, from hand-written bridge #1
}
impl Signal0 for DComboBox { ... }   // connect any arg-less signal
impl SignalI32 for DComboBox { ... } // connect any (int) signal
```

Note: **no `Drop` on widgets** — see Lifetime below.

### 4. Call chain at runtime

```rust
let combo = DComboBox::new();
// → genffi::gen_d_combo_box_new()     cxx ABI glue
//   → dtkrs::gen_d_combo_box_new()   C++ shim
//     → new DComboBox                real DTK object

combo.show();
// → ffi::widget_show(self.ptr.cast())   hand bridge #1 (base-class op)
//   → w->show()

combo.show_popup();
// → genffi::gen_d_combo_box_show_popup(self.ptr)   generated bridge #2
//   → self->showPopup()
```

Base-class Qt methods (e.g. `QProgressBar::setValue`) are not in DTK headers,
so the generator never sees them — they're added by hand to shim.cpp /
bridge #1 when needed (e.g. `impl widgets::DProgressBar` in dtk/src/lib.rs).

## Type mapping and inheritance

Classified using the transitive Qt base hierarchy from the headers: 85 QWidget
subclasses, 26 QObject subclasses without QWidget, 35 non-QObject opaque
classes. Only the first category gets `as_widget`/QWidget operations; only the
first two get QObject signal operations. A prefix-based guess previously gave
`DPrinter` and `DStylePainter` invalid QWidget casts. Generated DTK/Qt
pointer getters return `Option<Wrapper>` (e.g. an empty stack has no current
widget) rather than panicking on Qt's null. This classification is checked by
`python3 -m unittest tools.test_gen` and reflected in GEN_REPORT.md.

| C++ | crosses the bridge as | Rust-facing type |
|---|---|---|
| `QString` | `rust::Str` (in) / `rust::String` (out) | `&str` / `String` |
| `int`/`qint32`, `bool`, `qreal`… | `int32_t`, `bool`, `double`… | `i32`, `bool`, `f64`… |
| enums | `int32_t` + `static_cast` | `i32`; constants in `dtk::qt` |
| `QFlags` (e.g. `Qt::Alignment`) | `int32_t` + `fromInt`/`toInt` | `i32` |
| value types (`QColor`, `QFont`, `QPixmap`, `QIcon`, `QPalette`, `QSize`, `QPoint`, `QRect`, `QMargins`, `QVariant`, `QVariantList`, `DDciIcon`) | owning heap pointer | wrapper with `Drop` → shim `*_delete` |
| `QWidget*` / `QLayout*` / `QBoxLayout*` / `QAction*` / `QAbstractItemModel*` / DTK class pointers | opaque pointer | guarded non-owning QObject view |
| other `QList<T>`/`QMap`, `QPrinter*`, … | — | **unmapped → skipped** (GEN_REPORT.md, TODO.md) |

Two bridges, one namespace: bridge #1 (hand, core) and #2 (generated, bulk)
both declare opaque types in `namespace dtkrs`. Only `QWidget*` and a fixed
list of value types cross between the two bridges, via a `as _` pointer cast.
Sound because cxx opaque types are zero-sized Rust structs that are never
dereferenced or moved in Rust, and both bridges name the same C++ type.
The audit is recorded in tools/gen.py comments.

## Signals: runtime connect by name

No per-signal glue code. Any signal on any QObject can be connected at runtime:

```rust
btn.connect_signal("clicked(bool)", || { ... });
table.connect_signal_i32("currentRowChanged(int)", |row| { ... });
```

Mechanism:

```
Rust:  connect_signal("clicked(bool)", f)
         1. register_cb0(f) → id      closure boxed into thread_local HashMap<usize, Cb>
         2. relay_connect0(obj, "clicked(bool)", id)
C++:   DtkRelay (Q_OBJECT, SLOTs fire0()/fireI32(int))
         3. string-based connect: "2" + QMetaObject::normalizedSignature(signal)
            QObject::connect(sender, sig, relay, SLOT(fire0()))
         4. on signal: fire0() → extern "Rust" dtk_cb0(id)
Rust:  5. dtk_cb0(id) looks the id up and calls the closure
```

Details that matter:

- The string-based `connect(sender, SIGNAL, relay, SLOT)` form is used because
  Qt6 dropped the unregistered-`QMetaMethod` + lambda connect overloads.
- The relay object is parented to the sender → destroyed with it automatically.
- `relay.h` contains `Q_OBJECT`, so `build.rs` runs **moc** on it.
- The registry is `thread_local` — no locks; callbacks only ever fire on the Qt
  main thread.
- Dispatch is **remove-then-call-then-reinsert**, so a callback may safely
  register new callbacks (no RefCell double-borrow).
- `dtk::unregister_callback(id)` removes the closure and disconnects its relay;
  destruction of the Qt owner removes its Rust callback automatically. Callback
  ids currently being dispatched are cancelled without resurrecting the closure.
  One-shot timers use `FnOnce` (no extra `Rc<Cell>`), including panic cleanup.
- Signals with other argument types (e.g. `checkedChanged(bool)`) can still be
  connected via the arg-less relay — the argument is dropped; read the state
  from the widget inside the closure.

## Event overrides and custom painting

Signals can't intercept events. For those, shim.cpp subclasses DTK/Qt classes
and forwards to Rust callback ids (same registry):

| C++ subclass | Overrides | Rust entry point |
|---|---|---|
| `DtkAppEx : DApplication` | `event()` — `QEvent::Quit` asks a guard; `false` swallows the quit | `DApplication::new_with_quit_guard(guard)` |
| `DtkMainWindowEx : DMainWindow` | `showEvent` / `closeEvent` (close can be vetoed) | `DMainWindow::new_with_events(...)` |
| `RustDelegate : QStyledItemDelegate` | `paint` — paints the default DTK background first, then calls Rust for the overlay (icon/text) | `PaintDelegate::new(...)` |
| `DtkPaintWidget : QWidget` | paint, keyboard, mouse, IME, resize and focus forwarding | `PaintWidget::new(...)` |

Reentrant `PaintWidget` events with owned payloads (keys, resize, etc.) are
queued until the current Rust callback returns. A nested paint event cannot be
queued: its stack-scoped `QPainter*` becomes invalid when that inner C++ event
returns. Instead it requests another Qt repaint on the next event-loop turn.

`dtk_cb_paint` hands Rust the raw `QPainter*`/`QModelIndex*`; the safe layer
wraps them as `Painter` (draw_text/draw_pixmap/fill_rect/…) and `ModelIndex`
(`data_string`/`data_i64`/…).

## Lifetime & memory

Three wrapper cases, deliberately different:

1. **Widget/object wrappers** (`DComboBox`, `DLabel`, layouts, …): non-owning
   `Clone` views sharing a `QPointer<QObject>` guard, **no `Drop` on Qt objects**.
   Generated DTK methods and hand-written widget operations check the guard
   before crossing FFI; C++ `static_cast` adjusts QObject/QWidget base pointers
   for generated subclasses once at wrapper construction and caches both base
   pointers. This avoids an extra upcast FFI call on every common QWidget op. Qt deletes children when their parent is deleted,
   **not** unparented top-level windows when `QApplication` exits. Use
   `delete_later()` explicitly for unparented objects. Constructors require an
   existing `DApplication` before allocating GUI QObjects; `QVariant` and
   `QVariantList` remain usable as QtCore values without it.
   `QTimer`, `QSocketNotifier`, and `PaintDelegate` require an owning QObject parent.
   Parent deletion clears the guard and a later method call panics rather than
   dereferencing the dead pointer; `lifetime_selfcheck` exercises QWidget,
   generated widget and QObject handles. This alone does not prove every C++
   method safe against destruction *during* a reentrant call, nor provide RAII
   for top-level QObject ownership.
2. **Non-QObject generated constructors** (`DPrinter`, `DStyleOptionButton`,
   `DFileIconProvider`, etc.): return `OwnedOpaque<T>`, which uniquely owns the
   allocation and calls a generated `gen_*_delete` in `Drop`. Getter results
   (`global_provider()` etc.) remain non-owning and must never be deleted by
   Rust. The borrowed views must not outlive their C++ owner.
3. **Value-type wrappers** (`QColor`, `QFont`, `QPixmap`, `QIcon`, `QPalette`,
   `QSize`, `QPoint`, `QRect`, `QMargins`, `QVariant`, `QVariantList`,
   `DDciIcon`): own a heap copy created
   by the shim (`new QColor(...)`), freed in `Drop` via shim `*_delete`.

Object and value wrappers carry `PhantomData<*mut ()>` → `!Send`/`!Sync`;
`OwnedOpaque<T>` inherits its non-Send semantics from its inner view.
`Painter`/`ModelIndex` are temporary raw callback views, not storable references;
Qt widgets must only be touched from the GUI thread. Callbacks are
`'static` — share state into them with `Rc`/`RefCell`/`Cell` (never
`Arc<Mutex>`: everything runs on one thread anyway).

## Threading: worker → GUI

`GuiSender<T: Send>` (in `dtk/src/threading.rs`) exposes a typed, cloneable
producer. Its `FnMut(T)` handler is registered on the Qt owner's GUI thread:

```
worker: sender.send(msg) ── mpsc ───────────▶ GUI: bounded try_recv() + handler
        coalesced eventfd write ────────────▶ QSocketNotifier::on_activated()
```

The low-level Linux fd is private and RAII-owned; no widget or raw QObject
pointer crosses threads. Dropping the Qt owner destroys the notifier/relay,
drops the receiver, then `send()` returns BrokenPipe. Enqueue does not imply
successful delivery if destruction races with the GUI dispatch. Counter drains
are nonblocking, messages are processed in batches of 256, and the next batch
is rescheduled so fast workers do not starve GUI input. See
`dtk/examples/threading_selfcheck.rs`; `mpsc_eventfd.rs` illustrates the
older manual fd pattern for other external event sources.

## Build

`dtk-sys/build.rs`:

1. locates Qt6 **moc** (`/usr/lib/qt6/libexec/moc` → fallbacks), runs it on
   `include/relay.h` (the only header with `Q_OBJECT`);
2. `cxx_build::bridges(["src/lib.rs", "src/gen_ffi.rs"])` — both bridges;
3. compiles `shim.cpp` + `relay.cpp` + `dtk_gen_shim.cpp` + moc output with
   `-std=c++17 -fPIC` (Qt6 requires PIC);
4. `pkg-config` probes `Qt6Widgets/Gui/Core` + `dtk6widget/gui/core` for
   include paths, defines, and link flags;
5. `links = "dtk6widget"` in dtk-sys/Cargo.toml.

## Which files are hand-written vs generated

| File | Owner |
|---|---|
| `dtk-sys/src/lib.rs` (bridge #1, callback registry) | hand |
| `dtk-sys/cpp/shim.cpp`, `include/dtk_shim.h` | hand |
| `dtk-sys/cpp/relay.cpp`, `include/relay.h` | hand |
| `dtk-sys/build.rs` | hand |
| `dtk/src/lib.rs` (core classes, macros, signals, qt constants) | hand |
| `dtk-sys/src/gen_ffi.rs` | **generated** |
| `dtk-sys/cpp/dtk_gen_shim.cpp`, `include/dtk_gen_shim.h` | **generated** |
| `dtk/src/widgets.rs` | **generated** |
| `GEN_REPORT.md` | generated report |

Regenerate after DTK upgrades or when adding coverage:

```sh
python3 tools/gen.py && cargo build
```

Adding a new DTK method usually means extending the generator's type map and
regenerating. The current 228 skipped methods, raw non-QObject borrowed
pointers and unfinished top-level ownership mean this is not yet an exhaustive,
memory-safe Qt/DTK binding. CXX-Qt's
`extern "C++Qt"` can import C++ QObject methods and typed signals; integration
for existing DTK object types is a candidate for the next API layer. A local
DSwitchButton probe confirms CXX-Qt's typed `checkedChanged(bool)` connection
is disconnected on guard drop and its callback is released when Qt destroys the
widget. But the generated API requires `Pin<&mut DSwitchButton>`: converting
these clonable guarded handles to mutable pinned references from safe `&self` would
create unsound aliasing during reentrant Qt callbacks. Fix handle ownership and
borrowing first. CXX-Qt 0.10.0 also needs an edition-2021 bridge crate on this
Rust 1.97 setup; its macro currently fails in edition 2024. Its `CxxQtThread`
helper targets CXX-Qt-generated QObjects, so imported DTK objects still need a
verified cross-thread delivery strategy. Hand-written additions
(new Qt base-class methods, new value types, new event overrides) go in
shim.cpp + bridge #1.
