#include "runtime_worker_probe.h"
#include "application_runtime.h"
#include <DConfig>
#include <QCoreApplication>
#include <QHash>
#include <QMetaObject>
#include <QSemaphore>
#include <QThread>
#include <atomic>
#include <memory>
#include <thread>

namespace dtkrs {

struct ConfigWorkerProbe::Impl {
    QSemaphore entered;
    QSemaphore release;
    std::atomic<bool> completed{false};
    std::thread producer;
};

// Exercise a real DConfig setter/value path without relying on a host DBus
// service or writing global/user metadata. Native DConfig owns this backend.
class ProbeBackend : public Dtk::Core::DConfigBackend {
public:
    bool isValid() const override { return true; }
    bool load(const QString &) override { return true; }
    QStringList keyList() const override { return {QStringLiteral("value")}; }
    QVariant value(const QString &key, const QVariant &fallback) const override {
        return values.value(key, fallback);
    }
    void setValue(const QString &key, const QVariant &value) override { values.insert(key, value); }
private:
    QHash<QString, QVariant> values;
};

ConfigWorkerProbe::ConfigWorkerProbe() : impl(std::make_unique<Impl>()) {
    Q_ASSERT(application_is_core());
    auto *thread = Dtk::Core::DConfig::globalThread();
    Q_ASSERT(thread->isRunning());
    auto *gate = new QObject;
    auto *update = new QObject;
    gate->moveToThread(thread);
    update->moveToThread(thread);
    auto *owner_thread = QThread::currentThread();
    auto *state = impl.get();
    QMetaObject::invokeMethod(gate, [gate, state, owner_thread] {
        state->entered.release();
        state->release.acquire(); // current event A is deterministically held
        gate->moveToThread(owner_thread);
        gate->deleteLater(); // app scope tracks the main-affinity helper below
    }, Qt::QueuedConnection);
    // Register before moving back: the scope's QPointer handles subsequent
    // affinity changes; synchronous deletion happens only after worker joins.
    application_runtime_track_object(gate);
    state->entered.acquire();
    // B is already accepted while A is blocked. Direct quit would leave it
    // behind. The runtime's queued stop marker must run AFTER B completes.
    QMetaObject::invokeMethod(update, [update, state, owner_thread] {
        std::unique_ptr<Dtk::Core::DConfig> config(Dtk::Core::DConfig::createGeneric(
            new ProbeBackend, QStringLiteral("dtk-rs-worker-probe"), QString(), nullptr));
        const bool valid = config && config->isValid();
        if (valid)
            config->setValue(QStringLiteral("value"), QVariant(42));
        const bool updated = valid && config->value(QStringLiteral("value")).toInt() == 42;
        config.reset(); // native worker-owned allocation, freed on its thread
        state->completed.store(updated, std::memory_order_release);
        update->moveToThread(owner_thread);
        update->deleteLater();
    }, Qt::QueuedConnection);
    application_runtime_track_object(update);
    state->producer = std::thread([state] {
        // Release A only after shutdown actually accepts its FIFO stop marker.
        // No sleeps/timeouts/scheduler assumptions: B must precede that marker.
        application_runtime_wait_for_worker_stop();
        state->release.release();
    });
}
ConfigWorkerProbe::~ConfigWorkerProbe() { impl->producer.join(); }
bool ConfigWorkerProbe::completed() const { return impl->completed.load(std::memory_order_acquire); }
std::unique_ptr<ConfigWorkerProbe> config_worker_probe_new() {
    return std::make_unique<ConfigWorkerProbe>();
}
bool config_worker_probe_completed(const ConfigWorkerProbe &probe) { return probe.completed(); }

} // namespace dtkrs
