# TODO

Known gaps in dtk-rs, roughly in priority order.

## Safety/ownership (blocking a fully safe public API)

- [x] Queue copied arguments for nested emissions of the same 0/int/bool/
      int,int Qt signal until the outer `FnMut` returns; do not drop nested
      events or alias mutable callbacks. Cancellation destroys queued events;
      one-shot callbacks fire once (`signal_reentrancy_selfcheck`, sys tests).
- [x] Fail before entering Qt GUI constructors without `DApplication` **or on
      the wrong thread**, including generated QWidget/QObject, timers, menus,
      clipboard and QPixmap; QtCore `QVariant` still works first
      (`no_application_selfcheck`).
- [x] Replace generated QWidget/QObject and hand core widget/timer/notifier/
      delegate/layout `Copy` raw handles with `Clone` views sharing a Qt QPointer.
      Generated methods, widget operations and signal traits check liveness
      before entering FFI; C++ static_cast adjusts generated base pointers.
      `lifetime_selfcheck` rejects use after parent destruction.
- [x] Constructor results for generated and hand-written QObject/QWidget classes
      now use `Owned<T>`; `owned_selfcheck` verifies deferred deletion for objects
      never adopted by Qt and preservation of those parented later. Original
      owner relinquishes deletion permanently after `ParentChange`;
      `ownership_transfer_selfcheck` verifies ownership returned by
      `QMainWindow::takeCentralWidget` survives dropping the old owner.
- [ ] Finish the remaining lifetime/FFI audit: `DApplication` is not RAII;
      `deleteLater()` cannot guarantee deletion after the event loop stops,
      non-QObject borrowed returns have no liveness guard, and native Qt calls
      may reentrantly delete objects *after* a guard check. A public method
      must not become sound solely because QPointer was checked earlier.
- [x] Split 18 generated non-QObject constructors (including `DPrinter`,
      `DStyleOptionButton`, `DFileIconProvider`) into `OwnedOpaque<T>` with a
      generated C++ deleter in `Drop`; opaque views are not `Copy`/`Clone`.
      Unguarded opaque pointer getters (e.g. `globalProvider()`,
      `createDialogSettingInfo()`) now require `unsafe` because the native
      ownership and lifetime of their return allocations have not been audited.
- [ ] Audit each unsafe opaque pointer getter's native ownership (some may
      return borrowed singletons, others transferred heap storage), model
      ownership correctly, and remove `unsafe` only with lifetime evidence.
- [ ] Audit all native pointer-return ownership and handoff APIs individually;
      never infer from method names. Local DTK6 `DListView::takeHeaderWidget()`
      returns a still-parented widget *scheduled for native deletion*, so it
      remains a borrowed `Option<QWidget>`, not `Owned<QWidget>`. Only the
      verified `QMainWindow::takeCentralWidget` returns `Owned<QWidget>`.
      `QTableWidgetItem*` was removed from the safe API: `TableCell` re-resolves
      (row,col) on every call, but a full item identity/lifetime design is still
      needed for pointer-oriented Qt model APIs.
- [x] `PaintWidgetEvent::Paint` now borrows `Painter` for the paint callback
      lifetime; a compile-fail doc test rejects storing it in `'static` state.
      `PaintDelegate` already borrows `&Painter`/`&ModelIndex`.
- [ ] Audit other borrowed handles and reentrant deletion before claiming Rust
      safety; Qt may destroy the native object *during* an FFI call.
- [ ] Make QApplication creation/destruction a genuine singleton RAII contract,
      including cleanup of unparented objects and event-loop shutdown semantics.
      Offscreen `.pi/app-drop-probe` demonstrated that after dropping Rust
      `DApplication`, a 60-second QTimer still holds its Rust callback and
      `QWidget::new()` still succeeds: native QApplication remains allocated.
      Synchronously deleting QApplication from `Drop` is **not** safe if an
      app owner is moved into a click/signal callback and dropped during its
      C++ event frame. Design scoped/non-reentrant shutdown plus invalidation
      of QApplication-dependent QObject views and deferred-deletion draining.

## Qt/DTK feature coverage (see GEN_REPORT.md for per-method detail)

- [ ] Integrate CXX-Qt where it adds verified value: imported QObject methods,
      typed signals/properties and Rust-defined QObject subclasses, while keeping
      DTK-specific API in this crate. A local DTK `DSwitchButton` probe verified
      `checkedChanged(bool)`, RAII disconnect, and callback cleanup on Qt deletion
      with CXX-Qt 0.10.0 + Qt 6.8. On Rust 1.97 the bridge macro fails in edition
      2024 but compiles in edition 2021. **Do not** convert a current guarded
      `Clone` handle into `Pin<&mut DSwitchButton>` from safe `&self`: that
      fabricates an exclusive borrow and can alias reentrant callbacks even
      though QPointer now detects destruction. Migrate only after a verified
      ownership/borrowing redesign;
      CXX-Qt's Threading helper still does not cover imported QObject types.
- [ ] Build a coherent QObject/QtWidgets API (action, menu, model/view, layout,
      event filters, rendering, properties) rather than only isolated shim calls;
      support the use cases required by arbitrary DTK applications.
- [ ] Add QtCore/Gui modules and typed signals/slots plus cross-bridge object
      hierarchy, not only the currently hand-picked QWidget subset.

- [x] `QVariant` value-type signatures use owned heap copies; DTK tab data
      roundtrips strings (including NUL) and 64-bit integers (`variant_selfcheck`).
- [x] `QVariantList` is an owned, implicitly shared Qt container bound through
      generated `DListView::addItems`/`insertItems` (`variant_selfcheck`).
- [x] Generated null-model guards prevent native DTK `DListView::addItems()`
      from SIGSEGVing inside `count()` before a model is installed; safe Rust
      returns false/0 (`variant_selfcheck` and GDB-verified original crash).
- [x] Reject DListView item mutation when a model has zero columns. Native
      addItems returned true/incremented rows while `(row,0)` remained invalid;
      generated shims now return false (`model_selfcheck`).
- [ ] Other Qt containers `QList<T>`/`QVector<T>`/general `QMap` remain unsupported
      (e.g. `DDrawerGroup::addWidget`, `DSimpleListView` item lists). The
      `QHash<QString,QString>` return of `DImageHandler::findAllMetaData` is
      now an owned `BTreeMap`, but its native contents depend on an optional
      FreeImage backend and may be empty for PNG text. Map other containers only
      when element ownership and QVariant semantics remain intact.
      (`QStringList` maps to `Vec<String>`; `QVariantList` is an owned Qt value.)
- [x] `QLayout*` params use guarded `QLayout` base views; Qt adopts inserted
      child layouts and destroys them with `DFlowLayout` (`layout_selfcheck`).
- [x] `QAction*` maps across both bridges; `QAction::new` requires an owner,
      `DMenu::add_action_object` retains the native QObject parent, and
      `action_selfcheck` verifies signals and callback cleanup on deletion.
- [x] `QAbstractItemModel*` now maps through C++ upcasts from DTK models;
      `model_selfcheck` verifies DListView data and Qt parent ownership.
- [x] `QBoxLayout*` getters return guarded, nullable views; deleting `DBaseLine`
      invalidates its layout and content handles (`layout_selfcheck`).
- [ ] Qt classes beyond `QWidget`/`QAbstractButton`/`QLayout`/`QBoxLayout`/
      `QAction`/`QAbstractItemModel`: `QPrinter*`, ... Declare each in both
      bridges and preserve QObject/ownership semantics (QT_PTRS).
- [x] Multi-line and templated-return signatures: gen.py joins continuations,
      strips `inline`/`Q_SLOT`, and skips ctor init lists/inline bodies. The
      current Widget and pilot reports have zero signature-parse failures;
      remaining skips are explicit unmapped parameter/return types.
- [ ] Non-exported nested classes (`DPrinter` inside dprintpreviewwidget.h etc.):
      parser registers their enums but can't bind the classes themselves.
- [ ] Most Widget `Q_ENUM` values still map to bare `i32` with hand-maintained
      `dtk::qt` constants; new DCore/DGui pilots extract selected enum member
      names and ask compiled C++ for native values. Extend typed enum coverage
      without accepting invalid/sentinel discriminants as safe Rust inputs.
- [ ] Remaining painting/model APIs skipped: `QPainter*`/`QStyle*`/
      `QStyleOption*` and `QModelIndex` params require scope-aware borrowing
      or an appropriate Rust-defined model subclass. `QVariant` and
      `QAbstractItemModel*` already have basic mappings; typed model events,
      proxy models and complete view integration are not yet available.
- [x] `QImage`, `QPainterPath`, `QBrush`, `QKeySequence`, `QUrl` now use
      owning value wrappers; `image_selfcheck` and `value_types_selfcheck`
      roundtrip through image viewer, watermark, clip, shortcut and chooser APIs.
      `WaterMarkData` returned by value/const& becomes an owned C++ copy.
- [x] `QTableWidget::set_horizontal_header_labels` sends a real QStringList;
      `table_header_selfcheck` covers embedded `|`, NUL, Unicode and empty text.
      Out-of-bounds cell setters return false without allocating a Qt item;
      writing UserRole data into an empty valid cell now creates an item,
      and later setting its text preserves the role.
- [ ] Value types still not wrapped: `QEasingCurve`, `QTextOption`, `QRegion`,
      `QTransform`, `QRectF`, etc. Add explicit ownership and roundtrip tests.

## Libraries

- [x] Add generator-backed DCore/DGui pilot modules with separate bridges,
      shims and scoped coverage reports: `DStandardPaths` (13 methods) and
      `DSysInfo` (28 methods) work without QApplication;
      `DGuiApplicationHelper` (29 mapped methods) and `DImageHandler` (37 mapped
      methods) are guarded and require the GUI app thread (`core_paths_selfcheck`,
      `gui_helper_selfcheck`, `gui_image_selfcheck`). Core XDG/DSG and Qt StandardLocation/LocateOption,
      plus GUI ColorType/SizeMode/SingleScope inputs use Rust enums whose
      values are read from generated C++ functions. `DPalette` is a value-type
      wrapper, copied on return, deleted on Drop; non-const `DPalette&` inputs
      require `&mut DPalette`. Remaining raw integer enum inputs (`set_mode`,
      `set_attribute`) require `unsafe` pending validation.
- [ ] Expand DGui bindings beyond `DGuiApplicationHelper`/`DImageHandler`/
      `DPalette` (`/usr/include/dtk6/DGui`): DCI icons, platform themes,
      desktop services etc. `DImageHandler::rotateImage(QImage&)` uses `&mut`
      to preserve the C++ non-const reference requirement.
- [ ] Expand DCore bindings beyond `DStandardPaths`/`DSysInfo` (`/usr/include/dtk6/DCore`):
      DConfig, DSettings, DLog, file watchers etc. QObject wrappers need
      QCoreApplication-aware constructors and thread affinity, not the Widget
      module's GUI-application requirement.

## Hygiene

- [x] Classify generated DTK classes by transitive QWidget/QObject/opaque
      ancestry; never cast an arbitrary Q*/D* subclass to QWidget. Nullable
      pointer getters now use Option<T> instead of panicking on nullptr.
- [x] Hide Linux worker → GUI wakeup machinery behind `GuiSender<T: Send>`;
      coalesce wakeups, bound GUI dispatch and release the receiver with its Qt owner.
- [x] Value-type wrappers (QColor/QFont/QPalette/QPixmap/QSize/QPoint/QRect, QIcon):
      fixed — `Drop` calls a shim `*_delete` fn; no more intentional leak.
- [x] `unregister_callback` records Qt relay ids per thread. A worker call on a
      GUI callback id now returns false without touching the GUI-only C++ QHash;
      GUI disconnect and destruction clean up the id (`signal_reentrancy_selfcheck`).
- [x] Callback registry: fixed — `unregister_cb`/`dtk::unregister_callback(id)` exists,
      `connect_signal*` returns the id, and `QTimer::single_shot` self-cleans.
      Object destruction now releases callbacks from relays, actions, shortcuts,
      paint widgets/delegates and event overrides; in-flight cancellation prevents
      resurrection and unknown ids cannot grow tombstones. One-shots are `FnOnce`
      and clean up on panic/cancellation. This does **not** make raw handles safe.
- [x] `qtptr` cross-bridge cast (`as _`): audited — sound. cxx opaque C++ types are
      zero-sized Rust structs never dereferenced in Rust; both bridge modules name
      the same C++ type (`using ::QWidget;` in namespace dtkrs). The cast moves no
      bits and assumes no layout. Conclusion recorded in tools/gen.py comments.
