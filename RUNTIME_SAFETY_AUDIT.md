# Runtime borrowing and native ownership audit

## Source authority and reproducible inventory

The audit uses the **installed package versions**, obtained with `apt-get source`,
not an older checkout, guessed naming conventions, or library disassembly:

```sh
mkdir -p /tmp/dtk-runtime-audit-sources
cd /tmp/dtk-runtime-audit-sources
apt-get source dtkwidget=6.7.47.1-1 dtkcore=6.7.47 dtkgui=6.7.47
```

Qt source is `qt6-base 6.8.0+dfsg-0deepin31`; `pkg-config --modversion Qt6Widgets`
is `6.8.0`. All three installed DTK pkg-config versions are `6.7.47`.
The `.dsc` SHA-256 values identifying the DTK inputs are:

| Source | SHA-256 |
|---|---|
| `dtkwidget_6.7.47.1-1.dsc` | `f35ad3097d80abe67b7be15db9f9bd0ac5cfad3ef132b36123f0ed96461148cb` |
| `dtkcore_6.7.47.dsc` | `5619e46aad12d4f239313c3586d8774a9810d12d4f0fc48234913c1dd59bfa0a` |
| `dtkgui_6.7.47.dsc` | `dee923daeaf5fd1a3e568d27991ee46b5340850463c965eebe6d7a57fe53fc0c` |

The audited public/native boundary includes every currently emitted method,
constructor, mapped pointer parameter, pointer return, value copy, ownership
handoff and callback view in the four bridges. The per-signature inventories are:

- [Widget ownership](OWNERSHIP_WIDGET_REPORT.md): all 146 scanned classes,
  constructors, method returns, pointer contracts and Qt value inputs.
- [DCore ownership](OWNERSHIP_CORE_REPORT.md) and
  [DGui ownership](OWNERSHIP_GUI_REPORT.md): every emitted pilot signature.
- [Hand-written ownership](OWNERSHIP_HAND_REPORT.md): every raw pointer return
  and CXX unique state owner, plus call and retained-parameter contracts.

The generated **201 base-pointer conversion helpers** are additionally audited
as one uniform, verified operation: `gen_*_as_qobject` (QObject ancestry),
`gen_*_as_widget` (QWidget ancestry), and `gen_d_variant_list_model_as_model`.
Every body is exactly a native `static_cast` of its correctly typed receiver;
none allocates, transfers, deletes, calls a virtual method or invokes callbacks.
The original owning-thread guard/transaction covers conversion and wrapper
construction, and later base views share that guard. No Rust offset guess is
used for generated DTK bases. The concrete helper list is reproducible from
`dtk-sys/cpp/dtk_{gen,core,gui}_shim.cpp` and ancestry/upcast generator tests.
Hand Qt primary-base views (QWidget/QBoxLayout/QVBoxLayout/QHBoxLayout) are
verified against the Qt headers; DSearchEdit's non-QLineEdit inheritance was
specifically repaired rather than justified by its guard.

`tools/ownership.py` and `tools/pointer_inputs.py` are explicit reviewed tables;
new mapped pointer contracts fail generation until reviewed. `tools/audit_hand.py`
fails on unknown returns and stale contracts. `tools/native_rules.py` is the
single generator backend for source-specific native repairs. Regeneration and
inventory checks are part of `bash tools/check.sh`; reports are not hand-edited.

## Confirmed failure, not just a hypothetical QPointer gap

A repository-external program using the original safe API reproduced this chain:

```text
DMainWindow::show()
  -> DtkMainWindowEx::showEvent
     -> Rust show callback
        -> window.delete_later()
        -> QMessageBox::exec() / nested event loop
           -> the window is physically deleted (QPointer becomes null)
     -> DMainWindow::showEvent on freed storage
  -> SIGSEGV before the original show() returns
```

Checking the QPointer before `show()` cannot protect this stack. The callback
must not run in the frame it can destroy. The regression is now reproduced with
real Qt modal/event paths in `runtime_borrow_selfcheck`, not a fabricated pointer
or an assertion that merely checks a guard after shutdown.

## Runtime state and proof obligations

### Ordinary native operations

`dtk_sys::NativeCall` brackets each generated method and hand-written QObject
operation. Its lifetime includes argument conversion and **return wrapping**:

```text
reject closing / wrong-thread / incompatible synchronous borrow
  -> check receiver and QObject argument guards
  -> call the native free function
  -> create the result's guarded view, unique owner, or owned value
  -> destroy the native state token
  -> dispatch owned notifications at this Rust boundary
```

`ScopedDApplication::notify` and `ScopedCoreApplication::notify` also bracket
main-thread native event delivery, including native-only handlers. DTK worker
threads do not access the main-thread borrow registry. Scope ownership, worker
FIFO stop/join, and synchronous outer shutdown remain unchanged.

Ordinary signal, show, input, resize and focus callbacks queue copied payloads;
no such callback executes inside its originating signal/event frame. Calls
made directly from Rust drain after their native return and result conversion;
native event-loop delivery uses a private application-owned callback event.
Nested emissions of the same `FnMut` still use the per-id queue: there is never
a second mutable invocation while its first one is on the Rust stack.
Cancellation/owner destruction removes ids immediately, so pending data cannot
resurrect a cancelled closure. Socket-notifier readiness is a special
**level-triggered** notification: the relay coalesces the same id (including
legacy typed `activated(int)` descriptor callbacks) while pending or running,
including an undrained fd in a nested loop. Native polling is paused at the first
activation; completion restores the caller's desired enabled state rather than
blindly reenabling an explicitly disabled notifier. Coalescing alone is not
enough because the native dispatcher can retain an earlier readiness snapshot.
Without this, Qt can queue several activations before the delayed handler drains the fd; the
second handler blocks reading already-consumed data. A source-level stack in
`demo --smoke` confirmed this exact `read` failure; the fix is in the relay and
queue, not a demo-specific nonblocking-fd workaround.

### Synchronous painting and return-value guards

A stack-scoped `QPainter`/`QModelIndex`, close veto or quit-guard return cannot be
queued. `SynchronousBorrow` therefore admits only its callback-local painter/
index operations and independent hand-written Qt values. Safe QObject access,
construction, timer registration and nested native loops reject before entering
Qt; this applies to **all aliases**, not just the apparent receiver.
A deferred destruction request may be recorded but cannot be delivered while
that borrowed frame exists. Nested paint pointers are never replayed.
`NativeCall::paint()` is the single exception inside a synchronous borrow:
generated drawing and style-query operations whose painter/style option
arguments are callback-scoped borrows (QStyle/DStyle draw*/pixelMetric/...,
DBlurEffectGroup::paint, DGraphicsGlowEffect::draw, DStyledIconEngine::paint).
They read widget/style state and draw; none deletes, reparents or runs a loop
(dtkwidget `dstyle.cpp`, `dblureffectwidget.cpp`; Qt `qcommonstyle.cpp`).
Rust-defined `ListModel<T>` hooks run under the same synchronous borrow.

### Suspended native/modal loops

An outstanding native call records its Qt loop level. A deeper loop can deliver
owned callbacks, but ordinary QObject mutation/access through any cloned handle
is rejected while the outer native world remains borrowed. The audited controls
are one-shot timer registration, static quit and dialog close; an arbitrary
widget close is rejected. These controls permit the modal loop to unwind rather
than deadlock while maintaining the receiver/parent/parameter allocation.

Both application `notify` implementations intercept **only main-thread
DeferredDelete events while a native borrow is active**. They retain a clone of
the original event, including Qt's loop/scope level; they do not globally flush
posted events or take ownership of borrowed native singletons. If the receiver
is a visible modal dialog or its ancestor, native `reject()` unwinds its modal
loop without freeing it. Once all native frames return, surviving QPointer
entries get their original deletion event reposted. A simultaneous native
synchronous deletion clears the QPointer, preventing a second deletion.

### Capture/destructor retirement

Qt destruction immediately disconnects/cancels callbacks, but user capture
`Drop` is retired to the safe Rust boundary. It runs outside registry borrows and
before unrelated queued notifications observe the completed cleanup. A cleanup
boundary prevents a Qt call inside a capture destructor from recursively
flushing unrelated callbacks before that destructor returns. If the destructor
explicitly enters a modal loop, owned callbacks can run to unwind it, but their
native operations still obey the suspended-frame control restriction.
Panicking destructors are contained; opaque unique owners move their single deleter into deferred cleanup
while native code may still hold their pointers. Scope closing clears pending
notifications and drains retirement before native object/application teardown.

## Ownership categories and source-backed repairs

| API / dependency | Source finding | Enforced result |
|---|---|---|
| QObject constructors | Verified `new` allocations, not arbitrary getters | `Owned<T>`, scope tracking, constructor deletion provenance |
| QObject/base getters | Existing Qt/native allocation | Guarded `Option<T>`; getter alone grants no destruction or transfer capability |
| `QMainWindow::takeCentralWidget` | Qt detaches and transfers ownership | Fresh `Option<Owned<QWidget>>`; old constructor history cannot reclaim it |
| `DFeatureDisplayDialog::addItem/removeItem` | `m_featureItems` owns without a QObject parent; `removeOne` releases without deleting | Explicit membership, duplicate rejection, native adoption recorded, verified removal returns a fresh owner; no owner for an absent item |
| `DListView::takeHeaderWidget/takeFooterWidget` | Returned widget stays parented; empty header/footer container is deferred-deleted | Borrow only, never an inferred `take` owner; native next-turn invalidation retained |
| `DPrintPreviewDialog::createDialogSettingInfo` | `loadInfo` allocates a derived object; rejected plugin result deleted internally; successful result not deleted by `updateSettingInfo`; virtual base destructor | `Option<OwnedOpaque<DPrintPreviewSettingInfo>>`; typed non-sentinel input; generated matching virtual deleter |
| `DFileIconProvider::globalProvider` | `Q_GLOBAL_STATIC` storage | Safe process-static borrowed view, no deleter, scope invalidation |
| `DFontSizeManager::instance` | Function-static manager storage | Same static-borrow rule; native size array indexing checked against installed `NSizeTypes` |
| `WaterMarkData` and Qt values | Native value or const-reference copied into a new allocation | Rust owns its heap copy; original QObject may die independently |
| `DDrawerGroup` | Raw `expandMap/checkedMap`, unchecked absent-member indexing, no removal disconnect | One guarded membership adjunct; stale native bookkeeping is never populated; removal/destruction disconnects, no drawer ownership inferred; negative ids preserved, only -1 means native automatic assignment |
| `DLoadingIndicator` | `widgetSource` remains raw after scene clear deletes proxy/widget; destructor dereferences it | Guarded derived indicator uses public Qt proxy ownership and QPointer; original broken native field is never populated; replacement and destructor cannot dereference a stale source |
| DTK anchors | Fill/center observers retain raw targets; edge `targetInfo` can outlive the target's anchor-private storage | Explicit dependency bookkeeping; unlink before `clearAnchors`; Qt `QWidget::destroyed` cleanup runs **before child anchor storage destruction**; internal observer is not a retarget capability |
| `DStackWidget` animation | Transition retains raw `oldWidget/newWidget` across event turns | Stop the old animation before push/insert/pop/clear/transition replacement; no later animation tick dereferences deleted widgets |
| Layout handoffs | `DFlowLayout::insertLayout` inserts after `addChildLayout` has rejected a parented child; uninstalled layouts retain raw widget items without setting a widget parent | Reject duplicate/parented/cyclic layout insertion and stolen installed layouts; register widget adoption/retention even before installation; never insert a second raw item for the same widget |
| Titlebar/content/tab/proxy handoffs | Native containers may retain identity outside QObject parent tracking | Constructor/verified-transfer provenance, cycle/duplicate/cross-owner rejection, recorded native owner; no ownership inferred from a getter/setter name |
| `DStyledItemDelegate` default constructor | Body dereferences the default null view parent | Unsafe null-parent constructor is not emitted; explicit report records the required native parent |
| `DFloatingWidget::initStyleOption` | Writes an option through a native non-const pointer | `&mut OwnedOpaque<DStyleOptionFloatingWidget>`; audited value-initialized constructor, no replaceable mutable pointer view |
| `QAbstractItemView` model/delegate inputs | Model is not adopted; Qt stores guarded delegates and resets destroyed model state | Borrowed owning-thread inputs with transaction; model/index mutation cannot reenter a borrowed paint callback |
| Hand DSearchEdit text getter | DSearchEdit inherits the QWidget-container DLineEdit, not QLineEdit | Native checked casts access the actual contained line edit; cached adjusted QWidget bases used for hand generated-subclass access |
| Table items | Native table owns and may replace/delete items | `TableCell` coordinates re-resolved under the same transaction; item pointer cannot escape into safe Rust |

Important primary implementation locations, relative to matching source packages:

- `qt6-base/src/widgets/dialogs/qdialog.cpp`: `QDialog::exec`, `done`, modal exit.
- `qt6-base/src/corelib/kernel/qobject.cpp`: `deleteLater`, DeferredDelete dispatch.
- `qt6-base/src/corelib/kernel/qcoreevent_p.h`: cloned deferred event loop/scope state.
- `qt6-base/src/widgets/kernel/qwidget.cpp`: widget destruction order.
- `qt6-base/src/widgets/kernel/qlayout.cpp`: `addChildLayout/addChildWidget`, native rejection and parent behavior.
- `qt6-base/src/widgets/itemviews/qabstractitemview.cpp` and `_p.h`: model connections and QPointer delegate maps.
- `dtkwidget/src/widgets/{danchors,denhancedwidget,ddrawergroup,dloadingindicator,dflowlayout,dfeaturedisplaydialog,dprintpreviewdialog,dlistview}.cpp`: per-API findings above.

## Shared handles and genuine exclusive outputs

A `Clone` view shares liveness/parent history and a serialized native transaction;
it is **not** an exclusive reference to the native C++ object. No safe `&self`
method creates `&mut QObject`, `Pin<&mut NativeQObject>` or a mutable CXX-Qt
reference. `Owned<T>` deliberately has no `DerefMut`. The CXX-Qt bridge
(`dtk-cxxqt`) declares only `self: &T` receivers (cxx opaque types are
`UnsafeCell`-based, so `&T` asserts no exclusivity) and `tools/audit_hand.py`
rejects `Pin<&mut T>`/`&mut T`; its signal functors queue owned values and
never hand the sender reference to user code.

Neither does `OwnedOpaque<T>` expose `DerefMut<Target=T>`. A mutable *pointer
view* could be replaced with a borrowed singleton, detaching the old view beyond
allocation deletion and causing the owner to delete static storage. Native
output APIs borrow the unique **owner**, whose inner pointer remains private and
stable. Independent value wrappers with ordinary Rust ownership can provide
`&mut Value` when native signatures require it.

The two arbitrary foreign virtual widget factories remain explicitly unsafe:
`DTitleBarToolInterface::createView` and
`DTitleBarSpacerInterface::createPlaceholderView`. Their native headers specify
no universal borrow/transfer contract; no safe factory constructs those foreign
implementations. The boundary requires the external implementation's concrete
contract, not a guessed unique owner. Raw `dtk-sys` pointer calls and foreign
unsafe trait implementations likewise must uphold their stated native contracts;
they cannot bypass those obligations by invoking a safe wrapper afterward.

## Observable API/behavior changes

1. Signals and show/input notifications are delivered after native frames and
   result conversion, not inside the originating Qt call. They still finish
   before a direct Rust operation returns; native event notifications run on the
   private queued event. Same-id nested events retain copied FIFO arguments.
2. Paint, close/quit veto and suspended native loops enforce the borrow policy
   above with explicit rejection rather than allowing a second native borrow.
3. `DStackWidget` cancels its current native transition before a conflicting
   structural mutation; prior transition pointers cannot survive deleting its
   widgets. `delete_later` requires actual constructor/transfer provenance and
   rejects borrowed/native-owned views. Original adopted owners cannot destroy retained
   children; verified constructors of independent timers/notifiers remain
   cancellable. Owned drop never deletes synchronously.
4. Invalid duplicate, cyclic, stolen or already-deleting ownership handoffs are
   rejected before native calls. Uninstalled layouts preserve retained widgets
   through installation or scope cleanup, even when the old constructor owner
   drops; they do not manufacture a take capability.
5. Factory ownership and explicit transfers use the returned owner types above.
   Anchor/print setting enums obtain their values from compiled installed C++
   headers. Reserved C++ double-underscore names and Rust enum naming collisions
   are rejected during generation.

## Regression and validation entry points

- `runtime_borrow_selfcheck`: native signal/call ordering, synchronous native
  child deletion and active-animation cancellation, callback deletion of sender
  and parent, modal loops, clone alias rejection, capture-Drop reentry, paint/index protection, opaque cleanup,
  close veto, coalesced level-triggered fd readiness (before and during a
  callback's nested loop), and physically dead guards while the application
  remains alive.
- `pointer_contract_selfcheck`: borrowed destruction rejection, central/feature
  ownership transfers, old/new owners, missing/duplicate membership, destroyed
  drawers, borrowed widgets, proxy replacement, layout duplicate/cycle/retention,
  native anchor-target deletion, exclusive output and all twelve print factory
  result types, including deletion after the dialog/application ends.
- Compile-fail docs: painter escape, application-token escape, no pinned mutable
  QObject borrow through `Owned<T>`, no mutable replacement of an opaque view.
- Existing ownership, callback, GUI/Core scope, worker, configuration, painting,
  model/value and wrong-thread regressions remain included unchanged in purpose.
  Closing-scope method reentry now reports the transaction's closing rejection
  before the per-object dead-guard diagnostic; the checks verify that ordering.

The complete acceptance command is **`bash tools/check.sh`**: inventory and
regeneration hashes, generator tests, formatting, and Debug **and** Release
workspace/all-targets check, `clippy -D warnings`, build, tests, doctests, every
GUI offscreen/Core headless example, and the existing 32 fresh-process shutdown
stress checks per profile. No local subset substitutes for that command.

## Completion audit and executed evidence (2026-10-13)

Final acceptance ran on Rust `1.99.0`, Cargo `1.99.0`, Qt `6.8.0`,
DTK Widget/Core/Gui `6.7.47`. The exact command was:

```sh
bash tools/check.sh
sha256sum --check /tmp/dtk-runtime-acceptance-inputs-final.sha256
git diff --check
```

Background task `b98219a98` completed with **exit 0**. Its immutable local
record and output are under `.pi/tasks/` (not committed build artifacts).
Output SHA-256:
`e2102ff6163dbd80e83cf07cee0049a9e9f2870b1daf0a5166f3fbae349f8613`.
The input manifest covered **75** implementation, test and generator files;
every hash still matched after both profiles completed. Manifest SHA-256:
`2c34960f6dfd62d29460d4fe64c6642d983a2740f5b1a0b2032a3a9a7e9aff20`.

| Required check | Debug | Release |
|---|---|---|
| Generator unit checks | 27 passed; domain and hand reports regenerated byte-identically | same reproducibility gate |
| Formatting, workspace/all-targets check | pass | pass |
| `clippy --workspace --all-targets -- -D warnings` | pass | pass |
| Workspace/all-targets build and tests | 3 facade + 13 sys tests passed; all targets built | same tests and all targets passed |
| Doctests | 2 positive + 9 compile-fail passed | 2 positive + 9 compile-fail passed |
| Every example, GUI offscreen/Core headless | **31 passed** | **31 passed** |
| Native shutdown, fresh processes | **32 passed** | **32 passed** |

Additional task `b6abd1aa2` completed with **exit 0**: ten fresh executions each
of `runtime_borrow_selfcheck`, `pointer_contract_selfcheck` and the original
blocking-fd `demo --smoke` in each profile (**60 extra processes**). Its output
SHA-256 is
`f1f2ac7e3b41912c1d849233de8b7ba8b41cec1500da006ced8808b7c55222f1`.
This is supplemental stress, not a replacement for the complete gate.

| Explicit delivery requirement | Concrete final evidence |
|---|---|
| All pointer returns, ownership handoffs and copies audited | 44 explicit generated pointer contracts; 65 hand raw returns, CXX state owners and 267 hand pointer parameters; all 1,125 mapped domain methods and constructors in the four inventories; 201 native adjusted-base helpers audited above; matching apt source identifiers; fail-closed generator/audit tests |
| Receiver/parent/argument/dependency destruction during calls | `NativeCall` through arguments/return wrapping, both notify frames, synchronous borrow/held deletes, retired capture/opaque deletion, source-backed anchor/animation/proxy/layout/group repairs; both new native regressions passed in each profile |
| Shared-handle borrowing and no fabricated mutable Pin | no `DerefMut` on either owner; no native mutable-reference conversion from shared views; exclusive owner output; argument-conversion and clone reentry tests; both owner compile-fail tests passed |
| Targeted callbacks, nested loops and complete verification | sender/parent/native child deletion, modal/paint/index/return-value protection, typed and arg-less paused fd readiness, old/fresh ownership transfer; 62 example executions, 64 shutdown checks and supplemental 60 native smoke processes all passed |
| Existing cleanup/threading behavior and honest delivery | GUI/Core scope and worker FIFO tests retained; no blanket deferred-event flush; user changes not discarded; API changes enumerated above; final source/generated diff and input hashes inspected; source-specific failures fixed and complete gate rerun |

The failed intermediate build/lint, capture-retirement ordering and blocking-fd
runs are not completion evidence. In particular, the fd regression's old
independent 10ms completion timer could fire inside the modal loop before the
read finished; its final assertions are causally scheduled after read/disable,
not weakened, slept through, or skipped. Final acceptance uses the corrected
implementation and regression together.
