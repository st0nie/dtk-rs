// Scoped headless QtCore runtime. Rust exposes only a borrowed scope token;
// the native owner is destroyed after user code and all event frames return.
#pragma once
#include "rust/cxx.h"
#include <QObject>
#include <memory>

namespace dtkrs {

// Kept by headless object guards even after their native runtime is destroyed.
// GUI guards have no such state and need no runtime lookup on method calls.
struct CoreRuntimeState {
    bool closing = false;
};

class CoreRuntime {
public:
    CoreRuntime(rust::Str name, rust::Vec<uint8_t> bytes, rust::Vec<size_t> lengths);
    ~CoreRuntime();
    QObject *application() const;
    bool can_exec() const;
    int32_t exec();
    void track(QObject *object);
    bool accepting_objects() const;
    std::shared_ptr<const CoreRuntimeState> guard_state() const;

private:
    struct Impl;
    std::unique_ptr<Impl> impl;
};

std::unique_ptr<CoreRuntime> core_runtime_new(rust::Str name, rust::Vec<uint8_t> bytes,
                                             rust::Vec<size_t> lengths);
QObject *core_runtime_application(const CoreRuntime &runtime);
bool core_runtime_can_exec(const CoreRuntime &runtime);
int32_t core_runtime_exec(CoreRuntime *runtime);
// Only records constructors/verified transfers, NEVER arbitrary borrowed getters.
void core_runtime_track_object(QObject *object);

bool application_is_main_thread();
bool application_can_construct();
bool application_is_core();
// Guards may outlive QApplication; false only during managed Core teardown.
bool core_runtime_accepting_objects();
std::shared_ptr<const CoreRuntimeState> core_runtime_guard_state();

} // namespace dtkrs
