"""Explicit contracts for every currently mapped native pointer parameter.

Native source: matching apt source packages, as recorded in ownership.py.
The generator refuses new pointer-input APIs without a reviewed entry here.
The runtime column names the enforcement needed in addition to pre-call QPointer.
"""
# class -> method -> (ownership, retained lifetime / native handoff rule)
POINTER_INPUTS = {
    "DAlertControl": {"showAlertMessage": ("borrow", "alert follows widget; native QObject connections/parented alert")},
    "DAnchorsBase": {
        "setAnchor": ("retained borrow", "anchor/observer relations; clearAnchors deletes the observer, not widgets"),
        "clearAnchors": ("borrow; deletes observer", "widget stays owned by its original parent/owner"),
        "getAnchorBaseByWidget": ("borrow", "lookup only"),
        "setFill": ("retained borrow", "fill target is not adopted"),
        "setCenterIn": ("retained borrow", "center target is not adopted"),
    },
    "DArrowRectangle": {"setContent": ("Qt adoption", "QPointer content and installed widget layout")},
    "DBackgroundGroup": {"setLayout": ("Qt adoption", "QWidget::setLayout; only valid layout handoff")},
    "DBaseLine": {
        "setLeftContent": ("Qt adoption", "layout removes QWidgetItems, NOT their widgets, then adopts new content"),
        "setRightContent": ("Qt adoption", "layout removes QWidgetItems, NOT their widgets, then adopts new content"),
    },
    "DBlurEffectGroup": {
        "addWidget": ("retained borrow", "widget destructor removes itself from group; group destructor clears widget group pointers"),
        "removeWidget": ("borrow; releases relation", "removes map entry and clears widget group pointer"),
    },
    "DBoxWidget": {"addWidget": ("Qt adoption", "QBoxLayout owns item and reparents widget to containing widget")},
    "DButtonBox": {
        "setId": ("borrow", "button must already be a member"),
        "id": ("borrow", "QButtonGroup lookup; no transfer"),
    },
    "DDialog": {
        "insertButton": ("Qt adoption", "button layout/parent and buttonList; duplicate membership forbidden"),
        "removeButton": ("borrow; may delete", "removes native list/layout entries; deletion policy is native"),
        "setDefaultButton": ("retained borrow", "must select a live button owned by this dialog"),
        "addContent": ("Qt adoption", "contentList/layout; reject duplicate/cyclic handoffs"),
        "insertContent": ("Qt adoption", "contentList/layout; reject duplicate/cyclic handoffs"),
        "removeContent": ("borrow; optional native delete", "deletes only if requested; native list removal precedes deletion"),
    },
    "DDrawer": {
        "setHeader": ("Qt adoption", "header layout and raw m_header; replacement does not delete old widget"),
        "setContent": ("Qt adoption", "content layout and raw m_content; replacement does not delete old widget"),
    },
    "DDrawerGroup": {
        "addExpand": ("retained borrow", "native raw expandMap/checkedMap; destroyed-member cleanup required"),
        "setId": ("borrow", "member required; avoid duplicate native connections"),
        "removeExpand": ("borrow; releases relation", "native remove does not disconnect/clear checkedMap; shim must repair"),
        "id": ("borrow", "reject absent member before native keys().at(-1)"),
    },
    "DEnhancedWidget": {"setTarget": ("retained borrow", "native raw target; observer does not adopt; clear before target lifetime ends")},
    "DFeatureDisplayDialog": {
        "addItem": ("native ownership transfer without QObject parent", "m_featureItems retains item; dialog destructor/clearItems schedules its deletion; original owner must relinquish deletion"),
        "removeItem": ("ownership transfer back to caller", "removeOne does not delete/reparent item; only verified membership may produce a fresh owner"),
    },
    "DFloatingMessage": {"setWidget": ("Qt adoption", "content layout; deletes old layout item, not widget")},
    "DFloatingWidget": {
        "setWidget": ("Qt adoption", "layout manages content; removes layout items before insertion"),
        "initStyleOption": ("exclusive output borrow", "non-const option pointer is populated; Rust requires &mut OwnedOpaque<T>, not replaceable &mut borrowed T"),
    },
    "DFlowLayout": {
        "insertWidget": ("Qt adoption", "addChildWidget + owned QWidgetItemV2"),
        "insertLayout": ("Qt adoption", "reject already-parented/duplicate/cyclic layout before addChildLayout + insertItem"),
    },
    "DHeaderLine": {"setContent": ("Qt adoption", "delegates to DBaseLine layout handoff")},
    "DLineEdit": {"showAlertMessage": ("borrow", "alert follows widget; no caller ownership transfer")},
    "DListView": {
        "setModel": ("retained borrow", "QAbstractItemView::setModel does not transfer QObject ownership; Qt tracks model destruction"),
        "addHeaderWidget": ("Qt adoption", "view headerList/layout; take leaves widget parented; empty header container's deferred deletion destroys it"),
        "addFooterWidget": ("Qt adoption", "view footerList/layout; take leaves widget parented; empty footer container's deferred deletion destroys it"),
    },
    "DLoadingIndicator": {"setWidgetSource": ("proxy adoption", "QGraphicsProxyWidget shares destruction with embedded widget, not necessarily QObject parenting; raw widgetSource requires repair")},
    "DMessageManager": {
        "sendMessage": ("Qt adoption / borrow parent", "floating message becomes content-layout child; parent argument is borrowed"),
        "setContentMargens": ("borrow", "changes existing content layout; no transfer"),
    },
    "DPaletteHelper": {
        "palette": ("borrow", "returns owned palette copy"),
        "setPalette": ("borrow", "copies palette value into widget/native palette state"),
        "resetPalette": ("borrow", "resets widget palette, no ownership transfer"),
    },
    "DPlatformWindowHandle": {
        "enableDXcbForWindow": ("borrow", "platform property/configuration, not QWidget ownership"),
        "isEnabledDXcb": ("borrow", "platform property lookup"),
    },
    "DPrintPreviewDialog": {"updateDialogSettingInfo": ("borrow", "reads populated derived setting info synchronously; does not retain/delete successful caller result")},
    "DPrintPreviewSettingInterface": {"settingFilter": ("foreign implementation borrow", "virtual plugin may mutate info; caller must uphold foreign implementation contract")},
    "DSimpleListItem": {"sameAs": ("foreign implementation borrow", "abstract QObject item with QPointer view; no safe constructor/getter exists")},
    "DSimpleListView": {"removeItem": ("borrow; native item removal", "abstract QObject item borrow; native removeOne does not transfer/delete ownership; no safe source of foreign concrete items exists")},
    "DSpinBox": {"showAlertMessage": ("borrow", "alert follows follower widget")},
    "DDoubleSpinBox": {"showAlertMessage": ("borrow", "alert follows follower widget")},
    "DStackWidget": {
        "pushWidget": ("Qt adoption", "raw widgetList + parent; reject duplicate widgets"),
        "insertWidget": ("Qt adoption", "raw widgetList + parent; reject duplicate/cyclic handoffs and invalid index"),
        "popWidget": ("borrow; optional native delete", "no transfer to Rust; bound count to remaining stack entries"),
        "indexOf": ("borrow", "lookup only"),
        "setTransition": ("Qt adoption", "parents new transition, schedules old deletion; self replacement must be a no-op"),
    },
    "DStyle": {
        "setFocusRectVisible": ("borrow", "widget style property"),
        "setFrameRadius": ("borrow", "widget style property"),
        "setUncheckedItemIndicatorVisible": ("borrow", "widget style property"),
    },
    "DStylePainter": {"begin": ("callback-scoped device borrow", "QPainter paint device lifetime; this opaque class has no safe constructor/getter")},
    "DViewItemAction": {"setWidget": ("retained borrow", "stores QPointer QWidget and hides it; no transfer")},
    "DStyleOption": {"init": ("borrow", "copies option state; no caller ownership transfer")},
    "DStyleOptionButton": {"init": ("borrow", "copies option state from widget")},
    "DStyleOptionLineEdit": {"init": ("borrow", "copies option state from widget")},
    "DStyleOptionBackgroundGroup": {"init": ("borrow", "copies option state from widget")},
    "DFontSizeManager": {
        "bind": ("retained borrow", "native binder map removes widgets on QObject::destroyed"),
        "unbind": ("borrow; releases relation", "removes widget from all binder lists"),
    },
    "DTabBar": {"setTabButton": ("Qt adoption", "QTabBar owns/replaces tab button widgets")},
    "DToolTip": {
        "setToolTipShowMode": ("borrow", "widget property"),
        "toolTipShowMode": ("borrow", "widget property lookup"),
        "needUpdateToolTip": ("borrow", "widget property lookup"),
        "setShowToolTip": ("borrow", "widget property"),
    },
    "DWaterMarkHelper": {
        "registerWidget": ("retained borrow", "native QObject destruction cleanup; watermark child overlay"),
        "setData": ("value copy", "copies WaterMarkData; does not retain pointer to Rust-owned copy"),
    },
    "DWaterMarkWidget": {"setData": ("value copy", "copies WaterMarkData into native value state")},
}


def pointer_input_contract(class_name, method):
    try:
        return POINTER_INPUTS[class_name][method]
    except KeyError as error:
        raise ValueError(f"unaudited pointer input: {class_name}::{method}") from error
