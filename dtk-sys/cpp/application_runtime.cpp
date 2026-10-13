#include "application_runtime.h"
#include "runtime_borrow.h"
#include "dtk_shim.h"
#include "dtk-sys/src/lib.rs.h"
#include <DApplication>
#include <DConfig>
#include <QApplication>
#include <QCoreApplication>
#include <QEvent>
#include <QMetaObject>
#include <QPointer>
#include <QThread>
#include <QString>
#include <sys/syscall.h>
#include <unistd.h>
#include <string>
#include <utility>
#include <unordered_map>
#include <vector>
#include <condition_variable>
#include <mutex>

namespace dtkrs {

// Wrong-thread checks reject workers before reading this registry or Qt's app.
static thread_local ApplicationRuntime *active_runtime = nullptr;

// DTK's process-global DConfigThread is NOT a Qt QDaemonThread. In Qt6 its
// event loop calls the application's virtual notify(), so it must finish all
// frames before application destruction (Qt's documented notify contract).
// Keep only a borrowed pointer: never delete this native-owned singleton.
static QPointer<QThread> native_config_thread;
// A native regression producer can release a blocked event exactly after the
// FIFO stop marker is accepted, rather than racing shutdown using a sleep.
static std::mutex worker_stop_mutex;
static std::condition_variable worker_stop_condition;
static bool worker_stop_queued = false;
void application_runtime_wait_for_worker_stop() {
    std::unique_lock<std::mutex> lock(worker_stop_mutex);
    worker_stop_condition.wait(lock, [] { return worker_stop_queued; });
}

// Serialize after already accepted normal-priority configuration MetaCalls.
// The QThread QObject itself belongs to the main thread, so use a private
// worker-affinity context. Blocking acknowledgement returns only after that
// worker's notify/event frame finishes; move the context back for safe deletion.
static bool acknowledge_config_work(bool stop) {
    if (!native_config_thread || !native_config_thread->isRunning())
        return false;
    const auto worker = native_config_thread;
    auto context = std::make_unique<QObject>();
    auto *owner_thread = QThread::currentThread();
    context->moveToThread(worker);
    const bool accepted = QMetaObject::invokeMethod(context.get(), [context = context.get(), owner_thread, worker, stop] {
        if (stop)
            worker->quit();
        context->moveToThread(owner_thread);
    }, stop ? Qt::QueuedConnection : Qt::BlockingQueuedConnection);
    if (stop && accepted) {
        {
            std::lock_guard<std::mutex> lock(worker_stop_mutex);
            worker_stop_queued = true;
        }
        worker_stop_condition.notify_all();
        if (!worker->wait())
            qFatal("dtk-rs: native configuration worker did not join");
    }
    return accepted;
}

class ScopedDApplication : public Dtk::Widget::DApplication {
public:
    ScopedDApplication(int &argc, char **argv, size_t guard_id)
        : DApplication(argc, argv), quit_guard_id(guard_id) {
        if (guard_id)
            QObject::connect(this, &QObject::destroyed, [guard_id] { dtk_cb_dispose(guard_id); });
    }

    bool notify(QObject *receiver, QEvent *event) override {
        if (!application_is_main_thread() || runtime_is_callback_event(receiver, event))
            return DApplication::notify(receiver, event);
        if (runtime_hold_deferred_delete(receiver, event))
            return true;
        NativeCallFrame frame; // includes native-only event handlers and their nested loops
        if (event->type() == QEvent::ChildAdded)
            guard_observe_parent(static_cast<QChildEvent *>(event)->child());
        return DApplication::notify(receiver, event);
    }
    bool event(QEvent *event) override {
        if (event->type() == QEvent::Quit && quit_guard_id) {
            SynchronousBorrow borrow;
            if (!dtk_cb_guard(quit_guard_id))
                return true;
        }
        return DApplication::event(event);
    }

private:
    size_t quit_guard_id;
};

class ScopedCoreApplication : public QCoreApplication {
public:
    using QCoreApplication::QCoreApplication;
    bool notify(QObject *receiver, QEvent *event) override {
        if (!application_is_main_thread() || runtime_is_callback_event(receiver, event))
            return QCoreApplication::notify(receiver, event);
        if (runtime_hold_deferred_delete(receiver, event))
            return true;
        NativeCallFrame frame;
        if (event->type() == QEvent::ChildAdded)
            guard_observe_parent(static_cast<QChildEvent *>(event)->child());
        return QCoreApplication::notify(receiver, event);
    }
};

struct ApplicationRuntime::Impl {
    std::vector<std::string> arguments;
    std::vector<char *> argv;
    int argc = 0;
    // Qt retains int& argc and writable argv: both live until after app.reset().
    std::unique_ptr<QCoreApplication> app;
    std::unordered_map<QObject *, QPointer<QObject>> constructed;
    std::shared_ptr<ApplicationRuntimeState> state = std::make_shared<ApplicationRuntimeState>();
    bool gui;
    bool executing = false;
};

ApplicationRuntime::ApplicationRuntime(rust::Str name, rust::Vec<uint8_t> bytes,
                                       rust::Vec<size_t> lengths, bool gui, size_t guard_id)
    : impl(std::make_unique<Impl>()) {
    impl->gui = gui;
    {
        std::lock_guard<std::mutex> lock(worker_stop_mutex);
        worker_stop_queued = false;
    }
    size_t offset = 0;
    for (size_t length : lengths) {
        if (length > bytes.size() - offset)
            break;
        impl->arguments.emplace_back(length ? reinterpret_cast<const char *>(bytes.data() + offset) : "", length);
        offset += length;
    }
    if (impl->arguments.empty())
        impl->arguments.emplace_back(name.data(), name.size());
    for (auto &argument : impl->arguments)
        impl->argv.push_back(argument.data());
    impl->argv.push_back(nullptr);
    impl->argc = static_cast<int>(impl->arguments.size());
    // Reserve the singleton before the native constructor too. No nested app or
    // binding object access is allowed during partially constructed native state.
    active_runtime = this;
    dtk_callbacks_begin_scope(guard_id);
    if (gui)
        impl->app = std::make_unique<ScopedDApplication>(impl->argc, impl->argv.data(), guard_id);
    else
        impl->app = std::make_unique<ScopedCoreApplication>(impl->argc, impl->argv.data());
    impl->app->setApplicationName(QString::fromUtf8(name.data(), static_cast<qsizetype>(name.size())));
    if (gui)
        native_config_thread = Dtk::Core::DConfig::globalThread();
    // A preceding GUI scope may have stopped this native singleton. Restart
    // only AFTER the next Core/GUI app is fully constructed, not while Qt's
    // singleton is null or its most-derived vtable is partially constructed.
    if (native_config_thread && !native_config_thread->isRunning())
        native_config_thread->start();
    impl->state->closing = false;
    runtime_borrow_begin_scope(impl->app.get());
}

ApplicationRuntime::~ApplicationRuntime() {
    // with() owns us privately: no user can move/forget this owner or destroy it
    // from a Qt callback. All native exec/event/FFI frames have returned here.
    impl->state->closing = true;
    // Disable dispatch and release Rust captures BEFORE any native destructor
    // can emit signals/focus/paint events. This includes callbacks on surviving
    // borrowed DTK singletons. Capture Drop reentry sees a closing runtime.
    dtk_callbacks_end_scope(); // closes dispatch, then releases borrow/pump state
    if (native_config_thread) {
        // A FIFO stop marker completes accepted config MetaCalls before exit;
        // direct quit() could strand updates until the next application. Join
        // the notify/event stack while the app still exists, without dispatching
        // the GUI loop or deleting the borrowed native singleton allocation.
        if (native_config_thread->isRunning()) {
            if (!acknowledge_config_work(true))
                qFatal("dtk-rs: native configuration stop marker was rejected");
        } else if (!native_config_thread->wait()) {
            // isRunning()==false alone need not mean native thread-local
            // destructors have returned; join even an already finished worker.
            qFatal("dtk-rs: native configuration worker did not join");
        }
    }

    auto constructed = std::move(impl->constructed);
    impl->constructed.clear();
    if (impl->gui) {
        // Qt does not own unparented windows. Include native-created top levels
        // (dialogs/popups), not only Rust constructor results. Snapshot QPointers:
        // deleting a window can also delete other windows in this same list.
        while (!QApplication::topLevelWidgets().isEmpty()) {
            std::vector<QPointer<QWidget>> windows;
            for (auto *window : QApplication::topLevelWidgets())
                windows.emplace_back(window);
            for (const auto &window : windows) {
                if (window)
                    delete window.data();
            }
        }
    }
    // Preserve native parent semantics. QPointers clear if deleting a parent
    // already destroyed a later entry. Explicit delete removes DeferredDelete
    // events too; cleanup works without exec or another event-loop turn.
    for (const auto &entry : constructed) {
        if (entry.second)
            delete entry.second.data();
    }
    // Application-owned timers, notifiers and native children are destroyed
    // while Qt's runtime/argv still exist. Borrowed singletons are NOT deleted.
    impl->app.reset();
    active_runtime = nullptr;
    dtk_callbacks_finish_scope();
}

QObject *ApplicationRuntime::application() const { return impl->app.get(); }
bool ApplicationRuntime::accepting_objects() const { return !impl->state->closing; }
std::shared_ptr<const ApplicationRuntimeState> ApplicationRuntime::guard_state() const { return impl->state; }
bool ApplicationRuntime::can_exec() const {
    return accepting_objects() && !impl->executing && QThread::currentThread()->loopLevel() == 0;
}
int32_t ApplicationRuntime::exec() {
    if (!can_exec())
        return -1;
    impl->executing = true;
    // exec is static, not virtual: calling through QCoreApplication* would
    // bypass QtGui/Widgets' startup/shutdown hooks for the GUI runtime.
    const int result = impl->gui ? QApplication::exec() : QCoreApplication::exec();
    impl->executing = false;
    return result;
}
void ApplicationRuntime::track(QObject *object) {
    if (!accepting_objects() || !object || object == impl->app.get())
        return;
    if (!impl->constructed.emplace(object, object).second)
        return;
    QObject::connect(object, &QObject::destroyed, [this, object] {
        impl->constructed.erase(object);
    });
}

bool application_is_main_thread() { return ::syscall(SYS_gettid) == ::getpid(); }
std::shared_ptr<const ApplicationRuntimeState> application_runtime_guard_state() {
    return active_runtime ? active_runtime->guard_state() : nullptr;
}
bool application_can_construct() {
    return application_is_main_thread() && !active_runtime && !QCoreApplication::instance();
}
bool application_is_core() {
    if (!application_is_main_thread() || !active_runtime || !active_runtime->accepting_objects())
        return false;
    auto *app = QCoreApplication::instance();
    return app && QThread::currentThread() == app->thread();
}
bool application_runtime_worker_roundtrip() {
    return application_is_core() && acknowledge_config_work(false);
}
bool application_runtime_workers_stopped() {
    return application_is_main_thread() && (!native_config_thread || !native_config_thread->isRunning());
}
bool application_is_gui() {
    return application_is_core()
        && qobject_cast<QApplication *>(QCoreApplication::instance()) != nullptr;
}
std::unique_ptr<ApplicationRuntime> application_runtime_new(
    rust::Str name, rust::Vec<uint8_t> bytes, rust::Vec<size_t> lengths,
    bool gui, size_t guard_id) {
    if (!application_can_construct())
        return nullptr;
    return std::make_unique<ApplicationRuntime>(name, std::move(bytes), std::move(lengths), gui, guard_id);
}
QObject *application_runtime_application(const ApplicationRuntime &runtime) { return runtime.application(); }
bool application_runtime_can_exec(const ApplicationRuntime &runtime) { return runtime.can_exec(); }
int32_t application_runtime_exec(ApplicationRuntime *runtime) { return runtime->exec(); }
void application_runtime_track_object(QObject *object) {
    if (application_is_core())
        active_runtime->track(object);
}

} // namespace dtkrs
