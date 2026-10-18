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
- [x] Scoped headless QCoreApplication runtime: `with` lends a token that cannot
      enter static callbacks or escape the scope (compile-fail doc tests).
      Scope cleanup releases pending timers/callbacks, destroys registered
      constructor-owned DConfig objects without reparenting, and invalidates
      escaped guards. Tests cover panic/no-exec, repeated/nested scopes,
      wrong-thread/teardown reentry and headless worker-message delivery
      (`core_application_selfcheck`). GUI and Core now share the native scope.
- [x] Audit all exposed pointer returns/inputs, constructor/factory/take
      ownership and value copies against matching apt source. Explicit tables
      and generated `OWNERSHIP_*_REPORT.md` fail on unaudited additions.
      Runtime native transactions include argument conversion, the call and
      guarded/owned result wrapping; event frames participate too. Ordinary
      callbacks dispatch after native return, not after a mere QPointer check.
      Synchronous paint/index/return-value borrows reject QObject reentry;
      suspended modal loops admit only verified unwind controls and hold
      DeferredDelete with its original Qt loop/scope state. See
      `RUNTIME_SAFETY_AUDIT.md` and `runtime_borrow_selfcheck`.
- [x] Split 18 generated non-QObject constructors (including `DPrinter`,
      `DStyleOptionButton`, `DFileIconProvider`) into `OwnedOpaque<T>` with a
      generated C++ deleter in `Drop`; opaque views are not `Copy`/`Clone`.
      Audited process-static `globalProvider`/font-size-manager getters are
      safe borrowed views; print-setting factories return unique owned heap
      storage with a virtual deleter. Opaque views remain neither Clone nor
      Copy; owners do not expose replaceable mutable pointer views.
- [x] Audit each mapped opaque getter/factory individually: explicit source
      ownership/lifetime evidence separates process-static borrows, unique
      constructors/transfers and owned copies. Arbitrary foreign pure-virtual
      widget factories have no universal contract and remain explicit unsafe
      boundaries, not fabricated Rust owners.
- [x] Audit all native pointer-return ownership and handoff APIs individually;
      never infer from method names. Local DTK6 `DListView::takeHeaderWidget()`
      returns a still-parented widget *scheduled for native deletion*, so it
      remains a borrowed `Option<QWidget>`, not `Owned<QWidget>`. The verified
      `QMainWindow::takeCentralWidget` and DFeatureItem release return fresh
      owners. Source-backed repairs enforce retained anchor/proxy/group,
      animation, layout and titlebar dependencies plus duplicate/cycle and
      destruction-provenance rules (`pointer_contract_selfcheck`).
      `QTableWidgetItem*` stays outside the safe API: `TableCell` re-resolves
      (row,col) under one transaction and never exposes the native pointer.
      Pointer-oriented model identity APIs remain a feature-coverage task.
- [x] `PaintWidgetEvent::Paint` now borrows `Painter` for the paint callback
      lifetime; a compile-fail doc test rejects storing it in `'static` state.
      `PaintDelegate` already borrows `&Painter`/`&ModelIndex`.
- [x] Enforce shared-handle alias and destructor/callback boundaries: no
      `Pin<&mut NativeQObject>` or exclusive native reference from `&self`,
      no `DerefMut` on QObject/opaque owners, and exclusive opaque outputs use
      `&mut OwnedOpaque<T>`. Compile-fail docs reject mutable native borrowing
      and opaque pointer-view replacement. Capture Drop retirement and panic/
      closing cleanup remain independently covered by GUI/Core selfchecks.
- [x] Replace GUI's leaked application handle with `DApplication::with` and
      `with_quit_guard`: privately owned singleton/argv, borrowed non-Clone
      tokens, native deletion outside callback/event/exec frames. Shared
      GUI/Core closing state rejects constructor/method/dispatch reentry;
      scope exit frees top-level windows, constructor/transfer-owned objects,
      pending timers, connections and Rust captures without another loop turn.
      Finish accepted native DConfig work with a FIFO worker-affinity stop
      marker, then join its event thread before application deletion (Qt6
      notify/self-pointer race). Keep its borrowed allocation and restart only
      after the next GUI/Core runtime is constructed. A gate fixture verifies
      actual config updates complete before scope return; worker roundtrips
      prove functionality across GUI→Core→GUI rather than only isRunning.
      Borrowed DTK QObject and opaque singleton views expire, even when native
      singletons survive a new application. Preserve Qt parents and adoption
      history; do not globally drain borrowed native DeferredDelete events.
      `gui_application_selfcheck` includes independent native QPointer deletion
      witnesses, no-exec/panic/capture-Drop panic, close veto, singleton relays,
      worker sender cancellation and repeated GUI/Core scopes. Compile-fail
      docs reject token escape/capture/owning clone; sys `guard_selfcheck` checks
      recursive exec rejection and fresh per-scope argv. Run all existing and
      new Debug/Release regressions with `bash tools/check.sh`.

## Qt/DTK feature coverage (see GEN_REPORT.md for per-method detail)

- [x] CXX-Qt first layer: `dtk-cxxqt` (edition 2024) imports DSwitchButton,
      DLineEdit, DSearchEdit, DSlider, DTabBar and DDialog typed signals with
      `&T` receivers only; `dtk::Connection` RAII; `cxxqt_signal_selfcheck`.
      See CXXQT_INTEGRATION.md.
- [x] CXX-Qt phase 2/3: `tools/signals.py` generates the bridge from
      `Q_SIGNALS:` of every wrapped class and its Qt/DTK bases (348 signals,
      84 imported classes); value-type arguments (by-value cxx-qt-lib types,
      owned heap copies, `PersistentModelIndex`, `QList<int>`/`QList<QUrl>`);
      `signal_values_selfcheck`. QColor/QPoint(F)/QSize(F)/QRect(F)/
      QMargins(F)/QUrl are now cxx-qt-lib by-value types everywhere.
- [x] Pointer-valued signals (`QAction*`, `QAbstractButton*`, `QWidget*`) via
      guarded-view conversion with per-signal borrowed-pointer contracts
      (`POINTER_CONTRACTS`, quoted in GEN_SIGNAL_REPORT.md); the QPointer is
      taken in the emission and rejects objects already in ~QObject.
      `QTableWidgetItem*`/`QStandardItem*` become (row, column)/persistent
      indexes; QFlags/scoped enums/enum typedefs become `i32`; model index
      lists become `Vec<PersistentModelIndex>` (`signal_pointer_selfcheck`).
      Still skipped with reasons: non-QObject `DAnchorInfo*`/`DSimpleListItem*`,
      `QObject*`/`QDockWidget*` (no view), synchronous `DPrinter*` paint
      requests, overloaded names, `QTextCharFormat`.
- [ ] Re-evaluate CxxQtThread for imported QObjects when KDAB/cxx-qt#1155
      lands; `GuiSender` stays the worker→GUI path until then.
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
- [ ] Scalar/string containers are mapped: `QList<int>`/`QVector<int>` (`&[i32]`
      in, `Vec<i32>` out), `QVector<quint32>` returns, `QList<QString>` like
      `QStringList` (`container_selfcheck`); overloads that became bindable are
      numbered after existing generated names. Still unsupported: pointer
      element lists (`QList<QWidget*>`, `QList<QAbstractButton*>`), `QList<QUrl>`/
      `QList<QLocale>` parameters, general `QMap` (e.g. `DDrawerGroup::addWidget`,
      `DSimpleListView` item lists). The
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
- [x] Painting/model APIs: `QPainter*` → `&Painter` (callback or
      `Painter::on_image`), `QStyle*` guarded view, owned/borrowed
      `QStyleOption(ViewItem)`, `QModelIndex` → `PersistentModelIndex`,
      `ListModel<T>` (Rust rows, no `Pin<&mut T>`), `PaintDelegate::new_styled`;
      `NativeCall::paint` admits drawing inside paint borrows. 47 previously
      skipped Widget methods emitted (`paint_model_selfcheck`). DStyleHelper/
      DStylePainter/`setFrontRole` stay out (raw retained pointers).
- [ ] Tree models (`ListModel` is flat, one column), proxy models,
      `QStyleOption` subclasses beyond `QStyleOptionViewItem`, `QPaintDevice*`.
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
      `DSysInfo` (28 methods) work without QApplication. DConfig has 11 mapped
      methods; its manually audited, caller-owned create/createGeneric factories
      run on either scoped QCoreApplication or GUI DApplication's owning thread. `setAppId` is unsafe and must precede any
      QCoreApplication. `DGuiApplicationHelper` (30 mapped methods),
      `DImageHandler` (37) and
      `DFontManager` (20) are guarded and require the GUI app thread
      (`core_paths_selfcheck`, `gui_helper_selfcheck`, `gui_image_selfcheck`,
      `gui_font_selfcheck`). `FontSizeType` omits the invalid `NSizeTypes` sentinel.
      Native DFontManager adds a base-font pixel-size offset to per-size getters,
      so a setter/getter only roundtrips exactly after `reset_base_font()`. Core XDG/DSG and Qt StandardLocation/LocateOption,
      plus GUI ColorType/SizeMode/SingleScope inputs use Rust enums whose
      values are read from generated C++ functions. `DPalette` is a value-type
      wrapper, copied on return, deleted on Drop; non-const `DPalette&` inputs
      require `&mut DPalette`. Remaining raw integer enum inputs (`set_mode`,
      `set_attribute`) require `unsafe` pending validation.
- [x] DGui: `DPlatformTheme`/`DNativeSettings` (helper-owned borrowed
      views, `system_theme`/`application_theme`), `DWindowManagerHelper`
      (Q_GLOBAL_STATIC borrow), `DDesktopServices` (typed `SystemSoundEffect`,
      requires the GUI app because natives create a static QDBusInterface)
      (`gui_platform_selfcheck`).
- [ ] Expand DGui further (`/usr/include/dtk6/DGui`): DCI icon players and
      palettes, DIconTheme, DFileDrag, DRegionMonitor, DForeignWindow
      (`QWindow*`/`QList<T*>` mappings needed). `DImageHandler::rotateImage(QImage&)` uses `&mut`
      to preserve the C++ non-const reference requirement.
- [x] DCore: `DSettings` (`from_json`/`from_json_file` caller-owned
      factories, `set_file_backend` with native worker join and backend
      deletion on its thread, null-option `setOption` rejected), `DLogManager`,
      `DFileSystemWatcher`, `DFileWatcher`(+`DBaseFileWatcher` base view),
      `DFileWatcherManager` (`add` returns a borrowed child) —
      `core_extras_selfcheck`. DTK's `DFileWatcher` shares one process-global
      `Q_GLOBAL_STATIC` inotify watcher whose QSocketNotifier stays bound to the
      FIRST application's event dispatcher: it only delivers in the first
      application scope of a process (owned `DFileSystemWatcher` works in
      every scope).
- [ ] Expand DCore further (`/usr/include/dtk6/DCore`): DSettingsOption/
      DSettingsGroup views (`QPointer<T>` returns), DDesktopEntry, DTrashManager,
      DRecentManager, D-Bus helpers etc.
      DConfig, QTimer and QSocketNotifier support scoped, headless QCoreApplication; other
      QtCore QObject classes still need individual lifecycle/thread audits.
      `DConfig::globalThread()` is a native-owned borrowed QThread that must
      never be deleted by Rust.

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
