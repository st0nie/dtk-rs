"""Reproducible, fail-closed inventory of hand-written bridge pointer returns."""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
from tools.hand_contracts import pointer_contract

# Every entry names an individually inspected allocation/borrow in the shim.
OBJECT_CONSTRUCTORS = {
    "scrollbar_new", "mainwindow_new", "mainwindow_new_ex", "label_new",
    "suggest_button_new", "push_button_new", "qmessagebox_new", "qmessagebox_new_with",
    "vbox_new", "hbox_new", "widget_new", "table_new", "standard_item_model_new",
    "socket_notifier_new", "rust_delegate_new", "timer_new", "action_new", "menu_new",
    "paint_widget_new",
}
VALUE_ALLOCATIONS = {
    "widget_palette", "icon_from_theme", "icon_from_theme_fallback", "icon_from_file",
    "model_data", "variant_new", "variant_from_bool", "variant_from_i64",
    "variant_from_f64", "variant_from_string", "variant_clone", "variant_list_new",
    "variant_list_clone", "variant_list_get", "font_new",
    "palette_new", "dpalette_new", "dpalette_text_title", "pixmap_new",
    "image_new", "image_from_file", "painter_path_new",
    "brush_from_color", "key_sequence_from_string", "ddci_icon_new", "ddci_icon_from_file", "standard_icon_pixmap",
    "persistent_index_from", "persistent_index_clone", "persistent_index_data",
    "persistent_index_parent", "model_index",
    "style_option_new", "style_option_clone", "view_item_option_new", "view_item_option_clone",
}
SPECIAL_RETURNS = {
    "application_runtime_application": ("scope borrow", "application_runtime.cpp: runtime.application(); no public owning/Clone token"),
    "application_runtime_gui_application": ("scope borrow", "shim.cpp: qobject_cast of private runtime application"),
    "core_config_create_generic": ("unique Owned<DConfig>", "dtkcore/src/dconfig.cpp createGeneric: explicitly released by caller; parent nullptr"),
    "core_config_create": ("unique Owned<DConfig>", "dtkcore/src/dconfig.cpp create: explicitly released by caller; parent nullptr"),
    "core_settings_from_json": ("unique Owned<DSettings>", "dtkcore/src/settings/dsettings.cpp fromJson: QPointer(new DSettings) without parent, documented as caller-released"),
    "core_settings_from_json_file": ("unique Owned<DSettings>", "dtkcore/src/settings/dsettings.cpp fromJsonFile -> fromJson: parentless new DSettings"),
    "core_file_watcher_new": ("constructor Owned; Qt adoption revokes original deletion", "dtkcore/src/filesystem/dfilewatcher.cpp: DFileWatcher(path, parent = 0) allocation"),
    "widget_style": ("borrowed guarded view", "qt6-base qwidget.cpp style(): widget's own style or QApplication::style(); never transferred"),
    "style_as_dstyle": ("borrowed guarded view", "qobject_cast of the same borrowed style object; no allocation"),
    "view_item_option_base": ("borrowed base view", "static upcast into the same option allocation; deleted only through the owning QStyleOptionViewItem"),
    "painter_begin_image": ("closure-scoped unique owner", "new QPainter(image); Painter::on_image ends and deletes it (painter_finish) before returning, image borrowed &mut"),
    "rust_list_model_new": ("constructor Owned; parent adoption", "shim.cpp RustListModel(cb_id, editable, parent): new parent-owned QObject holding only a callback id"),
    "mainwindow_titlebar": ("borrowed guarded view", "DMainWindow owns titlebar; no deletion authority from getter"),
    "mainwindow_take_central_widget": ("unique transferred Owned<QWidget>", "qt6-base/src/widgets/widgets/qmainwindow.cpp takeCentralWidget: setParent(nullptr), caller ownership"),
    "qmessagebox_add_button_text": ("Qt-owned guarded view", "QMessageBox::addButton creates/owns the button, not a Rust constructor owner"),
    "qmessagebox_add_button_standard": ("Qt-owned guarded view", "QMessageBox::addButton owns or returns existing standard button"),
    "table_item": ("call-scoped item borrow", "TableCell resolves coordinates under one transaction; QTableWidget owns/deletes items; item* never escapes"),
    "standard_item_model_as_model": ("shared adjusted base view", "C++ static_cast; shares original QObject guard, not a new allocation"),
}


def render():
    bridge = (ROOT / "dtk-sys/src/lib.rs").read_text().split('extern "Rust"')[0]
    raw = re.findall(r"unsafe fn (\w+)\([^;]*?\)\s*->\s*\*mut\s+(\w+)\s*;", bridge, re.S)
    cpp = (ROOT / "dtk-sys/cpp/shim.cpp").read_text()
    rows = ["# Hand-written FFI pointer/ownership audit\n\n",
            "Source authority: matching apt source Qt 6.8.0+dfsg-0deepin31, dtkwidget 6.7.47.1-1, dtkcore/dtkgui 6.7.47.\n",
            "Regenerate with `uv run --no-project python tools/audit_hand.py`. New pointer-return functions fail closed until reviewed.\n\n",
            "| Bridge API | Pointer type | Ownership | Evidence / lifetime |\n|---|---|---|---|\n"]
    for name, typ in raw:
        if name in SPECIAL_RETURNS:
            ownership, evidence = SPECIAL_RETURNS[name]
        elif name in OBJECT_CONSTRUCTORS:
            ownership = "constructor Owned; Qt adoption revokes original deletion"
            evidence = f"shim.cpp `{name}`: verified new allocation; registered after guard construction"
        elif name in VALUE_ALLOCATIONS:
            ownership = "unique heap value/copy; matching *_delete"
            evidence = f"shim.cpp `{name}`: new allocation/copy; Qt storage never borrowed into Rust"
        else:
            raise ValueError(f"unaudited hand pointer return {name}: {typ}")
        if name not in {"application_runtime_application"} and name not in cpp:
            raise ValueError(f"native implementation missing for {name}")
        rows.append(f"| `{name}` | `{typ}*` | {ownership} | {evidence} |\n")
    declared = {name for name, _ in raw}
    unused = (OBJECT_CONSTRUCTORS | VALUE_ALLOCATIONS | set(SPECIAL_RETURNS)) - declared
    if unused:
        raise ValueError(f"stale hand pointer contracts: {sorted(unused)}")
    rows.append(f"\nRaw pointer-return declarations audited: **{len(raw)}**.\n\n")
    unique = re.findall(r"unsafe fn (\w+)\([^;]*?\)\s*->\s*UniquePtr<(\w+)>\s*;", bridge, re.S)
    rows.append("## CXX unique native state owners\n\n")
    for name, typ in unique:
        if typ == "QObjectGuard":
            evidence = "owns only guard/event filter/QPointer, never the watched QObject"
        elif typ == "ApplicationRuntime":
            evidence = "private outer application owner; no user escape or callback deletion"
        elif typ == "NativeCallFrame":
            evidence = "state-only transaction token; no Qt ownership or mutable pinned reference"
        elif typ == "ConfigWorkerProbe":
            evidence = "native-only worker regression fixture; no DTK public API coverage"
        else:
            raise ValueError(f"unaudited CXX unique owner {name}: {typ}")
        rows.append(f"- `{name}` → `UniquePtr<{typ}>`: {evidence}.\n")
    rows.append("\n## Every hand-written raw pointer input\n\n"
                "Each row identifies a receiver, retained/call borrow, unique destructor or output, not inferred transfer by spelling.\n\n"
                "| Native operation | Parameter | Native pointer | Ownership / lifetime rule |\n"
                "|---|---|---|---|\n")
    count = 0
    for name, signature in re.findall(r"unsafe fn (\w+)\((.*?)\)\s*(?:->[^;]+)?;", bridge, re.S):
        pointers = re.findall(r"(\w+):\s*\*mut\s+(\w+)", signature)
        for index, (param, native_type) in enumerate(pointers):
            contract = pointer_contract(name, param, native_type, index)
            rows.append(f"| `{name}` | `{param}` | `{native_type}*` | {contract} |\n")
            count += 1
    rows.append(f"\nRaw pointer input parameters audited: **{count}**.\n")
    rows.append("\n## Callback and retained borrows\n\n"
                "- `QPainter*` / `QModelIndex*`: callback-lifetime `&Painter` / `&ModelIndex`; never Clone/Copy/owned pointers. Synchronous native borrowing rejects QObject access and nested loops; independent Qt values and painter/index operations remain usable.\n"
                "- QObject parameters/base views: non-owning, owning-thread QPointer checks plus native transaction spanning arguments, call, and result wrapping. Shared views never manufacture exclusive/pinned references.\n"
                "- QWidget/layout adoption: cycle/duplicate-parent checks, permanent adoption history (including QObject ChildAdded and parentless native transfers); old constructor owners cannot reclaim transferred allocations.\n"
                "- Model/delegate setters: Qt retains borrowed QObject pointers; Qt ownership is not inferred from setter spelling. Callback-time mutation cannot invalidate a currently borrowed paint/index/model frame.\n"
                "- Relays, timers, shortcuts and actions: native owner destruction cancels pending events immediately; capture destructors are retired to a safe Rust boundary, with scope cleanup outside registry borrows.\n")
    return "".join(rows) + render_by_value() + render_cxxqt()


# cxx-qt-lib by-value types: `cxx::kind::Trivial` ExternType impls whose C++
# layout and trivial copyability/relocatability are static_asserted in
# cxx-qt-lib's own C++ sources against the installed Qt.
BY_VALUE = {
    "QColor": "trivially copyable (Qt6 static_assert in cxx-qt-lib qcolor.cpp)",
    "QPoint": "trivially copyable POD", "QPointF": "trivially copyable POD",
    "QSize": "trivially copyable POD", "QSizeF": "trivially copyable POD",
    "QRect": "trivially copyable POD", "QRectF": "trivially copyable POD",
    "QMargins": "trivially copyable POD", "QMarginsF": "trivially copyable POD",
    "QUrl": "implicitly shared d-pointer; `rust::IsRelocatable<QUrl>`; Rust Clone/Drop call the C++ copy ctor/destructor",
}


def render_by_value():
    """Every hand + generated bridge by-value Qt type must be reviewed here."""
    rows = ["\n## By-value Qt types (cxx-qt-lib)\n\n",
            "No heap allocation, deleter or pointer crosses FFI: returns are C++ copies moved into Rust, "
            "inputs are `const T &` borrows of Rust-owned values. These are plain Rust values with cxx-qt-lib's Clone/Drop (no QObject reference, no scope guard); "
            "independent of the application scope.\n\n| Type | Bridges | Copy semantics |\n|---|---|---|\n"]
    seen = {}
    for path in ("dtk-sys/src/lib.rs", "dtk-sys/src/gen_ffi.rs", "dtk-sys/src/core_ffi.rs", "dtk-sys/src/gui_ffi.rs"):
        for name in re.findall(r"type (\w+) = cxx_qt_lib::(\w+);", (ROOT / path).read_text()):
            if name[0] != name[1]:
                raise ValueError(f"{path}: aliased by-value type {name}")
            seen.setdefault(name[0], []).append(Path(path).name)
    unknown = set(seen) - set(BY_VALUE)
    if unknown:
        raise ValueError(f"unaudited by-value Qt types: {sorted(unknown)}")
    for name in sorted(seen):
        rows.append(f"| `{name}` | {', '.join(seen[name])} | {BY_VALUE[name]} |\n")
    return "".join(rows)


def _snake(name):
    return re.sub(r"_+", "_", re.sub(r"(?<!^)(?=[A-Z])", "_", name).lower())


def render_cxxqt():
    """Fail closed unless every CXX-Qt import keeps the shared-borrow model."""
    bridge = (ROOT / "dtk-cxxqt/src/bridge.rs").read_text()
    glue = (ROOT / "dtk-cxxqt/src/connectors.rs").read_text()
    code = "\n".join(line.split("//")[0] for line in bridge.splitlines())
    for banned in ("Pin<", "&mut", '"RustQt"', "Threading", "#[qinvokable]", "#[qproperty"):
        if banned in code:
            raise ValueError(f"dtk-cxxqt bridge declares forbidden `{banned}`")
    rows = ["\n## CXX-Qt imported QObjects (dtk-cxxqt)\n\n",
            "Imports only: no construction, ownership, deletion or `Pin<&mut T>`/`&mut T` receiver. "
            "Senders are guarded live QObject pointers inside a `NativeCall`, cast with C++ `dynamic_cast` "
            "to the declaring class (generated by tools/signals.py); functors own a thread-checked "
            "`SignalToken` that queues owned values (by-value copies, `OwnedValue` heap copies with their "
            "matching deleter, `QPersistentModelIndex` for `QModelIndex`) and cancels/retires the callback "
            "when Qt destroys it.\n\n",
            "| C++ type | Item | Receiver | Kind | Rust-facing glue |\n|---|---|---|---|---|\n"]
    current = None
    count = 0
    items = re.finditer(
        r"type (\w+);|(#\[qsignal\][^;]*?)?(pub\(self\) )?fn (\w+)\(\s*self: (&\w+)[^;]*;", code, re.S)
    for item in items:
        if item.group(1):
            current = item.group(1)
            continue
        signal, private, name, receiver = item.group(2), item.group(3), item.group(4), item.group(5)
        if receiver != f"&{current}":
            raise ValueError(f"{current}::{name}: receiver {receiver} is not a shared borrow")
        if signal and not private:
            raise ValueError(f"{current}::{name}: signals must be pub(self) (no Rust emitter)")
        glue_name = f"sender.on_{name}(" if signal else f".{name}()"
        if (f"ffi::as_{current}(" not in glue) or glue_name not in glue:
            raise ValueError(f"{current}::{name}: imported but no audited dtk-cxxqt glue")
        kind = "typed signal connection" if signal else "const method"
        rows.append(f"| `{current}` | `{name}` | `{receiver}` | {kind} | `connectors::{_snake(current)}_{name}` |\n")
        count += 1
    rows.append(f"\nCXX-Qt imported items audited: **{count}**.\n")
    return "".join(rows)


def main():
    content = render()
    path = ROOT / "OWNERSHIP_HAND_REPORT.md"
    if not path.exists() or path.read_text() != content:
        path.write_text(content)


if __name__ == "__main__":
    main()
