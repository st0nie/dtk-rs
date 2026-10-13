#include "runtime_borrow.h"
#include "application_runtime.h"
#include "dtk-sys/src/lib.rs.h"
#include <QApplication>
#include <QCoreApplication>
#include <QDialog>
#include <QPointer>
#include <QThread>
#include <utility>
#include <vector>
#include <algorithm>

namespace dtkrs {
namespace {
thread_local std::vector<NativeCallFrame *> calls;
thread_local unsigned synchronous_borrows = 0;
struct HeldDelete {
    QPointer<QObject> object;
    std::unique_ptr<QEvent> event;
};
thread_local std::vector<HeldDelete> held_deletes;
thread_local QPointer<QObject> callback_pump;
thread_local bool pump_posted = false;
thread_local bool scope_open = false;

QEvent::Type callback_event_type() {
    static const auto type = static_cast<QEvent::Type>(QEvent::registerEventType());
    return type;
}

// A suspended Qt modal/native loop still has the outer receiver/parameters on
// its stack. Default QObject operations cannot reenter that borrowed world.
// The only controls admitted are audited dialog close and internal timers/quit.
bool suspended_call() {
    const int level = QThread::currentThread()->loopLevel();
    for (const auto *call : calls) {
        if (level > call->loop_level)
            return true;
    }
    return false;
}

void release_deletes() {
    if (!calls.empty() || synchronous_borrows)
        return;
    auto pending = std::move(held_deletes);
    held_deletes.clear();
    for (auto &entry : pending) {
        if (entry.object)
            QCoreApplication::postEvent(entry.object.data(), entry.event.release());
    }
}

class CallbackPump : public QObject {
public:
    using QObject::QObject;
    bool event(QEvent *event) override {
        if (event->type() != callback_event_type())
            return QObject::event(event);
        pump_posted = false;
        if (runtime_can_dispatch_callbacks())
            dtk_callbacks_drain();
        // A blocked event is not immediately reposted (no nested-loop spin).
        // NativeCallFrame/SynchronousBorrow exit schedules it once again.
        return true;
    }
};
} // namespace

NativeCallFrame::NativeCallFrame()
    : loop_level(QThread::currentThread()->loopLevel()), registered(scope_open) {
    if (registered)
        calls.push_back(this);
}
NativeCallFrame::~NativeCallFrame() {
    // A low-level state token owns no QObject. It may be dropped out of order
    // or survive a closed scope; never let that reclaim Qt or affect a new scope.
    const auto found = std::find(calls.begin(), calls.end(), this);
    if (!registered || found == calls.end())
        return;
    calls.erase(found);
    release_deletes();
    runtime_schedule_callbacks();
}
SynchronousBorrow::SynchronousBorrow() { ++synchronous_borrows; }
SynchronousBorrow::~SynchronousBorrow() {
    --synchronous_borrows;
    release_deletes();
    runtime_schedule_callbacks();
}

bool runtime_call_allowed(bool control) {
    // Reject a second ordinary operation during ANY native transaction,
    // including user-supplied unsafe-trait argument conversion before the
    // outer FFI starts. Loop-level checks alone would miss that aliasing gap.
    return !synchronous_borrows && (control || calls.empty());
}
std::unique_ptr<NativeCallFrame> runtime_call_new(bool control) {
    return runtime_call_allowed(control) ? std::make_unique<NativeCallFrame>() : nullptr;
}
bool runtime_native_borrow_active() { return !calls.empty() || synchronous_borrows; }
bool runtime_can_dispatch_callbacks() {
    return !synchronous_borrows && (calls.empty() || suspended_call());
}
bool runtime_can_finish_call() { return calls.empty() && !synchronous_borrows; }
bool runtime_can_close(QObject *object) {
    // A modal loop is suspended, but closing its dialog is what unwinds it.
    // Arbitrary QWidget close/reparent/mutation is not admitted here.
    return !suspended_call() || qobject_cast<QDialog *>(object) != nullptr;
}
void runtime_schedule_callbacks() {
    if (!scope_open || !callback_pump || pump_posted)
        return;
    pump_posted = true;
    QCoreApplication::postEvent(callback_pump.data(), new QEvent(callback_event_type()));
}
void runtime_borrow_begin_scope(QObject *application) {
    if (!calls.empty() || synchronous_borrows || !held_deletes.empty() || callback_pump)
        qFatal("dtk-rs: native borrow state survived an application scope");
    scope_open = true;
    pump_posted = false;
    callback_pump = new CallbackPump(application);
}
void runtime_borrow_end_scope() {
    // The application owner is private to with(): all REAL exec/event/FFI
    // frames have returned before this boundary. Invalidate state-only tokens
    // that raw sys users deliberately retained/forgot; they are not Qt owners.
    if (synchronous_borrows)
        qFatal("dtk-rs: application teardown inside a synchronous native callback");
    calls.clear();
    scope_open = false;
    pump_posted = false;
    // Do not discard a borrowed native singleton's accepted deletion request:
    // preserve its original event for native handling, without flushing it or
    // inventing ownership. Tracked allocations are still deleted at the outer
    // boundary; QObject destruction removes their reposted events itself.
    release_deletes();
    if (callback_pump)
        delete callback_pump.data();
    callback_pump.clear();
}

bool runtime_is_callback_event(QObject *receiver, QEvent *event) {
    return callback_pump && receiver == callback_pump.data()
        && event->type() == callback_event_type();
}

bool runtime_hold_deferred_delete(QObject *receiver, QEvent *event) {
    // Qt's notify also runs on non-daemon DTK workers. Do not touch the main
    // thread's registry or QApplication widget list from those worker frames.
    if (!application_is_main_thread() || event->type() != QEvent::DeferredDelete
        || !runtime_native_borrow_active())
        return false;
    held_deletes.push_back({receiver, std::unique_ptr<QEvent>(event->clone())});
    // Deleting a dialog (or its ancestor) must not deadlock its borrowed exec.
    // End the native modal loop without freeing its still-active allocation.
    if (application_is_gui()) {
        std::vector<QPointer<QDialog>> dialogs;
        for (auto *window : QApplication::topLevelWidgets()) {
            auto *dialog = qobject_cast<QDialog *>(window);
            if (!dialog || !dialog->isVisible() || !dialog->isModal())
                continue;
            for (auto *parent = static_cast<QObject *>(dialog); parent; parent = parent->parent()) {
                if (parent == receiver) {
                    dialogs.emplace_back(dialog);
                    break;
                }
            }
        }
        for (const auto &dialog : dialogs) {
            if (dialog)
                dialog->reject();
        }
    }
    return true;
}
} // namespace dtkrs
