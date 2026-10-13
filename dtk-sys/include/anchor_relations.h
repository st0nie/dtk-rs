// Source-audited DTK anchor/observer relation invalidation before native targets die.
#pragma once
#include "dtk_shim.h"
#include <danchors.h>
#include <denhancedwidget.h>
namespace dtkrs {
bool anchors_set(QWidget *widget, int32_t edge, QWidget *target, int32_t target_edge);
bool anchors_set_on(Dtk::Widget::DAnchorsBase *anchors, int32_t edge, QWidget *target, int32_t target_edge);
bool anchors_fill(Dtk::Widget::DAnchorsBase *anchors, QWidget *target, bool centered);
void anchors_clear(QWidget *widget);
void anchors_observer_target(Dtk::Widget::DEnhancedWidget *observer, QWidget *target);
} // namespace dtkrs
