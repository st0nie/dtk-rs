// Audited adjuncts for DTK APIs that retain raw pointers or transfer without parents.
#pragma once
#include "dtk_shim.h"
#include <DDrawerGroup>
#include <DDrawer>
#include <denhancedwidget.h>
#include <dloadingindicator.h>
#include <dfeaturedisplaydialog.h>
#include <QPixmap>

namespace dtkrs {
bool object_can_adopt(QObject *parent, QObject *child);
bool layout_prepare_widget(QLayout *layout, QWidget *widget);
bool layout_can_install(QWidget *widget, QLayout *layout);
void object_mark_native_adopted(QObject *parent, QObject *child);
void object_release_native_adoption(QObject *child);
void object_note_deletion_requested(QObject *object);
void object_note_constructor(QObject *object);
void observer_set_target(Dtk::Widget::DEnhancedWidget *observer, QWidget *target);
Dtk::Widget::DLoadingIndicator *loading_indicator_new();
void loading_indicator_set_widget(Dtk::Widget::DLoadingIndicator *indicator, QWidget *widget);
void loading_indicator_set_image(Dtk::Widget::DLoadingIndicator *indicator, const QPixmap &image);
QWidget *loading_indicator_widget(Dtk::Widget::DLoadingIndicator *indicator);

Dtk::Widget::DDrawer *drawer_group_checked(Dtk::Widget::DDrawerGroup *group);
Dtk::Widget::DDrawer *drawer_group_find(Dtk::Widget::DDrawerGroup *group, int id);
void drawer_group_add(Dtk::Widget::DDrawerGroup *group, Dtk::Widget::DDrawer *drawer, int id);
void drawer_group_set_id(Dtk::Widget::DDrawerGroup *group, Dtk::Widget::DDrawer *drawer, int id);
void drawer_group_remove(Dtk::Widget::DDrawerGroup *group, Dtk::Widget::DDrawer *drawer);
int drawer_group_checked_id(Dtk::Widget::DDrawerGroup *group);
int drawer_group_id(Dtk::Widget::DDrawerGroup *group, Dtk::Widget::DDrawer *drawer);
void feature_dialog_add(Dtk::Widget::DFeatureDisplayDialog *dialog, Dtk::Widget::DFeatureItem *item);
void feature_dialog_clear(Dtk::Widget::DFeatureDisplayDialog *dialog);
Dtk::Widget::DFeatureItem *feature_dialog_take(Dtk::Widget::DFeatureDisplayDialog *dialog, Dtk::Widget::DFeatureItem *item);
} // namespace dtkrs
