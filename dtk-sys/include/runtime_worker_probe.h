// Native-only regression fixture. Not part of the Rust-facing DTK API.
#pragma once
#include <memory>

namespace dtkrs {
class ConfigWorkerProbe {
public:
    ConfigWorkerProbe();
    ~ConfigWorkerProbe();
    bool completed() const;
private:
    struct Impl;
    std::unique_ptr<Impl> impl;
};
std::unique_ptr<ConfigWorkerProbe> config_worker_probe_new();
bool config_worker_probe_completed(const ConfigWorkerProbe &probe);
} // namespace dtkrs
