#include "core_application.h"
#include <QCoreApplication>
#include <QPointer>
#include <QThread>
#include <QString>
#include <sys/syscall.h>
#include <unistd.h>
#include <string>
#include <utility>
#include <unordered_map>
#include <vector>

namespace dtkrs {

// The safe API only constructs applications on Linux's process main thread.
// Thread-local state also makes wrong-thread checks avoid our mutable registry.
static thread_local CoreRuntime *active_core_runtime = nullptr;

struct CoreRuntime::Impl {
    std::vector<std::string> arguments;
    std::vector<char *> argv;
    int argc = 0;
    // QCoreApplication stores int& argc and writable argv; own them per scope,
    // not in the GUI singleton's static first-call argv cache.
    std::unique_ptr<QCoreApplication> app;
    std::unordered_map<QObject *, QPointer<QObject>> constructed;
    std::shared_ptr<CoreRuntimeState> state = std::make_shared<CoreRuntimeState>();
    bool executing = false;
};

CoreRuntime::CoreRuntime(rust::Str name, rust::Vec<uint8_t> bytes,
                         rust::Vec<size_t> lengths) : impl(std::make_unique<Impl>()) {
    size_t offset = 0;
    for (size_t length : lengths) {
        if (length > bytes.size() - offset)
            break;
        impl->arguments.emplace_back(length ? reinterpret_cast<const char *>(bytes.data() + offset) : "", length);
        offset += length;
    }
    if (impl->arguments.empty())
        impl->arguments.emplace_back(name.data(), name.size());
    for (auto &arg : impl->arguments)
        impl->argv.push_back(arg.data());
    impl->argv.push_back(nullptr);
    impl->argc = static_cast<int>(impl->arguments.size());
    impl->app = std::make_unique<QCoreApplication>(impl->argc, impl->argv.data());
    impl->app->setApplicationName(QString::fromUtf8(name.data(), static_cast<qsizetype>(name.size())));
    active_core_runtime = this;
}

CoreRuntime::~CoreRuntime() {
    // A Rust Drop for an app moved into an event callback would be unsafe.
    // The with() scope owns us privately, so all native callback/exec frames
    // have returned here (including Rust panic unwinding from ordinary code).
    // Reject constructors and operations on remaining guards before Drop of
    // user captures can reenter the binding from QObject::destroyed.
    impl->state->closing = true;
    // Synchronous cleanup at this outer boundary does not need another event
    // loop turn. QPointers clear if a parent deletes a later tracked child;
    // duplicates or previously deleted objects are harmless. Do not reparent
    // objects to the app just to own them: preserve native parent semantics.
    auto constructed = std::move(impl->constructed);
    impl->constructed.clear();
    for (const auto &entry : constructed) {
        if (entry.second)
            delete entry.second.data();
    }
    // Deletes one-shot timers and their unfired Rust callbacks, even if exec
    // never ran or has already exited. Keep argv alive until after this reset.
    impl->app.reset();
    active_core_runtime = nullptr;
}

QObject *CoreRuntime::application() const { return impl->app.get(); }
bool CoreRuntime::accepting_objects() const { return !impl->state->closing; }
std::shared_ptr<const CoreRuntimeState> CoreRuntime::guard_state() const { return impl->state; }
bool CoreRuntime::can_exec() const {
    return !impl->state->closing && !impl->executing && QThread::currentThread()->loopLevel() == 0;
}
int32_t CoreRuntime::exec() {
    if (!can_exec())
        return -1;
    impl->executing = true;
    const int result = impl->app->exec();
    impl->executing = false;
    return result;
}
void CoreRuntime::track(QObject *object) {
    if (impl->state->closing || !object || object == impl->app.get())
        return;
    if (!impl->constructed.emplace(object, object).second)
        return;
    // Remove normally deleted objects immediately: a long-running service
    // must not accumulate QPointer entries for every past constructor call.
    QObject::connect(object, &QObject::destroyed, [this, object] {
        impl->constructed.erase(object);
    });
}

bool application_is_main_thread() { return ::syscall(SYS_gettid) == ::getpid(); }
bool core_runtime_accepting_objects() {
    return !active_core_runtime || active_core_runtime->accepting_objects();
}
std::shared_ptr<const CoreRuntimeState> core_runtime_guard_state() {
    return active_core_runtime ? active_core_runtime->guard_state() : nullptr;
}
bool application_can_construct() {
    return application_is_main_thread() && !active_core_runtime && !QCoreApplication::instance();
}
bool application_is_core() {
    // Reject a worker BEFORE reading/dereferencing the application's pointer;
    // the main thread may concurrently be tearing down its scoped runtime.
    if (!application_is_main_thread())
        return false;
    auto *app = QCoreApplication::instance();
    return app && QThread::currentThread() == app->thread() && core_runtime_accepting_objects();
}
std::unique_ptr<CoreRuntime> core_runtime_new(rust::Str name, rust::Vec<uint8_t> bytes,
                                             rust::Vec<size_t> lengths) {
    if (!application_can_construct())
        return nullptr;
    return std::make_unique<CoreRuntime>(name, std::move(bytes), std::move(lengths));
}
QObject *core_runtime_application(const CoreRuntime &runtime) { return runtime.application(); }
bool core_runtime_can_exec(const CoreRuntime &runtime) { return runtime.can_exec(); }
int32_t core_runtime_exec(CoreRuntime *runtime) { return runtime->exec(); }
void core_runtime_track_object(QObject *object) {
    if (active_core_runtime && application_is_core())
        active_core_runtime->track(object);
}

} // namespace dtkrs
