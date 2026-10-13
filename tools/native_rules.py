"""Source-backed repairs for mapped native lifetime/handoff hazards.

The ordinary call/return conversions stay in gen.py. Only implementations
whose native contract needs extra enforcement appear here; no name heuristic.
"""

# Adopts a QWidget into a native container which may retain raw membership.
WIDGET_ADOPTION = {
    ("DArrowRectangle", "setContent"), ("DBaseLine", "setLeftContent"),
    ("DBaseLine", "setRightContent"), ("DBoxWidget", "addWidget"),
    ("DDialog", "addContent"), ("DDialog", "insertContent"),
    ("DDrawer", "setHeader"), ("DDrawer", "setContent"),
    ("DFloatingMessage", "setWidget"), ("DFloatingWidget", "setWidget"),
    ("DHeaderLine", "setContent"), ("DListView", "addHeaderWidget"),
    ("DListView", "addFooterWidget"), ("DTabBar", "setTabButton"),
}


def method_body(name, method, args, body, ret_rs):
    """Return C++ body with concrete source-audited guards/adjuncts."""
    names = [arg[0].split(":", 1)[0] for arg in args]
    p = names[0] if names else None
    if name == "DAnchorsBase":
        if method == "setAnchor":
            if len(args) == 4: return f"return anchors_set({', '.join(names)});"
            return f"return anchors_set_on(self, {', '.join(names)});"
        if method == "clearAnchors": return f"anchors_clear({p});"
        if method in {"setFill", "setCenterIn"}:
            target = f"{p}->target()" if args[0][4] == "DAnchorsBase" else p
            return f"return anchors_fill(self, {target}, {'true' if method == 'setCenterIn' else 'false'});"
    if name == "DDrawerGroup":
        functions = {
            "checkedExpand": "drawer_group_checked(self)",
            "expand": f"drawer_group_find(self, {p})",
            "addExpand": f"drawer_group_add(self, {', '.join(names)})",
            "setId": f"drawer_group_set_id(self, {', '.join(names)})",
            "removeExpand": f"drawer_group_remove(self, {p})",
            "checkedId": "drawer_group_checked_id(self)",
            "id": f"drawer_group_id(self, {p})",
        }
        return ("return " if ret_rs != "()" else "") + functions[method] + ";"
    if name == "DEnhancedWidget" and method == "setTarget":
        return f"observer_set_target(self, {p});"
    if name == "DLoadingIndicator":
        if method == "setWidgetSource": return f"loading_indicator_set_widget(self, {p});"
        if method == "setImageSource": return f"loading_indicator_set_image(self, *{p});"
        if method == "widgetSource": return "return loading_indicator_widget(self);"
    if name == "DFeatureDisplayDialog":
        if method == "addItem": return f"feature_dialog_add(self, {p});"
        if method == "removeItem": return f"return feature_dialog_take(self, {p});"
        if method == "clearItems": return "feature_dialog_clear(self);"
    if name == "DStackWidget":
        if method == "getWidgetByIndex":
            return f"if ({p} < 0 || {p} >= self->depth()) return nullptr; {body}"
        if method in {"pushWidget", "insertWidget"}:
            widget = next(a[0].split(':')[0] for a in args if a[4] == "QWidget")
            fallback = "return -1;" if method == "pushWidget" else "return;"
            guard = f"!object_can_adopt(self, {widget}) || self->indexOf({widget}) >= 0"
            if method == "insertWidget": guard += f" || {p} < 0 || {p} > self->depth()"
            return f"if ({guard}) {{ {fallback} }} self->transition()->animation()->stop(); object_mark_native_adopted(self, {widget}); {body}"
        if method == "popWidget":
            # Native loop indexes widgetList[i] repeatedly without bounding count.
            return (f"const int start = self->indexOf({p}); if (start < 0 || count <= 0) return; "
                    "count = std::min(count, self->depth() - start); self->transition()->animation()->stop(); " + body)
        if method == "setTransition":
            return f"if (self->transition() == {p} || !object_can_adopt(self, {p})) return; self->transition()->animation()->stop(); object_mark_native_adopted(self, {p}); {body}"
        if method == "clear":
            return "self->transition()->animation()->stop(); " + body
    if name == "DBackgroundGroup" and method == "setLayout":
        return f"if (!layout_can_install(self, {p})) return; {body}"
    if name == "DFlowLayout" and method == "insertLayout":
        return f"if ({p} < 0 || {p} > self->count() || !layout_can_adopt(self, {names[1]})) return; {body}"
    if name == "DFlowLayout" and method == "insertWidget":
        widget = names[1]
        return f"if ({p} < 0 || {p} > self->count() || !layout_prepare_widget(self, {widget})) return; {body}"
    if name == "DListView" and method in {"getHeaderWidget", "getFooterWidget", "takeHeaderWidget", "takeFooterWidget", "removeHeaderWidget", "removeFooterWidget"}:
        getter = "getHeaderWidget" if "Header" in method else "getFooterWidget"
        fallback = "nullptr" if method.startswith(("get", "take")) else None
        if method.startswith("get"):
            return body  # QList::value is natively bounds checked
        return f"if (!self->{getter}({p})) return{(' ' + fallback) if fallback else ''}; {body}"
    if name == "DDialog":
        if method in {"getButton", "getContent"}:
            count = "buttonCount" if method == "getButton" else "contentCount"
            return f"if ({p} < 0 || {p} >= self->{count}()) return nullptr; {body}"
        if method == "removeButton":
            index = f"self->getButtons().indexOf({p})" if args[0][4] == "QAbstractButton" else p
            # Bound the list/layout index and clear the raw default button
            # before the native removal schedules widget deletion.
            return f"const int removed_index = {index}; if (removed_index < 0 || removed_index >= self->buttonCount()) return; self->setDefaultButton(static_cast<QAbstractButton *>(nullptr)); self->removeButton(removed_index);"
        if method == "clearButtons":
            return body + " self->setDefaultButton(static_cast<QAbstractButton *>(nullptr));"
        if method == "setDefaultButton":
            if args[0][4] == "QAbstractButton":
                return f"if (!self->getButtons().contains({p})) return; {body}"
            if args[0][3] == "prim":
                return f"if ({p} < 0 || {p} >= self->buttonCount()) return false; {body}"
        if method == "insertButton" and any(a[4] == "QAbstractButton" for a in args):
            widget = next(a[0].split(':')[0] for a in args if a[4] == "QAbstractButton")
            return f"if (self->getButtons().contains({widget}) || !object_can_adopt(self, {widget})) return; {body}"
        if method in {"addContent", "insertContent"}:
            widget = next(a[0].split(':')[0] for a in args if a[4] == "QWidget")
            body = f"if (self->getContents().contains({widget})) return; {body}"
    if (name, method) in WIDGET_ADOPTION:
        widget = next(a[0].split(':')[0] for a in args if a[4] == "QWidget")
        fallback = "return -1;" if ret_rs == "i32" else "return;"
        body = f"if (!object_can_adopt(self, {widget})) {{ {fallback} }} object_mark_native_adopted(self, {widget}); {body}"
    if name == "DFontSizeManager" and any(a[4] == "DFontSizeManager::SizeType" for a in args):
        enum = next(a[0].split(':')[0] for a in args if a[4] == "DFontSizeManager::SizeType")
        fallback = "return;" if ret_rs == "()" else "return 0;" if ret_rs == "u16" else "return new QFont;"
        body = f"if ({enum} < 0 || {enum} >= DFontSizeManager::NSizeTypes) {{ {fallback} }} {body}"
    return body
