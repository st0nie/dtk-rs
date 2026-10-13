#include "relay.h"
#include "application_runtime.h"
#include "dtk-sys/src/lib.rs.h" // dtk_cb0 / dtk_cb_i32 / dtk_cb_bool

#include <QHash>
#include <QMetaObject>
#include <QSocketNotifier>
#include <QPointer>

namespace dtkrs {

// GUI-thread only; relays self-remove from this map on destruction (sender gone)
static QHash<size_t, DtkRelay *> g_relays;
static size_t g_live_relays = 0;

class NotifierReadiness final : public QObject {
public:
    explicit NotifierReadiness(QSocketNotifier *notifier)
        : QObject(notifier), watched(notifier), enabled(notifier->isEnabled()) {
        application_runtime_track_object(this);
    }
    ~NotifierReadiness() override {
        if (watched) watched->setEnabled(enabled); // never take sender ownership
    }
    QPointer<QSocketNotifier> watched;
    bool enabled;
    size_t pending = 0;
    void refresh() { if (watched) watched->setEnabled(enabled && pending == 0); }
};
static NotifierReadiness *notifierState(QSocketNotifier *notifier) {
    for (auto *child : notifier->children())
        if (auto *state = dynamic_cast<NotifierReadiness *>(child)) return state;
    return new NotifierReadiness(notifier);
}
void DtkRelay::setNotifierEnabled(QSocketNotifier *notifier, bool enabled) {
    auto *state = notifierState(notifier);
    state->enabled = enabled;
    state->refresh();
}
bool DtkRelay::prepareReady() {
    // Raw sys users without a managed runtime retain synchronous delivery;
    // there is no queued ReadyBoundary in that case to resume native polling.
    if (!m_coalesced_ready || !application_is_core()) return true;
    if (m_ready_pending) return false;
    auto *notifier = qobject_cast<QSocketNotifier *>(parent());
    if (!notifier) return false;
    auto *state = notifierState(notifier);
    ++state->pending;
    m_ready_pending = true;
    state->refresh(); // disable level polling before queuing the notification
    return true;
}
void DtkRelay::releaseReady() {
    if (!m_ready_pending) return;
    m_ready_pending = false;
    // The parent may be in QObject destruction (QPointer already null), and
    // scope cleanup may already have destroyed the adjunct. Never recreate it.
    auto *notifier = qobject_cast<QSocketNotifier *>(parent());
    if (!notifier) return;
    for (auto *child : notifier->children()) {
        if (auto *state = dynamic_cast<NotifierReadiness *>(child)) {
            if (state->pending) --state->pending;
            state->refresh();
            return;
        }
    }
}
void DtkRelay::finishReady(size_t cb_id) {
    if (auto *relay = g_relays.value(cb_id)) relay->releaseReady();
}

DtkRelay::DtkRelay(size_t cb_id, QObject *parent, bool coalesced_ready)
    : QObject(parent), m_cb_id(cb_id), m_coalesced_ready(coalesced_ready) {
    ++g_live_relays;
}
size_t DtkRelay::liveCount() { return g_live_relays; }

DtkRelay::~DtkRelay() {
    --g_live_relays;
    releaseReady();
    // Explicit unregister takes the entry first; destruction with the sender
    // must also release the Rust closure (including reentrant destruction).
    if (g_relays.value(m_cb_id) == this) {
        g_relays.remove(m_cb_id);
        dtk_cb_dispose(m_cb_id);
    }
}

static bool connect_impl(QObject *sender, const char *signal, size_t cb_id,
                         const char *slot) {
    const auto signature = QMetaObject::normalizedSignature(signal);
    // Readiness is a level, not a count of independent payloads. Delaying it
    // must not replay an already-drained fd into a second blocking read.
    const bool coalesced_ready = qobject_cast<QSocketNotifier *>(sender)
        && signature.startsWith("activated(");
    auto *relay = new DtkRelay(cb_id, sender, coalesced_ready);
    // The sender can be a process-global borrowed DTK singleton. Reclaim our
    // relay at scope exit even if that singleton survives QApplication.
    application_runtime_track_object(relay);
    QByteArray sig = "2" + signature; // '2' = signal prefix
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
        relay->releaseReady();
        // may be called from inside the relay's own signal activation
        relay->deleteLater();
    }
}

void DtkRelay::fire0() {
    if (!prepareReady()) return;
    if (m_coalesced_ready) dtk_cb_ready(m_cb_id);
    else dtk_cb0(m_cb_id);
}
void DtkRelay::fireI32(int v) {
    if (!prepareReady()) return;
    if (m_coalesced_ready) dtk_cb_ready_i32(m_cb_id, static_cast<int32_t>(v));
    else dtk_cb_i32(m_cb_id, static_cast<int32_t>(v));
}
void DtkRelay::fireBool(bool b) {
    if (!prepareReady()) return;
    if (m_coalesced_ready) dtk_cb_ready_bool(m_cb_id, b);
    else dtk_cb_bool(m_cb_id, b);
}
void DtkRelay::fireI32I32(int a, int b) {
    if (!prepareReady()) return;
    if (m_coalesced_ready) dtk_cb_ready_i32_i32(m_cb_id, static_cast<int32_t>(a), static_cast<int32_t>(b));
    else dtk_cb_i32_i32(m_cb_id, static_cast<int32_t>(a), static_cast<int32_t>(b));
}

} // namespace dtkrs
