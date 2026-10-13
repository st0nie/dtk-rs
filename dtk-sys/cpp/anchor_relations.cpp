#include "anchor_relations.h"
#include <QPointer>
#include <QSet>
#include <algorithm>
#include <vector>

namespace dtkrs {
using Dtk::Widget::DAnchorsBase;
using Dtk::Widget::DEnhancedWidget;
namespace {
enum class Relation { Edge, Fill, Center };
struct Link {
    QPointer<DAnchorsBase> source;
    QWidget *target; // identity only; never dereferenced after destruction
    Relation relation;
    int edge;
};
thread_local std::vector<Link> links;
thread_local QSet<QWidget *> watched;

bool valid_edge(int value) {
    switch (value) {
    case Qt::AnchorLeft: case Qt::AnchorHorizontalCenter: case Qt::AnchorRight:
    case Qt::AnchorTop: case Qt::AnchorVerticalCenter: case Qt::AnchorBottom:
        return true;
    default: return false;
    }
}
void unlink(const Link &link) {
    if (!link.source) return;
    auto *source = link.source.data();
    if (link.relation == Relation::Fill) { source->setFill(static_cast<QWidget *>(nullptr)); return; }
    if (link.relation == Relation::Center) { source->setCenterIn(static_cast<QWidget *>(nullptr)); return; }
    switch (static_cast<Qt::AnchorPoint>(link.edge)) {
    case Qt::AnchorLeft: source->setLeft(static_cast<const Dtk::Widget::DAnchorInfo *>(nullptr)); break;
    case Qt::AnchorHorizontalCenter: source->setHorizontalCenter(static_cast<const Dtk::Widget::DAnchorInfo *>(nullptr)); break;
    case Qt::AnchorRight: source->setRight(static_cast<const Dtk::Widget::DAnchorInfo *>(nullptr)); break;
    case Qt::AnchorTop: source->setTop(static_cast<const Dtk::Widget::DAnchorInfo *>(nullptr)); break;
    case Qt::AnchorVerticalCenter: source->setVerticalCenter(static_cast<const Dtk::Widget::DAnchorInfo *>(nullptr)); break;
    case Qt::AnchorBottom: source->setBottom(static_cast<const Dtk::Widget::DAnchorInfo *>(nullptr)); break;
    }
}
void release_target(QWidget *target) {
    // Remove before emitting native notifications; Rust never runs inline here.
    std::vector<Link> removed;
    links.erase(std::remove_if(links.begin(), links.end(), [&](const Link &link) {
        if (!link.source) return true;
        if (link.target == target) { removed.push_back(link); return true; }
        return false;
    }), links.end());
    for (const auto &link : removed) unlink(link);
}
void watch(QWidget *target) {
    if (watched.contains(target)) return;
    watched.insert(target);
    QObject::connect(target, &QObject::destroyed, [target] {
        // Qt6 QWidget::~QWidget emits destroyed BEFORE deleting children.
        // Anchor-info private storage is still live here; QObject/anchor
        // destroyed would be too late to dereference targetInfo->base.
        release_target(target);
        watched.remove(target);
    });
}
void record(DAnchorsBase *source, QWidget *target, Relation relation, int edge) {
    links.erase(std::remove_if(links.begin(), links.end(), [&](const Link &link) {
        return !link.source || (link.source == source &&
            (relation != Relation::Edge || link.relation != Relation::Edge || link.edge == edge));
    }), links.end());
    links.push_back({source, target, relation, edge});
    watch(target);
}
}

bool anchors_set(QWidget *widget, int32_t edge, QWidget *target, int32_t target_edge) {
    if (!widget || !widget->parentWidget() || !target || !valid_edge(edge) || !valid_edge(target_edge)) return false;
    const bool result = DAnchorsBase::setAnchor(widget, static_cast<Qt::AnchorPoint>(edge), target, static_cast<Qt::AnchorPoint>(target_edge));
    if (result) record(DAnchorsBase::getAnchorBaseByWidget(widget), target, Relation::Edge, edge);
    return result;
}
bool anchors_set_on(DAnchorsBase *source, int32_t edge, QWidget *target, int32_t target_edge) {
    if (!source->target() || !source->target()->parentWidget() || !target || !valid_edge(edge) || !valid_edge(target_edge)) return false;
    const bool result = source->setAnchor(static_cast<Qt::AnchorPoint>(edge), target, static_cast<Qt::AnchorPoint>(target_edge));
    if (result) record(source, target, Relation::Edge, edge);
    return result;
}
bool anchors_fill(DAnchorsBase *source, QWidget *target, bool centered) {
    if (!source->target() || !source->target()->parentWidget() || !target) return false;
    const bool result = centered ? source->setCenterIn(target) : source->setFill(target);
    if (result) record(source, target, centered ? Relation::Center : Relation::Fill, 0);
    return result;
}
void anchors_clear(QWidget *widget) {
    release_target(widget); // unbind dependents BEFORE native target-info deletion
    auto *source = DAnchorsBase::getAnchorBaseByWidget(widget);
    links.erase(std::remove_if(links.begin(), links.end(), [source](const Link &link) {
        return !link.source || link.source == source;
    }), links.end());
    DAnchorsBase::clearAnchors(widget);
}
void anchors_observer_target(DEnhancedWidget *observer, QWidget *target) {
    if (auto *anchors = qobject_cast<DAnchorsBase *>(observer->parent())) {
        // An imported internal observer is NOT a capability to rewrite the
        // widgetMap/shared anchor-private owner or move it to another widget.
        if (anchors->enhancedWidget() == observer) return;
    }
    observer->setTarget(target);
    const QPointer<DEnhancedWidget> guard(observer);
    QObject::connect(target, &QObject::destroyed, observer, [guard, target] {
        if (guard && guard->target() == target) guard->setTarget(nullptr);
    });
}
} // namespace dtkrs
