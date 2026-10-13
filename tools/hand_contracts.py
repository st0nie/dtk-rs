"""Source-audited non-receiver pointer-input contracts for bridge #1.

Receiver-only operations use the same native transaction. Each additional
QObject handoff, callback view, native owner or output is explicit here.
"""
INPUTS = {
    ("runtime_can_close", "object"): "borrowed QObject; only dialog close admitted in suspended native frames",
    ("guard_object", "object"): "live owning-thread QObject borrow; only QPointer/filter state allocated",
    ("guard_widget", "widget"): "live QWidget borrow; adjusted QObject base, never ownership",
    ("object_grant_constructor_authority", "object"): "unsafe verified constructor/transfer-only provenance; not a borrowed getter",
    ("guard_mark_adopted", "object"): "record original-owner relinquishment, never claim native ownership",
    ("object_can_reparent", "child"): "borrow; native ancestor-cycle validation",
    ("object_can_reparent", "parent"): "borrow; native ancestor-cycle validation",
    ("layout_can_adopt", "parent"): "borrow destination; unique unparented layout/provenance required",
    ("layout_can_adopt", "child"): "borrow transfer candidate; cannot duplicate native layout ownership",
    ("application_runtime_track_object", "object"): "unsafe verified allocation-only QPointer registration, no reparenting",
    ("application_runtime_exec", "runtime"): "private application state borrow through with; no public destruction inside exec",
    ("widget_set_parent", "child"): "verified constructor/transfer view; reject cycle, retained cross-owner theft and pending deletion",
    ("widget_set_parent", "parent"): "borrow destination; Qt adopts child, permanent parent history",
    ("main_window_titlebar_set_tabbar", "w"): "borrow main window; native qobject_cast checked",
    ("main_window_titlebar_set_tabbar", "tabbar"): "Qt/native-container adoption; constructor provenance + ancestor/cross-owner rejection",
    ("main_window_titlebar_add_widget", "w"): "borrow main window; native qobject_cast checked",
    ("main_window_titlebar_add_widget", "child"): "Qt/native-container adoption; constructor provenance + ancestor/cross-owner rejection",
    ("object_delete_later", "o"): "deletion REQUEST only; safe wrappers require provenance; active frames retain DeferredDelete",
    ("object_has_parent", "o"): "borrow; diagnostic parent query, not a complete ownership proof",
    ("mainwindow_set_central_widget", "w"): "borrow destination; replacement is native-managed",
    ("mainwindow_set_central_widget", "central"): "Qt/native-container adoption; verified take is the only release path",
    ("layout_add_widget", "l"): "borrow destination layout; managed-widget retention recorded even before installation",
    ("layout_add_widget", "w"): "layout/native retention plus QWidget parenting when installed; reject second raw item",
    ("layout_add_widget_ex", "l"): "borrow destination layout; managed-widget retention recorded even before installation",
    ("layout_add_widget_ex", "w"): "layout/native retention plus QWidget parenting when installed; reject second raw item",
    ("layout_add_layout", "l"): "borrow destination; no cyclic/duplicate/parented child layout",
    ("layout_add_layout", "child"): "native ownership adoption; addChildLayout validated before insertion",
    ("table_set_delegate_for_column", "t"): "borrow table; model/paint frame transaction includes setter",
    ("table_set_delegate_for_column", "delegate"): "retained borrow, NOT transfer; qabstractitemview_p.h stores QPointer delegates",
    ("model_set_parent", "model"): "verified model constructor view; reject cyclic/stolen native-parent handoff",
    ("model_set_parent", "parent"): "borrow destination; parent owns model after accepted QObject handoff",
    ("variant_to_i64", "ok"): "call-scoped initialized Rust bool output pointer; no escape/retention",
    ("menu_add_existing_action", "menu"): "borrow menu destination",
    ("menu_add_existing_action", "action"): "retained Qt action borrow, existing QObject parent unchanged",
    ("menu_popup", "m"): "borrow receiver; popup does not take widget-reference ownership",
    ("menu_popup", "ref_"): "borrow for mapToGlobal only, not retained",
}

# Individually audited QObject-parent constructor inputs, not name inference.
PARENTED = {
    ("scrollbar_new", "parent"), ("qmessagebox_new_with", "parent"),
    ("vbox_new", "parent"), ("hbox_new", "parent"), ("widget_new", "parent"),
    ("standard_item_model_new", "parent"), ("socket_notifier_new", "parent"),
    ("rust_delegate_new", "parent"), ("timer_new", "parent"),
    ("action_new", "parent"), ("menu_new", "parent"), ("paint_widget_new", "parent"),
}
for key in PARENTED:
    INPUTS[key] = "borrow existing parent; constructor creates a NEW parent-owned allocation, not ownership of the parent"

for name in ("qmessagebox_information", "qmessagebox_warning", "qmessagebox_critical", "qmessagebox_question", "qmessagebox_about"):
    INPUTS[name, "parent"] = "call-scoped parent borrow through modal native helper; transaction/DeferredDelete protection, no parent transfer"
for name in ("relay_connect0", "relay_connect_i32", "relay_connect_bool", "relay_connect_i32_i32"):
    INPUTS[name, "sender"] = "borrow sender; binding relay is created as child/scope-tracked; no ownership of sender"
INPUTS["shortcut_new", "parent"] = "borrow widget owner; new native shortcut scope/parent-owned; callback capture cancellation"

RECEIVER_TYPES = {
    "QObject", "QWidget", "DApplication", "DMainWindow", "DTitlebar", "DLabel", "DPushButton",
    "QAction", "QMenu", "QMessageBox", "QLayout", "QAbstractItemModel", "QStandardItemModel",
    "QTableWidget", "QTimer", "QSocketNotifier", "QStyledItemDelegate",
}
VALUE_TYPES = {
    "QVariant", "QVariantList", "QColor", "QFont", "QPalette", "DPalette", "QPixmap", "QImage",
    "QBrush", "QPainterPath", "QKeySequence", "QUrl", "QMargins", "DDciIcon", "QSize", "QPoint", "QRect", "QIcon",
}
UNIQUE_DELETERS = {
    "variant_delete", "variant_list_delete", "color_delete", "font_delete", "palette_delete",
    "dpalette_delete", "pixmap_delete", "image_delete", "painter_path_delete", "brush_delete",
    "key_sequence_delete", "url_delete", "margins_delete", "ddci_icon_delete", "size_delete",
    "point_delete", "rect_delete", "icon_delete",
}


def pointer_contract(name, param, native_type, index):
    if (name, param) in INPUTS:
        return INPUTS[name, param]
    if name in UNIQUE_DELETERS and index == 0 and native_type in VALUE_TYPES:
        return "consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow"
    if native_type in VALUE_TYPES:
        return "borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer"
    if native_type == "QTableWidgetItem" and index == 0:
        return "call-scoped table-owned item borrow resolved by TableCell under same transaction; never stored"
    if native_type in {"QPainter", "QModelIndex"} and index == 0:
        return "synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected"
    if native_type in RECEIVER_TYPES and index == 0:
        return "receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping"
    raise ValueError(f"unaudited hand pointer input {name}::{param}: {native_type}*")
