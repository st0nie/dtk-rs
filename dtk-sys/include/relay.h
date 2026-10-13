// generic signal relay: any QObject signal -> Rust callback.
// string-based connect(sender, SIGNAL, relay, SLOT): Qt6 dropped QMetaMethod+lambda connect.
#pragma once

#include <QObject>
#include <cstddef>
#include <QSocketNotifier>

namespace dtkrs {

class DtkRelay : public QObject {
    Q_OBJECT
public:
    explicit DtkRelay(size_t cb_id, QObject *parent, bool coalesced_ready);
    ~DtkRelay() override;

    /// signal looks like "clicked(bool)" / "timeout()"; returns false on failure (qWarning logged)
    static bool connect0(QObject *sender, const char *signal, size_t cb_id);
    static bool connectI32(QObject *sender, const char *signal, size_t cb_id);
    static bool connectBool(QObject *sender, const char *signal, size_t cb_id);
    static bool connectI32I32(QObject *sender, const char *signal, size_t cb_id);
    /// disconnect + deleteLater the relay for cb_id (no-op if unknown)
    static void disconnectId(size_t cb_id);
    // Owning-thread diagnostic, including relays waiting for deleteLater.
    static size_t liveCount();
    static void finishReady(size_t cb_id);
    static void setNotifierEnabled(QSocketNotifier *notifier, bool enabled);

public Q_SLOTS:
    void fire0();
    void fireI32(int v);
    void fireBool(bool b);
    void fireI32I32(int a, int b);

private:
    size_t m_cb_id;
    bool m_coalesced_ready;
    bool m_ready_pending = false;
    bool prepareReady();
    void releaseReady();
};

} // namespace dtkrs
