#include "object_relations.h"
#include "anchor_relations.h"
#include <QGraphicsProxyWidget>
#include <QGraphicsScene>
#include <QHash>
#include <QMap>
#include <QPointer>
#include <QSet>
#include <algorithm>
#include <vector>
#include <limits>

namespace dtkrs {
using namespace Dtk::Widget;

namespace {
struct ObjectProvenance {
    bool constructor = false;
    bool deleting = false;
    QPointer<QObject> native_owner;
};
thread_local QHash<QObject *, ObjectProvenance> provenance;
ObjectProvenance &provenance_for(QObject *object) {
    if (!provenance.contains(object)) {
        provenance.insert(object, {});
        QObject::connect(object, &QObject::destroyed, [object] { provenance.remove(object); });
    }
    return provenance[object];
}
}
void object_note_constructor(QObject *object) { provenance_for(object).constructor = true; }
void object_note_deletion_requested(QObject *object) { provenance_for(object).deleting = true; }
void object_mark_native_adopted(QObject *parent, QObject *child) {
    provenance_for(child).native_owner = parent;
    guard_mark_adopted(child);
}
void object_release_native_adoption(QObject *child) { provenance_for(child).native_owner.clear(); }

bool object_can_adopt(QObject *parent, QObject *child) {
    if (!parent || !child || !object_can_reparent(child, parent))
        return false;
    if (auto *widget = qobject_cast<QWidget *>(child)) {
        if (widget->graphicsProxyWidget())
            return false; // the proxy is already the native destruction owner
    }
    // A borrowed getter is not a transfer capability, even if its native
    // storage happens to have no QObject parent. Constructor/transfer provenance
    // and a released native-container relation are both required.
    const auto found = provenance.constFind(child);
    if (found == provenance.cend() || !found->constructor || found->deleting || found->native_owner)
        return false;
    if (auto *old = child->parent()) {
        for (auto *ancestor = old; ancestor; ancestor = ancestor->parent()) {
            if (ancestor == parent)
                return true;
        }
        return false;
    }
    return true;
}

bool layout_prepare_widget(QLayout *layout, QWidget *widget) {
    auto *owner = layout->parentWidget() ? static_cast<QObject *>(layout->parentWidget()) : layout;
    if (!object_can_adopt(owner, widget)) return false;
    // An uninstalled layout retains raw QWidgetItem pointers without assigning
    // a QWidget parent. Preserve that live borrow, too; do not enqueue deletion
    // from the historical constructor owner or insert the widget a second time.
    object_mark_native_adopted(layout, widget);
    return true;
}
bool layout_can_install(QWidget *widget, QLayout *layout) {
    return widget->layout() == layout || (!layout->parent() && object_can_adopt(widget, layout));
}

void observer_set_target(DEnhancedWidget *observer, QWidget *target) {
    anchors_observer_target(observer, target);
}

// Native DLoadingIndicator::setWidgetSource stores a raw QWidget* that outlives
// QGraphicsScene::clear() and is dereferenced in its destructor/replacement.
// Do not populate that broken native field. Use Qt's public proxy ownership
// channel plus a QPointer for the getter; preserve setLoadingItem's geometry.
class GuardedLoadingIndicator final : public DLoadingIndicator {
public:
    QPointer<QWidget> source;
};
DLoadingIndicator *loading_indicator_new() { return new GuardedLoadingIndicator; }
static GuardedLoadingIndicator *guarded_indicator(DLoadingIndicator *indicator) {
    return dynamic_cast<GuardedLoadingIndicator *>(indicator);
}
void loading_indicator_set_widget(DLoadingIndicator *indicator, QWidget *widget) {
    auto *guarded = guarded_indicator(indicator);
    if (!guarded || guarded->source == widget)
        return;
    if (!object_can_adopt(indicator, widget) || widget->parentWidget() || widget->graphicsProxyWidget())
        return;
    auto *proxy = new QGraphicsProxyWidget;
    object_mark_native_adopted(indicator, widget); // proxy adoption has no QObject ParentChange
    proxy->setWidget(widget);
    const QSizeF size = proxy->boundingRect().size();
    proxy->setPos((indicator->width() - size.width()) / 2, (indicator->height() - size.height()) / 2);
    proxy->setTransformOriginPoint(size.width() / 2, size.height() / 2);
    indicator->scene()->clear();
    indicator->scene()->addItem(proxy);
    guarded->source = widget;
}
void loading_indicator_set_image(DLoadingIndicator *indicator, const QPixmap &image) {
    if (auto *guarded = guarded_indicator(indicator))
        guarded->source.clear();
    indicator->setImageSource(image);
}
QWidget *loading_indicator_widget(DLoadingIndicator *indicator) {
    if (auto *guarded = guarded_indicator(indicator))
        return guarded->source.data();
    // Foreign native-created indicators are not writable through this helper.
    return indicator->widgetSource();
}

// DTK6 DDrawerGroup keeps raw map entries after drawer destruction and neither
// clears checkedMap nor disconnects in removeExpand(). A small adjunct provides
// the SAME public grouping semantics using guarded membership; native broken
// bookkeeping is never populated. Qt still owns each drawer independently.
class DrawerMembership final : public QObject {
public:
    explicit DrawerMembership(DDrawerGroup *owner) : QObject(owner) {}
    QMap<int, QPointer<DDrawer>> drawers;
    QHash<DDrawer *, QMetaObject::Connection> changes;
    QPointer<DDrawer> checked;
    int id(DDrawer *drawer) const {
        for (auto it = drawers.cbegin(); it != drawers.cend(); ++it)
            if (it.value() == drawer)
                return it.key();
        return -1;
    }
    void remove(DDrawer *drawer) {
        const int key = id(drawer);
        if (key != -1) drawers.remove(key);
        QObject::disconnect(changes.take(drawer));
        if (checked == drawer) checked.clear();
    }
    void choose(DDrawer *drawer, bool expanded) {
        if (!expanded || id(drawer) == -1 || checked == drawer)
            return;
        checked = drawer;
        const auto snapshot = drawers.values();
        for (const auto &other : snapshot)
            if (other && other != drawer && other->expand()) other->setExpand(false);
    }
};
static DrawerMembership *membership(DDrawerGroup *group) {
    for (auto *child : group->children())
        if (auto *existing = dynamic_cast<DrawerMembership *>(child)) return existing;
    return new DrawerMembership(group);
}
DDrawer *drawer_group_checked(DDrawerGroup *group) { return membership(group)->checked.data(); }
DDrawer *drawer_group_find(DDrawerGroup *group, int id) { return membership(group)->drawers.value(id).data(); }
int drawer_group_id(DDrawerGroup *group, DDrawer *drawer) { return membership(group)->id(drawer); }
int drawer_group_checked_id(DDrawerGroup *group) {
    auto *state = membership(group);
    return state->checked ? state->id(state->checked.data()) : -1;
}
void drawer_group_remove(DDrawerGroup *group, DDrawer *drawer) { membership(group)->remove(drawer); }
void drawer_group_add(DDrawerGroup *group, DDrawer *drawer, int id) {
    auto *state = membership(group);
    if (state->id(drawer) != -1) return;
    if (id == -1) {
        const int maximum = state->drawers.isEmpty() ? -1 : std::max(-1, state->drawers.lastKey());
        if (maximum == std::numeric_limits<int>::max()) return;
        id = maximum + 1;
    }
    if (state->drawers.contains(id)) return;
    state->drawers.insert(id, drawer);
    state->changes.insert(drawer, QObject::connect(drawer, &DDrawer::expandChange, state,
        [state, drawer](bool expanded) { state->choose(drawer, expanded); }));
    QObject::connect(drawer, &QObject::destroyed, state, [state, drawer] {
        // QPointer is already null at destroyed(); erase by raw key/identity.
        for (auto it = state->drawers.begin(); it != state->drawers.end();) {
            if (it.value().isNull()) it = state->drawers.erase(it); else ++it;
        }
        state->changes.remove(drawer);
    });
}
void drawer_group_set_id(DDrawerGroup *group, DDrawer *drawer, int id) {
    auto *state = membership(group);
    const int old = state->id(drawer);
    if (old == -1) return;
    if (id == -1) {
        int maximum = -1;
        for (auto it = state->drawers.cbegin(); it != state->drawers.cend(); ++it)
            if (it.key() != old) maximum = std::max(maximum, it.key());
        if (maximum == std::numeric_limits<int>::max()) return;
        id = maximum + 1;
    }
    if (id != old && state->drawers.contains(id)) return;
    state->drawers.remove(old);
    state->drawers.insert(id, drawer);
}

class FeatureMembership final : public QObject {
public:
    explicit FeatureMembership(DFeatureDisplayDialog *owner) : QObject(owner) {}
    QSet<DFeatureItem *> items;
};
static FeatureMembership *features(DFeatureDisplayDialog *dialog) {
    for (auto *child : dialog->children())
        if (auto *existing = dynamic_cast<FeatureMembership *>(child)) return existing;
    return new FeatureMembership(dialog);
}
void feature_dialog_add(DFeatureDisplayDialog *dialog, DFeatureItem *item) {
    auto *state = features(dialog);
    if (state->items.contains(item) || !object_can_adopt(dialog, item)) return;
    // A real native transfer even though upstream does not assign a parent.
    // Add a parent to express that already-established native ownership safely.
    item->setParent(dialog);
    object_mark_native_adopted(dialog, item);
    state->items.insert(item);
    dialog->addItem(item);
    const QPointer<DFeatureDisplayDialog> owner(dialog);
    QObject::connect(item, &QObject::destroyed, state, [state, owner, item] {
        // QObject's QPointer clears before children are destroyed. Never call
        // DFeatureDisplayDialog after its derived/private destructors returned.
        if (state->items.remove(item) && owner) owner->removeItem(item);
    });
}
void feature_dialog_clear(DFeatureDisplayDialog *dialog) {
    // clearItems relinquishes list membership but already queues native item
    // deletion. No subsequent take may turn that pending deletion into an owner.
    features(dialog)->items.clear();
    dialog->clearItems();
}
DFeatureItem *feature_dialog_take(DFeatureDisplayDialog *dialog, DFeatureItem *item) {
    auto *state = features(dialog);
    if (!state->items.remove(item)) return nullptr;
    dialog->removeItem(item);
    item->setParent(nullptr);
    object_release_native_adoption(item);
    return item; // Rust creates a NEW owner after the audited release
}
} // namespace dtkrs
