// Scoped QtCore/DTK GUI runtime. Rust lends only a borrowed application token;
// this private native owner dies after user code and native event frames return.
#pragma once
#include "rust/cxx.h"
#include <QObject>
#include <memory>

namespace dtkrs {

// Retained by guards even if the watched QObject is a process-global singleton.
// Closing permanently invalidates every view created in this particular scope.
struct ApplicationRuntimeState {
    bool closing = true;
};

class ApplicationRuntime {
public:
    ApplicationRuntime(rust::Str name, rust::Vec<uint8_t> bytes,
                       rust::Vec<size_t> lengths, bool gui, size_t quit_guard_id);
    ~ApplicationRuntime();
    QObject *application() const;
    bool can_exec() const;
    int32_t exec();
    void track(QObject *object);
    bool accepting_objects() const;
    std::shared_ptr<const ApplicationRuntimeState> guard_state() const;

private:
    struct Impl;
    std::unique_ptr<Impl> impl;
};

std::unique_ptr<ApplicationRuntime> application_runtime_new(
    rust::Str name, rust::Vec<uint8_t> bytes, rust::Vec<size_t> lengths,
    bool gui, size_t quit_guard_id);
QObject *application_runtime_application(const ApplicationRuntime &runtime);
bool application_runtime_can_exec(const ApplicationRuntime &runtime);
int32_t application_runtime_exec(ApplicationRuntime *runtime);
// Only constructors, verified transfers and binding-owned callback helpers;
// never arbitrary borrowed getters or DTK singletons. Native parents unchanged.
void application_runtime_track_object(QObject *object);

bool application_is_main_thread();
bool application_can_construct();
bool application_is_core();
bool application_is_gui();
// Diagnostic only: checks the borrowed DTK worker without creating one.
bool application_runtime_workers_stopped();
bool application_runtime_worker_roundtrip();
// Native fixture latch only; does not access QObject or require a Qt thread.
void application_runtime_wait_for_worker_stop();
std::shared_ptr<const ApplicationRuntimeState> application_runtime_guard_state();

} // namespace dtkrs
