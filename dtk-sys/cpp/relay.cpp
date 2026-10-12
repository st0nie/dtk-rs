#include "relay.h"
#include "application_runtime.h"
#include "dtk-sys/src/lib.rs.h" // dtk_cb0 / dtk_cb_i32 / dtk_cb_bool

#include <QHash>
#include <QMetaObject>

namespace dtkrs {

// GUI-thread only; relays self-remove from this map on destruction (sender gone)
static QHash<size_t, DtkRelay *> g_relays;
static size_t g_live_relays = 0;

DtkRelay::DtkRelay(size_t cb_id, QObject *parent) : QObject(parent), m_cb_id(cb_id) {
    ++g_live_relays;
}
size_t DtkRelay::liveCount() { return g_live_relays; }

DtkRelay::~DtkRelay() {
    --g_live_relays;
    // Explicit unregister takes the entry first; destruction with the sender
    // must also release the Rust closure (including reentrant destruction).
    if (g_relays.value(m_cb_id) == this) {
        g_relays.remove(m_cb_id);
        dtk_cb_dispose(m_cb_id);
    }
}

static bool connect_impl(QObject *sender, const char *signal, size_t cb_id,
                         const char *slot) {
    auto *relay = new DtkRelay(cb_id, sender); // parented to sender, destroyed with it
    // The sender can be a process-global borrowed DTK singleton. Reclaim our
    // relay at scope exit even if that singleton survives QApplication.
    application_runtime_track_object(relay);
    QByteArray sig = "2" + QMetaObject::normalizedSignature(signal); // '2' = signal prefix
    if (!QObject::connect(sender, sig.constData(), relay, slot)) {
        qWarning("dtkrs: connect signal %s on %s failed", signal,
                 sender->metaObject()->className());
        delete relay;
        return false;
    }
    g_relays.insert(cb_id, relay);
    return true;
}

bool DtkRelay::connect0(QObject *sender, const char *signal, size_t cb_id) {
    return connect_impl(sender, signal, cb_id, SLOT(fire0()));
}

bool DtkRelay::connectI32(QObject *sender, const char *signal, size_t cb_id) {
    return connect_impl(sender, signal, cb_id, SLOT(fireI32(int)));
}

bool DtkRelay::connectBool(QObject *sender, const char *signal, size_t cb_id) {
    return connect_impl(sender, signal, cb_id, SLOT(fireBool(bool)));
}

bool DtkRelay::connectI32I32(QObject *sender, const char *signal, size_t cb_id) {
    return connect_impl(sender, signal, cb_id, SLOT(fireI32I32(int, int)));
}

void DtkRelay::disconnectId(size_t cb_id) {
    if (auto *relay = g_relays.take(cb_id)) {
        relay->disconnect();
        // may be called from inside the relay's own signal activation
        relay->deleteLater();
    }
}

void DtkRelay::fire0() { dtk_cb0(m_cb_id); }
void DtkRelay::fireI32(int v) { dtk_cb_i32(m_cb_id, static_cast<int32_t>(v)); }
void DtkRelay::fireBool(bool b) { dtk_cb_bool(m_cb_id, b); }
void DtkRelay::fireI32I32(int a, int b) {
    dtk_cb_i32_i32(m_cb_id, static_cast<int32_t>(a), static_cast<int32_t>(b));
}

} // namespace dtkrs
