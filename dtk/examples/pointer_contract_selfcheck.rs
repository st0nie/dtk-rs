//! Audited borrowed/owned/copy results and DTK retained-pointer repairs.
//! QT_QPA_PLATFORM=offscreen cargo run --example pointer_contract_selfcheck
use dtk::widgets::{
    AnchorPoint, DAnchorsBase, DBlurEffectGroup, DBlurEffectWidget, DDialog, DDrawer, DDrawerGroup,
    DFeatureDisplayDialog, DFeatureItem, DFloatingWidget, DFlowLayout, DLoadingIndicator,
    DPrintPreviewDialog, DStyleOptionFloatingWidget, DViewItemAction, PrintSettingType,
};
use dtk::{DApplication, DMainWindow, QPixmap, QTimer, QVBoxLayout, QWidget};
use std::cell::Cell;
use std::rc::Rc;

#[path = "support/expected_panic.rs"]
mod expected_panic;
use expected_panic::expect_panic;

fn provenance_and_transfers() {
    DApplication::with("pointer-transfer-contracts", |app| {
        DApplication::set_quit_on_last_window_closed(false);
        let window = DMainWindow::new();
        window.as_widget().titlebar_add_widget(&window.as_widget()); // reject owner/ancestor cycle
        window.as_widget().titlebar_set_tabbar(&window.as_widget());
        let borrowed = window.titlebar();
        expect_panic("no deletion authority", || borrowed.delete_later());
        let original = QWidget::new(None);
        window.set_central_widget(&original);
        expect_panic("no deletion authority", || original.delete_later());
        let taken = window.take_central_widget().unwrap();
        let stale = original.clone();
        drop(original);

        let dialog = DFeatureDisplayDialog::new();
        let feature = DFeatureItem::new();
        feature.set_name("unique transfer");
        dialog.add_item(&feature);
        dialog.add_item(&feature); // duplicate must not duplicate a native owner
        assert!(!dialog.is_empty());
        expect_panic("no deletion authority", || feature.delete_later());
        let released = dialog
            .remove_item(&feature)
            .expect("verified membership transfer");
        assert!(dialog.is_empty());
        assert!(
            dialog.remove_item(&feature).is_none(),
            "unverified removal created an owner"
        );
        let released_view = released.clone();
        let cleared = DFeatureItem::new();
        dialog.add_item(&cleared);
        dialog.clear_items();
        assert!(
            dialog.remove_item(&cleared).is_none(),
            "pending native deletion became an owner"
        );
        drop(feature); // old owner had permanently relinquished deletion
        let after = Rc::new(Cell::new(false));
        let observed = after.clone();
        QTimer::single_shot(0, move || {
            assert!(stale.is_alive() && taken.is_alive());
            assert_eq!(released.name(), "unique transfer");
            drop(taken);
            drop(released);
            QTimer::single_shot(0, move || {
                assert!(!stale.is_alive() && !released_view.is_alive());
                observed.set(true);
                DApplication::quit();
            });
        });
        assert_eq!(app.exec(), 0);
        assert!(after.get());
    });
}

fn retained_members_and_proxy_replacement() {
    DApplication::with("pointer-retained-contracts", |app| {
        DApplication::set_quit_on_last_window_closed(false);
        let group = DDrawerGroup::new();
        let first = DDrawer::new();
        let second = DDrawer::new();
        first.set_animation_duration(0);
        second.set_animation_duration(0);
        group.add_expand(&first, -2); // only -1 is native automatic-id sentinel
        assert_eq!(group.id(&first), -2);
        group.set_id(&first, 1);
        group.add_expand(&second, 2);
        group.add_expand(&second, 2);
        first.set_expand(true);
        assert_eq!(group.checked_id(), 1);
        second.set_expand(true);
        assert_eq!(group.checked_id(), 2);
        assert!(!first.expand());
        group.set_id(&first, -2);
        assert_eq!(group.id(&first), -2);
        group.remove_expand(&first);
        group.remove_expand(&first); // absent member must not index at -1
        assert_eq!(group.id(&first), -1);
        first.set_expand(true); // removed signal connection must not affect group
        assert_eq!(group.checked_id(), 2);
        second.delete_later();

        let action = DViewItemAction::new();
        let target = QWidget::new(None);
        action.set_widget(&target); // borrowed QPointer, NOT transfer
        assert!(action.widget().is_some());
        target.delete_later();

        let indicator = DLoadingIndicator::new();
        let source = QWidget::new(None);
        indicator.set_widget_source(&source);
        assert!(indicator.widget_source().is_some());
        expect_panic("no deletion authority", || source.delete_later());
        let source_view = source.clone();
        drop(source); // proxy adoption must permanently revoke old RAII deletion
        let next = QWidget::new(None);
        indicator.set_widget_source(&next); // scene clear synchronously deletes old source
        assert!(!source_view.is_alive());
        let next_view = next.clone();
        drop(next);
        let image = QPixmap::new("");
        indicator.set_image_source(&image); // another scene clear; no stale widgetSource
        assert!(!next_view.is_alive() && indicator.widget_source().is_none());
        indicator.delete_later(); // native destructor cannot dereference freed proxy widget

        // Group owns no drawers: destroying a borrower must not reclaim the member.
        let blur = DBlurEffectWidget::new();
        let blur_group = DBlurEffectGroup::new();
        blur_group.add_widget(&blur, &dtk::QPoint::new(0, 0));
        drop(blur_group);
        assert!(blur.is_alive());
        blur.delete_later();
        let completed = Rc::new(Cell::new(false));
        let observed = completed.clone();
        QTimer::single_shot(0, move || {
            assert!(!second.is_alive());
            assert!(group.expand(2).is_none());
            assert!(group.checked_expand().is_none());
            assert_eq!(group.checked_id(), -1);
            assert!(action.widget().is_none());
            assert!(!indicator.is_alive() && !blur.is_alive());
            assert!(first.is_alive());
            observed.set(true);
            DApplication::quit();
        });
        assert_eq!(app.exec(), 0);
        assert!(completed.get());
    });
}

fn duplicate_and_cycle_handoffs() {
    DApplication::with("pointer-handoff-contracts", |app| {
        DApplication::set_quit_on_last_window_closed(false);
        let flow = DFlowLayout::new();
        let child = QVBoxLayout::new(None);
        let child_view = child.as_layout();
        flow.insert_layout(0, &child_view);
        flow.insert_layout(1, &child_view); // native addChildLayout rejects, insertItem did not
        assert_eq!(flow.count(), 1, "same QLayout was inserted twice");
        flow.insert_layout(-1, &child_view);
        assert_eq!(flow.count(), 1);
        flow.delete_later();
        let uninstalled = DFlowLayout::new();
        let pending = QWidget::new(None);
        let preserved = pending.clone();
        uninstalled.insert_widget(0, &pending);
        uninstalled.insert_widget(1, &pending); // no duplicate raw QWidgetItem
        assert_eq!(uninstalled.count(), 1);
        drop(pending); // no QWidget parent yet, but the layout still retains it
        let parent = QWidget::new(None);
        let child = QWidget::new(None);
        child.set_parent(&parent);
        parent.set_parent(&child); // must reject cycle without calling native setParent
        parent.delete_later();
        let checked = Rc::new(Cell::new(false));
        let seen = checked.clone();
        QTimer::single_shot(0, move || {
            assert!(!flow.is_alive() && !child_view.is_alive());
            assert!(preserved.is_alive());
            assert_eq!(uninstalled.count(), 1);
            uninstalled.set_geometry(&dtk::QRect::new(0, 0, 100, 100));
            assert!(!parent.is_alive() && !child.is_alive());
            seen.set(true);
            DApplication::quit();
        });
        assert_eq!(app.exec(), 0);
        assert!(checked.get());
    });
}

fn anchor_dependencies_clear_before_native_deletion() {
    DApplication::with("pointer-anchor-contracts", |_| {
        let parent = QWidget::new(None);
        let source = QWidget::new(Some(&parent));
        let target = QWidget::new(Some(&parent));
        source.resize(80, 20);
        target.resize(80, 20);
        assert!(DAnchorsBase::set_anchor(
            &source,
            AnchorPoint::AnchorLeft,
            &target,
            AnchorPoint::AnchorLeft
        ));
        let anchors = DAnchorsBase::get_anchor_base_by_widget(&source).unwrap();
        let target_anchors = DAnchorsBase::get_anchor_base_by_widget(&target).unwrap();
        DAnchorsBase::clear_anchors(&target); // deletes DAnchorInfo while source stays alive
        assert!(!target_anchors.is_alive());
        source.move_to(1, 1); // stale targetInfo must not be replayed by a native observer
        assert!(anchors.set_fill(&target));
        // The raw fill observer used to survive target destruction. A native
        // dialog clear destroys just that target while the source remains.
        let dialog = DDialog::new();
        dialog.as_widget().set_parent(&parent);
        let other = QWidget::new(None);
        dialog.add_content(&other, dtk::qt::alignment::LEFT);
        assert!(anchors.set_fill(&other));
        dialog.clear_contents(true);
        assert!(!other.is_alive());
        assert!(anchors.fill().is_none());
        source.resize(100, 30);
        assert!(anchors.is_alive());
    });
}

fn factory_unique_allocations_outlive_the_dialog() {
    let info = DApplication::with("pointer-value-contracts", |_| {
        let search = dtk::widgets::DSearchEdit::new();
        assert_eq!(search.text(), ""); // DLineEdit is a container, NOT QLineEdit's base
        let progress = dtk::widgets::DProgressBar::new();
        progress.set_range(0, 100);
        progress.set_value(37);
        assert_eq!(progress.value(), 37);
        let floating = DFloatingWidget::new();
        let mut option = DStyleOptionFloatingWidget::new();
        floating.init_style_option(&mut option); // exclusive allocation output borrow
        let dialog = DPrintPreviewDialog::new();
        let mut first = None;
        for setting in [
            PrintSettingType::PsPrinter,
            PrintSettingType::PsCopies,
            PrintSettingType::PsPageRange,
            PrintSettingType::PsOrientation,
            PrintSettingType::PsPaperSize,
            PrintSettingType::PsPrintDuplex,
            PrintSettingType::PsNUpPrinting,
            PrintSettingType::PsPageOrder,
            PrintSettingType::PsColorMode,
            PrintSettingType::PsPaperMargins,
            PrintSettingType::PsScaling,
            PrintSettingType::PsWatermark,
        ] {
            let info = dialog
                .create_dialog_setting_info(setting)
                .expect("valid typed setting must create independent derived allocation");
            if first.is_none() {
                first = Some(info);
            } else {
                drop(info);
            }
        }
        first.unwrap()
    });
    // Physical allocation ownership is independent of the dialog/application.
    // Its virtual deleter must still run exactly once at this Rust boundary.
    drop(info);
}

fn main() {
    provenance_and_transfers();
    retained_members_and_proxy_replacement();
    duplicate_and_cycle_handoffs();
    anchor_dependencies_clear_before_native_deletion();
    factory_unique_allocations_outlive_the_dialog();
    assert!(dtk_sys::callback_registry_is_empty());
    println!(
        "pointer contract selfcheck ok: borrowed/transfer/heap-copy distinctions, grouping, proxy adoption, duplicate/cycle rejection"
    );
}
