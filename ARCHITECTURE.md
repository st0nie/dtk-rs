# dtk-rs architecture

How Rust bindings for DTK6 (dtkwidget) are implemented. For usage see [README.md](README.md);
for known gaps see [TODO.md](TODO.md).

## Big picture

Three layers plus a header-scanning generator:

```
┌────────────────────────────────────────────────────────────────┐
│ dtk/            Rust-facing API (serialized native borrows)   │
│   src/lib.rs      hand-written: core classes, macros, signals  │
│   src/widgets.rs  GENERATED: 146 DTK Widget classes             │
│   src/core.rs/gui.rs GENERATED: DCore/DGui pilot classes      │
│   src/signals.rs  typed on_* signals via dtk-cxxqt (CXX-Qt)    │
│   ↑ guarded QObject views, non-owning, !Send                   │
├────────────────────────────────────────────────────────────────┤
│ cxx::bridge #1   dtk-sys/src/lib.rs      hand-written core     │
│ cxx::bridge #2-4 dtk-sys/src/{gen,core,gui}_ffi.rs GENERATED  │
│   ↑ cxx generates the Rust↔C++ ABI glue                        │
├────────────────────────────────────────────────────────────────┤
│ dtk-sys/cpp/    C++ shim (the unsafe FFI layer)                │
│   shim.cpp           hand-written: core + event-override       │
│                      subclasses + painter primitives           │
│   dtk_{gen,core,gui}_shim.cpp GENERATED free DTK methods     │
│   relay.cpp          DtkRelay (Q_OBJECT) signal forwarding     │
│   ↓ calls the real DTK6/Qt6 API                                │
├────────────────────────────────────────────────────────────────┤
│ dtk6widget / Qt6   (pkg-config link, moc for relay.h)          │
└────────────────────────────────────────────────────────────────┘

tools/gen.py  scans all DWidget headers plus selected DCore/DGui pilot headers,
              classifies Qt ancestry and regenerates three domain APIs/reports.
              Pilot report counts do NOT describe full DCore/DGui coverage.
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
// dtk/src/widgets.rs — safe wrapper (simplified for this walkthrough)
widget_wrapper!(DComboBox, genffi::DComboBox,
                genffi::gen_d_combo_box_as_qobject,
                genffi::gen_d_combo_box_as_widget);
impl DComboBox {
    pub fn new() -> Owned<Self> {
        crate::require_application();
        Owned::new(Self::from_raw(unsafe { genffi::gen_d_combo_box_new() }))
    }
    pub fn show_popup(&self) {
        unsafe { genffi::gen_d_combo_box_show_popup(self.live_ptr()) }
    }
}
impl Default for Owned<DComboBox> { fn default() -> Self { DComboBox::new() } }
```

`widget_wrapper!` (in `dtk/src/lib.rs`) expands to the struct plus all common
QWidget operations:

```rust
#[derive(Clone)]
pub struct DComboBox {
    pub(crate) ptr: *mut genffi::DComboBox, // non-owning checked view
    guard: Rc<ObjectGuard>,                 // shared liveness + parent history
    _not_send: PhantomData<*mut ()>,        // !Send/!Sync — GUI thread only
    // cached QObject and QWidget base pointers (C++ static_cast at construction)
}
impl DComboBox {
    pub(crate) fn from_raw(ptr: *mut genffi::DComboBox) -> Self { ... }
    pub(crate) fn live_ptr(&self) -> *mut genffi::DComboBox { ... }
    pub fn as_widget(&self) -> QWidget { ... } // guarded base-class view
    pub fn show(&self) { unsafe { ffi::widget_show(self.widget_ptr()) } }
    // ... every common QWidget op, from hand-written bridge #1
}
unsafe impl Signal0 for DComboBox { ... }
unsafe impl SignalI32 for DComboBox { ... }
```

`DComboBox` itself has no `Drop`; the unique `Owned<DComboBox>` constructor
result queues Qt deletion on drop only if it has never been adopted by Qt.

### 4. Call chain at runtime

```rust
let combo = DComboBox::new();
// → genffi::gen_d_combo_box_new()     cxx ABI glue
//   → dtkrs::gen_d_combo_box_new()   C++ shim
//     → new DComboBox                real DTK object

combo.show();
// → ffi::widget_show(self.widget_ptr())   hand bridge #1 (cached base)
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
| value types (`QColor`, `QFont`, `QPixmap`, `QImage`, `QBrush`, `QPainterPath`, `QKeySequence`, `QUrl`, `QIcon`, `QPalette`, `DPalette`, `QSize`, `QPoint`, `QRect`, `QMargins`, `QVariant`, `QVariantList`, `DDciIcon`) | owning heap pointer | wrapper with `Drop` → shim `*_delete` |
| `QWidget*` / `QLayout*` / `QBoxLayout*` / `QAction*` / `QAbstractItemModel*` / DTK class pointers | opaque pointer | guarded non-owning QObject view |
| `QHash<QString, QString>` (out) | owned alternating UTF-8 key/value strings | `BTreeMap<String, String>` (no delimiter or Qt borrow) |
| other `QList<T>`/`QMap`, `QPrinter*`, … | — | **unmapped → skipped** (GEN_REPORT.md, TODO.md) |

Four bridges, one namespace: bridge #1 (hand, core) and #2-4 (generated
DWidget/DCore/DGui) declare opaque types in `namespace dtkrs`. `QWidget*` and
selected value types cross bridges via `as _` pointer casts. Sound because
cxx opaque types are zero-sized Rust structs never dereferenced or moved in
Rust, and each bridge names the same C++ type.
The audit is recorded in tools/gen.py comments. DCore/DGui generation is
currently **pilot scope**: `DStandardPaths`, `DSysInfo`, `DConfig`,
`DGuiApplicationHelper`, `DImageHandler` and `DFontManager` are scanned (see
`GEN_CORE_REPORT.md` / `GEN_GUI_REPORT.md`). DStandardPaths and DSysInfo
provide static functions usable without QApplication. DConfig has an audited
`create`/`createGeneric` ownership transfer, exposed as hand-written
`Option<Owned<DConfig>>` factories on either a scoped QCoreApplication or
GUI DApplication's owning thread; native
`setAppId` must instead be invoked before QCoreApplication. DGui classes return
guarded QObject views and requires the application's GUI thread. New-domain
core XDG/DSG and Qt StandardLocation/LocateOption plus GUI ColorType/SizeMode/
SingleScope inputs use generated Rust enums. Their numeric values come from
safe C++ getters compiled against installed headers, never guessed in Rust;
other raw `i32` C++ enum/flags inputs require `unsafe` until similarly typed.

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
Rust:  5. dtk_cb0(id) queues owned notification data
         6. after native call + return wrapping, or private event pump:
            remove callback -> call -> reinsert unless cancelled
```

Details that matter:

- The string-based `connect(sender, SIGNAL, relay, SLOT)` form is used because
  Qt6 dropped the unregistered-`QMetaMethod` + lambda connect overloads.
- The relay object is parented to the sender → destroyed with it automatically.
- `relay.h` contains `Q_OBJECT`, so `build.rs` runs **moc** on it.
- The registry is `thread_local` — no locks; callbacks only ever fire on the Qt
  main thread.
- Dispatch is **remove-then-call-then-reinsert**, so a callback may safely
  register new callbacks (no RefCell double-borrow). Nested emissions of the
  same 0/int/bool/int,int signal are queued with copied arguments and delivered
  after its outer `FnMut` invocation returns; cancellation/destroy drops them.
  One-shot callbacks remain one-shot, and painter/guard callbacks cannot be
  replayed from an expired native pointer or return value.
- QSocketNotifier readiness is level-triggered, not copied event history.
  The relay coalesces its activation (including the legacy int fd) while
  that id is pending or running (including nested loops) and pauses native
  polling until delivery finishes, preventing cached readiness from causing a
  second read after the fd was drained. An explicit disable remains disabled
  on completion. Other signals retain FIFO copied arguments.
- `dtk::unregister_callback(id)` removes the closure and disconnects its relay
  **on the Qt thread where it was registered**; calling it on a worker for a GUI
  id returns false without accessing Qt's GUI-thread-only relay registry.
  Destruction of the Qt owner cancels its Rust callback immediately; capture
  destruction is retired to a safe Rust boundary outside the QObject destructor.
  A cleanup boundary prevents recursive flushing before that Drop returns.
  Callback ids currently being dispatched are cancelled without resurrection.
  One-shot timers use `FnOnce` (no extra `Rc<Cell>`), including panic cleanup.
- Signals with other argument types (e.g. `checkedChanged(bool)`) can still be
  connected via the arg-less relay — the argument is dropped; read the state
  from the widget inside the closure.

## Event overrides and custom painting

Signals can't intercept events. For those, shim.cpp subclasses DTK/Qt classes
and forwards to Rust callback ids (same registry):

| C++ subclass | Overrides | Rust entry point |
|---|---|---|
| `ScopedDApplication : DApplication` (application_runtime.cpp) | `event()` — `QEvent::Quit` asks a guard; `false` swallows the quit | `DApplication::with_quit_guard(name, guard, f)` |
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

### Runtime native borrowing and shared views

Every generated operation and hand-written QObject operation holds a
`dtk_sys::NativeCall` through argument conversion, native execution and return
wrapping. Both managed application `notify` implementations also bracket native
main-thread event frames. Ordinary signals, show and owned input events dispatch
only after those frames return; same-id nested `FnMut` events remain FIFO.

No second ordinary QObject operation may enter an outstanding transaction,
including reentry from custom argument conversion. Painter/ModelIndex and
close/quit return-value callbacks instead use a synchronous borrow: QObject
access and nested loops reject, while callback-local drawing and independent Qt
values remain available. A deeper native/modal loop admits owned notifications
but only audited one-shot timer, quit and dialog-close controls can enter Qt.
DeferredDelete events are held with their original Qt loop/scope state until
native frames return; deleting a modal dialog/ancestor unwinds the dialog first.
Physical QObject deletion and capture Drop cannot invalidate an active Rust
callback or revive its id. Opaque unique deletion is likewise retired while
native code can retain its allocation.

Clones are non-owning runtime views, never an exclusive/pinned C++ reference.
Neither `Owned<T>` nor `OwnedOpaque<T>` exposes `DerefMut`; opaque output APIs
borrow `&mut OwnedOpaque<T>` rather than a replaceable raw-pointer view. There
is no conversion from `&self` into `Pin<&mut NativeQObject>`.

Source authority, per-API repairs (including retained anchor/animation, proxy,
layout and group dependencies), observable restrictions and native regressions
are in [RUNTIME_SAFETY_AUDIT.md](RUNTIME_SAFETY_AUDIT.md). The four generated
`OWNERSHIP_*_REPORT.md` inventories cover every mapped return and pointer input;
new pointer contracts fail generation until reviewed.

### Scoped GUI and headless applications

`DApplication::with` / `with_quit_guard` and `QCoreApplication::with`
(hand-written `dtk/src/application.rs`) lend non-Clone scope tokens from one
private native `ApplicationRuntime` owner (`application_runtime.cpp`). No
public owner can be moved into a static callback, returned, or forgotten.
Native application deletion occurs only after the outer user closure and all
Qt `exec`/signal/event/FFI frames have returned, including ordinary Rust panic
unwinding. Callback panics remain caught at FFI. All applications require the
Linux process main thread; nested application creation and recursive `exec`
are rejected, including partially constructed or closing runtimes. Each scope
owns fresh writable argc/argv storage until native application deletion.

`Owned::new` registers only verified constructor/transfer QObjects in a
QPointer registry, **without assigning QObject parents** or taking ownership
of arbitrary borrowed getters. Binding-owned relays/shortcuts/actions/event
helpers also register so even helpers parented to surviving DTK singletons
are reclaimed. Normal native deletion promptly removes registry entries.
The individually audited DFeatureItem handoff expresses its existing native
ownership with a QObject parent; this is not a blanket reparenting policy.

Shutdown has this order, also when `exec` never ran or already returned:

1. Permanently set the scope's shared closing state. All GUI/Core guards,
   including views of process-global singletons, now reject native access;
   object/application creation and static application operations reject reentry.
2. Mark the Rust callback scope closing, block dispatch/registration, remove
   its ids, disconnect relays and drop captures outside registry borrows.
   Panicking capture destructors are caught; unregister reentry is safe.
3. Acknowledge already accepted native configuration work with a FIFO
   worker-affinity MetaCall stop marker, then join DTK's borrowed
   `DConfig::globalThread()` while the app is still alive. Calling `quit()`
   directly could leave accepted config updates in its queue until the next
   application; the marker invokes `quit()` only after preceding work returns. DTK's Q_GLOBAL_STATIC `DConfigThread` is a
   normal QThread, not Qt's QDaemonThread: Qt6 worker events call the app's
   virtual `notify()`. Native worker frames must return before deleting the
   app. The singleton allocation is not ours to delete; restart it only after
   a subsequent GUI/Core application is fully constructed. DTK's generated
   configuration constructor warns if the thread is paused but still queues
   its initialization (no early return); that warning on repeat construction
   is expected, and the queued work starts only once the application is ready.
   Qt's own daemon threads (e.g. DBus) are not stopped by the binding.
4. Synchronously delete GUI top-level windows (including native-created
   dialogs/popups), then remaining tracked objects. Snapshots use QPointers:
   parent deletion cannot cause double deletion of children or sibling windows.
   User close/quit guards cannot veto this cleanup.
5. Delete application children and the native application; retain Qt runtime
   and argv until widget cleanup is finished. Release the singleton reservation
   and finish the callback scope only after native teardown returns.

A native shutdown stress test exposed a Qt6 race without step 3:
`QThread::exec -> sendPostedEvents -> QCoreApplication::notifyInternal2` passed
its first global app check, then read a null app pointer after the GUI thread
cleared it. A low-overhead crash trace and the installed Qt disassembly proved
this was worker dispatch, not a QWidget destructor. Qt's `notify()` contract
requires non-Qt library event threads to stop before application destruction.
The worker stop/restart checks include actual worker-affinity task roundtrips,
not merely QThread::isRunning(). A native-only latch fixture blocks event A,
accepts a real DConfig update B, then releases A from a producer only after
shutdown has accepted its FIFO stop marker. B must complete before with
returns, without a GUI loop or sleeps. GUI→Core→GUI and panic cases verify
queue completion and restart. Fixture allocations remain scope-tracked and
are destroyed after the worker joins; no native singleton ownership changes.

No event-loop turn or blanket DeferredDelete drain is required. Only verified
allocations are synchronously deleted: native DTK singleton ownership is not
inferred. In DTK6 6.7.47, DGuiApplicationHelper is reset by a native post-routine,
but DFontManager, DMessageManager, DPaletteHelper and DWaterMarkHelper survive;
old Rust guards still remain dead, and their relays do not survive. In
particular DWaterMarkHelper has Q_GLOBAL_STATIC storage and a post-routine that
queues deleteLater: globally flushing arbitrary deferred deletes would be an
unsafe substitute for ownership auditing. Deleted tracked objects have their
posted events removed by QObject destruction itself.

`exec()` exit does not close the scope; methods remain usable until `with`
returns. Sequential fresh scopes cannot revive old guards even at reused
native addresses. `Owned::drop` for escaped dead handles does nothing.
`DConfig`, `QTimer` and `QSocketNotifier` use QtCore checks, and `GuiSender`
can target either borrowed application token; DWidget/DGui remain GUI-only.
`gui_application_selfcheck` checks native QPointer deletion separately from
guard invalidation, singleton lifetime, callbacks, panic/no-exec and repeat
startup; `core_application_selfcheck` preserves headless regression coverage.

As with other Rust RAII, process::exit/abort/forced termination bypass cleanup;
exit only after with returns. User-owned Rc cycles are not automatically
broken. Independent hand-written Qt values (QVariant/QColor/QFont/QImage etc.)
can still be created/accessed during capture cleanup: they do not reenter the
application or register callbacks. Generated opaque constructors/methods
conservatively reject closing-scope access. Runtime borrow queues and all
ownership/capture retirement state are closed before native scope teardown;
accepted native deferred events are not globally flushed or used as invented
singleton deletion authority.

Three wrapper cases, deliberately different:

1. **Widget/object constructors** (`DComboBox`, `DLabel`, layouts, …) return
   `Owned<T>`; `T` is a non-owning `Clone` view with a `QPointer<QObject>` guard.
   Generated DTK methods and hand-written widget operations check the guard
   before crossing FFI; C++ `static_cast` adjusts QObject/QWidget base pointers
   for generated subclasses once at wrapper construction and caches both base
   pointers. This avoids an extra upcast FFI call on every common QWidget op.
   Qt deletes children with their parent, **not** unparented top-level windows
   when `QApplication` exits. `Owned<T>::drop` queues `deleteLater()` only if
   Qt **never adopted** the QObject; the guard tracks `ParentChange`,
   `ChildAdded` and audited parentless native retention/transfer so
   an original constructor owner cannot delete an object after Qt parents it
   and an audited `take` method returns a new owner. Deletion requires an event
   loop to run while the scope is open. Both GUI and headless scopes also
   synchronously delete their tracked objects at the outer boundary, even after
   loop shutdown and without another event-loop turn.
   `QMainWindow::takeCentralWidget()` transfers ownership; local DTK6
   `DListView::takeHeaderWidget()` does **not**: it returns a still-parented
   widget whose empty header/footer container is scheduled for deletion on the
   next event turn. Never
   infer return ownership from a method name in the generator. Constructors
   require an existing `DApplication` **on its GUI thread** before allocating
   GUI QObjects; `QVariant`
   and `QVariantList` remain usable as QtCore values without it. `QTimer`,
   `QSocketNotifier`, and `PaintDelegate` require an owning QObject parent.
   Parent deletion clears the guard and a later method call panics rather than
   dereferencing the dead pointer; `lifetime_selfcheck`, `owned_selfcheck`, and
   `ownership_transfer_selfcheck` exercise these cases. Runtime transactions,
   synchronous borrows and source-backed retained-pointer repairs protect
   destruction *during* calls, separately from deterministic scope shutdown.
2. **Non-QObject generated constructors** (`DPrinter`, `DStyleOptionButton`,
   `DFileIconProvider`, etc.): return `OwnedOpaque<T>`, which uniquely owns the
   allocation and calls a generated `gen_*_delete` in `Drop`, retired if a
   native frame still borrows it. Audited process-static getter results
   (`global_provider()` / `DFontSizeManager::instance()`) are safe borrowed
   views, never owned allocations. They watch the application's guard, not
   the opaque allocation, and reject access after
   scope exit even when the native singleton survives a later app. Generated
   opaque receivers and pointer arguments use `live_ptr()` too.
   Constructor/copy results use `from_owned_raw` without a borrowed scope guard:
   these independent Rust-owned values keep their unique deleter, including
   after the scope. `create_dialog_setting_info` returns a unique owned derived
   allocation with a virtual deleter, not a borrowed view. QPrinter and blur
   group operations require GUI; independent opaque values keep no-app
   construction but reject closing-scope reentry. The views are not `Copy`/
   `Clone`; arbitrary foreign virtual factories retain an explicit unsafe
   ownership contract rather than manufacturing an owner.
3. **Value-type wrappers** (`QColor`, `QFont`, `QPixmap`, `QImage`, `QBrush`,
   `QPainterPath`, `QKeySequence`, `QUrl`, `QIcon`, `QPalette`, `QSize`,
   `QPoint`, `QRect`, `QMargins`, `QVariant`, `QVariantList`, `DPalette`, `DDciIcon`): own
   a heap copy created by the shim (`new QColor(...)`), freed in `Drop` via
   shim `*_delete`. `WaterMarkData` returned by value or const& is explicitly
   copy-constructed into `OwnedOpaque<WaterMarkData>`; its generated deleter
   runs in `Drop`, not when the Qt widget owning its original data is destroyed.

Object and value wrappers carry `PhantomData<*mut ()>` → `!Send`/`!Sync`;
`OwnedOpaque<T>` inherits its non-Send semantics from its inner view.
`Painter`/`ModelIndex` are temporary raw callback views. `PaintWidgetEvent::Paint`
borrows its `Painter` for the callback lifetime (compile-fail test prevents
storing it); `PaintDelegate` passes borrowed `&Painter`/`&ModelIndex`. Qt widgets
must only be touched from the GUI thread. Callbacks are
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
2. `cxx_build::bridges` for the hand, Widget, DCore and DGui bridges
   (`dtk-cxxqt/build.rs` separately runs `CxxQtBuilder` for the CXX-Qt bridge);
3. compiles `shim.cpp` + `application_runtime.cpp` + `runtime_borrow.cpp` +
   `object_relations.cpp` + `anchor_relations.cpp` + native worker regression
   fixture + `relay.cpp` + the three generated shim .cpp files + moc output with
   `-std=c++17 -fPIC` (Qt6 requires PIC);
4. `pkg-config` probes `Qt6Widgets/Gui/Core` + `dtk6widget/gui/core` for
   include paths, defines, and link flags;
5. `links = "dtk6widget"` in dtk-sys/Cargo.toml.

## Which files are hand-written vs generated

| File | Owner |
|---|---|
| `dtk-sys/src/lib.rs` (bridge #1, callback registry) | hand |
| `dtk-sys/cpp/shim.cpp`, `include/dtk_shim.h` | hand |
| `dtk-sys/cpp/application_runtime.cpp`, `include/application_runtime.h` | hand |
| `dtk-sys/cpp/runtime_borrow.cpp`, `include/runtime_borrow.h`, `src/runtime.rs` | native transactions, queued callbacks and destructor retirement |
| `dtk-sys/cpp/{object,anchor}_relations.cpp`, matching headers | audited retained-pointer/transfer repairs |
| `tools/{ownership,pointer_inputs,hand_contracts,native_rules,audit_hand}.py` | explicit audited ownership contracts and inventory renderer |
| `OWNERSHIP_{WIDGET,CORE,GUI,HAND}_REPORT.md` | generated exhaustive emitted-API inventories |
| `dtk-sys/cpp/runtime_worker_probe.cpp`, `include/runtime_worker_probe.h` | native-only regression fixture, no DTK API coverage |
| `dtk/src/application.rs` (scoped GUI/headless runtimes) | hand |
| `dtk-cxxqt/src/{bridge,lib}.rs`, `dtk-sys/src/native_signal.rs`, `dtk/src/signals.rs` | hand: CXX-Qt imports, typed connection runtime, safe `on_*` API |
| `dtk-sys/cpp/relay.cpp`, `include/relay.h` | hand |
| `dtk-sys/build.rs` | hand |
| `dtk/src/lib.rs` (core classes, macros, signals, qt constants) | hand |
| `dtk-sys/src/{gen,core,gui}_ffi.rs` | **generated** |
| `dtk-sys/cpp/dtk_{gen,core,gui}_shim.cpp`, matching headers | **generated** |
| `dtk/src/{widgets,core,gui}.rs` | **generated** |
| `GEN_REPORT.md`, `GEN_CORE_REPORT.md`, `GEN_GUI_REPORT.md` | generated reports (last two are pilot scopes only) |

Regenerate after DTK upgrades or when adding coverage:

```sh
uv run --no-project python tools/gen.py && cargo build
```

Adding a new DTK method requires its type mapping and explicit pointer/ownership
contract, then regeneration. The current 203 skipped Widget signatures and
pilot-only DCore/DGui coverage mean this is not an exhaustive Qt/DTK binding;
they do not bypass the runtime borrowing rules of exposed safe methods.
Foreign virtual factory ownership and raw sys FFI remain explicit unsafe
boundaries. Hand-written additions (new Qt base-class methods, new value
types, new event overrides) go in shim.cpp + bridge #1.

## CXX-Qt layer (dtk-cxxqt)

`dtk-cxxqt/src/bridge.rs` is an edition-2024 `#[cxx_qt::bridge]` importing
existing DTK classes (`extern "C++Qt"`): typed signals declared `pub(self)`
(connect code only) and `const` getters, all with `self: &T` receivers. It
never constructs, owns or deletes objects, and `Pin<&mut T>`/`&mut T` are
rejected by `tools/audit_hand.py`. `dtk/src/signals.rs` exposes
`on_*` methods returning an RAII `dtk::Connection`; each connects inside a
`NativeCall` from a guarded live pointer. The CXX-Qt functor owns a
thread-checked `dtk_sys::native_signal::SignalToken`, converts arguments to
owned values (`QString` → `String`) in the emission, and queues them as
`SignalEvent::Owned` into the shared callback runtime. Sender destruction
cancels the id and retires the connection guard and capture; scope exit
disconnects remaining ids. cxx, cxx-build and cxx-gen are pinned to one
release because CXX embeds it in C++ symbol names. Worker delivery remains
`GuiSender` (CxxQtThread does not cover imported QObjects, KDAB/cxx-qt#1155).
Evidence, API rules and the migration plan: [CXXQT_INTEGRATION.md](CXXQT_INTEGRATION.md).
