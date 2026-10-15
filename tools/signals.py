"""Generate the CXX-Qt typed-signal layer from header `Q_SIGNALS:` sections.

Inputs: every exported DTK (DWidget/DGui/DCore) and Qt (QtCore/QtGui/
QtWidgets) class reachable from a dtk-rs QObject wrapper. Each Qt/DTK class
that declares at least one bindable signal is imported ONCE into the CXX-Qt
bridge with only its own signals. A wrapper connects inherited signals through
a checked `qobject_cast` of its live QObject pointer to the declaring class, so
no per-subclass C++ connect code is instantiated.

Outputs (generator-owned, do not edit):
  dtk-cxxqt/src/bridge.rs            #[cxx_qt::bridge] imports (`&T` receivers)
  dtk-cxxqt/src/connectors.rs        unsafe connectors: owned argument conversion
  dtk-cxxqt/include/dtk_cxxqt_casts.h qobject_cast + value-copy helpers
  dtk/src/signals_gen.rs             safe `on_*` methods returning Connection
  GEN_SIGNAL_REPORT.md               generated/skipped signals with reasons

Argument rules (all values are converted to OWNED Rust data inside the
emission; the user closure runs later from the dtk-sys callback pump):
  scalars / unscoped enums   copied (enums as i32)
  QString/QStringList/QByteArray  String / Vec<String> / Vec<u8>
  cxx-qt-lib trivial values  cloned (QColor, QPoint, QSize, QRect, QUrl, ...)
  hand heap value types      C++ copy into a dtk-sys OwnedValue, then the dtk
                             wrapper (QVariant, QFont, QImage, QKeySequence, ...)
  QModelIndex                copied into dtk::PersistentModelIndex (QPersistentModelIndex)
  pointers / QFlags / scoped enums / other types  skipped and reported
"""
import os
import re
import subprocess

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DTK_DIRS = {"/usr/include/dtk6/DWidget": "Dtk::Widget", "/usr/include/dtk6/DGui": "Dtk::Gui",
            "/usr/include/dtk6/DCore": "Dtk::Core"}
QT_MODULES = ("QtCore", "QtGui", "QtWidgets")
# Macros defined for this Linux Qt6/DTK6 build; any other `#ifdef` region is inactive.
DEFINED = {"Q_OS_LINUX", "Q_OS_UNIX", "QT_CORE_LIB", "QT_GUI_LIB", "QT_WIDGETS_LIB", "__cplusplus"}

PRIM = {"bool": "bool", "int": "i32", "qint32": "i32", "uint": "u32", "quint32": "u32",
        "qint64": "i64", "qlonglong": "i64", "quint64": "u64", "qulonglong": "u64",
        "double": "f64", "qreal": "f64", "float": "f32", "short": "i16", "quint8": "u8",
        "qint8": "i8", "quint16": "u16", "qsizetype": "isize"}
# cxx-qt-lib `cxx::kind::Trivial` types (same list as tools/gen.py CXXQT_VALUE_TYPES).
CXXQT_VALUES = {"QColor", "QPoint", "QPointF", "QSize", "QSizeF", "QRect", "QRectF",
                "QMargins", "QMarginsF", "QUrl"}
# cxx-qt-lib QList instantiations: C++ spelling -> (bridge alias, element, header, element kind).
LISTS = {
    "QList<int>": ("QList_i32", "i32", "qlist_i32.h", "copy"),
    "QVector<int>": ("QList_i32", "i32", "qlist_i32.h", "copy"),
    "QList<QUrl>": ("QList_QUrl", "QUrl", "qlist_QUrl.h", "clone"),
}
# Hand-written dtk heap value wrappers: C++ type -> (namespace, include, dtk-sys deleter).
HEAP_VALUES = {
    "QVariant": ("", "<QtCore/QVariant>", "variant_delete"),
    "QFont": ("", "<QtGui/QFont>", "font_delete"),
    "QImage": ("", "<QtGui/QImage>", "image_delete"),
    "QPixmap": ("", "<QtGui/QPixmap>", "pixmap_delete"),
    "QIcon": ("", "<QtGui/QIcon>", "icon_delete"),
    "QBrush": ("", "<QtGui/QBrush>", "brush_delete"),
    "QPalette": ("", "<QtGui/QPalette>", "palette_delete"),
    "QPainterPath": ("", "<QtGui/QPainterPath>", "painter_path_delete"),
    "QKeySequence": ("", "<QtGui/QKeySequence>", "key_sequence_delete"),
    "DPalette": ("Dtk::Gui", "<dpalette.h>", "dpalette_delete"),
    "DDciIcon": ("Dtk::Gui", "<ddciicon.h>", "ddci_icon_delete"),
    "QModelIndex": ("", "<QtCore/QModelIndex>", "persistent_index_delete"),
}

# Rust-facing QObject wrappers written by hand in dtk/src/lib.rs: Rust -> C++ class.
HAND_WRAPPERS = {
    "QWidget": "QWidget", "QAbstractButton": "QAbstractButton", "PaintWidget": "QWidget",
    "QAction": "QAction", "DMenu": "QMenu", "DMainWindow": "DMainWindow",
    "DTitlebar": "DTitlebar", "DLabel": "DLabel", "DSuggestButton": "DSuggestButton",
    "DPushButton": "QPushButton", "DMessageBox": "QMessageBox", "QLayout": "QLayout",
    "QBoxLayout": "QBoxLayout", "QVBoxLayout": "QVBoxLayout", "QHBoxLayout": "QHBoxLayout",
    "QAbstractItemModel": "QAbstractItemModel", "QStandardItemModel": "QStandardItemModel",
    "QTableWidget": "QTableWidget", "QTimer": "QTimer", "PaintDelegate": "QStyledItemDelegate",
    "QSocketNotifier": "QSocketNotifier",
}
# DTK typedef names of Qt classes, filled by index_classes().
ALIASES = {}
# Signals deliberately left to hand-written code with stronger semantics.
EXCLUDED = {
    # Level-triggered readiness: relay pauses polling until delivery (RUNTIME_SAFETY_AUDIT.md).
    ("QSocketNotifier", "activated"): "hand-written level-triggered relay (QSocketNotifier::on_activated)",
    # Emitted from ~QObject: the sender is already being destroyed; the
    # binding's own destruction tracking cancels callbacks instead.
    ("QObject", "destroyed"): "emitted during destruction; use guards/Connection cancellation",
    ("QObject", "objectNameChanged"): "QObject base not imported (every wrapper would duplicate it)",
}
# Const getters imported with `&T` receivers (see CXXQT_INTEGRATION.md section 4).
CONST_METHODS = {"QAbstractButton": [("is_checked", "isChecked", "bool")]}

CLASS_LINE = re.compile(
    r"^\s*class\s+(?:(?:Q_\w+_EXPORT|LIBDTK\w+_EXPORT|D_DECL_DEPRECATED\w*)\s+)*(\w+)"
    r"(?:\s+final)?\s*(?::\s*([^{]*))?\s*\{?\s*$")
ACCESS = re.compile(r"^\s*(public|protected|private)\s*(?:Q_SLOTS|slots)?\s*:")
SIGNALS = re.compile(r"^\s*(?:Q_SIGNALS|signals)\s*:")


def snake(name):
    separated = re.sub(r"(?<!^)(?=[A-Z])", "_", name).lower()
    return re.sub(r"_+", "_", separated)


def _strip_comments(text):
    text = re.sub(r"/\*.*?\*/", " ", text, flags=re.S)
    return re.sub(r"//[^\n]*", "", text)


def _active_lines(path):
    """Yield lines outside DTK5-only, qdoc-only and deprecated preprocessor regions."""
    stack = []
    with open(path, encoding="utf-8", errors="replace") as f:
        text = _strip_comments(f.read())
    for line in text.splitlines():
        s = line.strip()
        if s.startswith("#"):
            directive = s[1:].strip()
            if directive.startswith(("if", "ifdef", "ifndef")):
                skip = bool(re.match(r"if\s+DTK_VERSION\s*<\s*DTK_VERSION_CHECK\(\s*6", directive)
                            or re.match(r"ifdef\s+Q_(CLANG_)?QDOC", directive)
                            or re.match(r"if\s+defined\(Q_(CLANG_)?QDOC\)", directive)
                            or "QT_DEPRECATED_SINCE" in directive or "QT_CORE_REMOVED_SINCE" in directive
                            or "QT_GUI_REMOVED_SINCE" in directive or "QT_WIDGETS_REMOVED_SINCE" in directive
                            # Optional feature macros (e.g. DTK_TITLE_DRAG_WINDOW) are not
                            # defined by the installed pkg-config flags.
                            or (directive.startswith("ifdef") and directive.split()[1] not in DEFINED))
                stack.append(skip)
            elif directive.startswith("else") or directive.startswith("elif"):
                if stack:
                    stack[-1] = not stack[-1]
            elif directive.startswith("endif"):
                if stack:
                    stack.pop()
            continue
        if not any(stack):
            yield line


def index_classes():
    """Map exported class name -> (header path, namespace, [public base names], include)."""
    qt_root = subprocess.check_output(["pkg-config", "--variable=includedir", "Qt6Core"], text=True).strip()
    found = {}
    dirs = [(d, ns, None) for d, ns in DTK_DIRS.items()]
    dirs += [(os.path.join(qt_root, m), "", m) for m in QT_MODULES]
    for directory, ns, module in dirs:
        for name in sorted(os.listdir(directory)):
            if not name.endswith(".h") or name.endswith("_p.h"):
                continue
            path = os.path.join(directory, name)
            for line in _active_lines(path):
                m = CLASS_LINE.match(line)
                if not m or m.group(1) in found:
                    continue
                bases = re.findall(r"public\s+(?:\w+::)*(\w+)", m.group(2) or "")
                include = f"<{module}/{name}>" if module else f"<{name}>"
                found[m.group(1)] = (path, ns, bases, include)
    # dwidgetstype.h: `typedef QPushButton DPushButton;` etc. resolve to the Qt class.
    for line in _active_lines("/usr/include/dtk6/DWidget/dwidgetstype.h"):
        m = re.match(r"^\s*typedef\s+(Q\w+)\s+(D\w+)\s*;", line)
        if m and m.group(1) in found and m.group(2) not in found:
            found[m.group(2)] = found[m.group(1)][:2] + ([m.group(1)],) + found[m.group(1)][3:]
            ALIASES[m.group(2)] = m.group(1)
    return found


def class_body(path, name):
    """Return the top-level body lines of `class ... name`, depth-1 statements only."""
    lines = list(_active_lines(path))
    start = None
    for i, line in enumerate(lines):
        m = CLASS_LINE.match(line)
        if m and m.group(1) == name:
            start = i
            break
    if start is None:
        return []
    depth, body, opened = 0, [], False
    for line in lines[start:]:
        for ch in line:
            if ch == "{":
                depth += 1
                opened = True
            elif ch == "}":
                depth -= 1
        body.append((depth, line))
        if opened and depth <= 0:
            break
    return body


def parse_class(path, name, enums):
    """(signals, declared function-name counts). Records enums as a side effect."""
    signals, names = [], {}
    section, statement = None, ""
    body = class_body(path, name)
    if not body:
        return signals, names
    depth_prev = body[0][0]
    for depth, line in body[1:]:
        s = line.strip()
        if s in ("{", "};", "}") or (depth_prev == 1 and depth == 1
                                     and re.fullmatch(r"[A-Z][A-Z0-9_]*(\(.*\))?", s)):
            depth_prev = depth
            continue  # brace lines and statement-less macros (Q_OBJECT, Q_PROPERTY(...))
        em = re.match(r"^\s*enum\s+(class\s+|struct\s+)?(\w+)", line)
        if em and depth_prev == 1:
            key = (name, em.group(2))
            enums[key] = "scoped" if em.group(1) else "unscoped"
        if depth_prev == 1 and SIGNALS.match(line):
            section, statement = "signals", ""
            continue
        if depth_prev == 1 and ACCESS.match(line):
            section, statement = "other", ""
            continue
        if depth_prev == 1 or depth == 1:
            statement += " " + s
            if depth == 1 and (s.endswith(";") or s.endswith("}")):
                decl = statement.split("{", 1)[0].strip()
                fm = re.search(r"(~?\w+)\s*\(", decl)
                if fm and not decl.startswith(("Q_", "using", "friend", "typedef")):
                    names[fm.group(1)] = names.get(fm.group(1), 0) + 1
                if section == "signals" and fm:
                    signals.append(decl.rstrip(";").strip())
                statement = ""
        depth_prev = depth
    return signals, names


def index_namespace_enums(enums):
    """Unscoped enums in namespace Qt (qnamespace.h) and DTK namespace-level enums."""
    qt_root = subprocess.check_output(["pkg-config", "--variable=includedir", "Qt6Core"], text=True).strip()
    for path, scope in [(os.path.join(qt_root, "QtCore/qnamespace.h"), "Qt")]:
        for line in _active_lines(path):
            em = re.match(r"^\s*enum\s+(class\s+|struct\s+)?(\w+)", line)
            if em:
                enums[(scope, em.group(2))] = "scoped" if em.group(1) else "unscoped"


def split_params(text):
    out, depth, cur = [], 0, ""
    for ch in text:
        if ch in "<(":
            depth += 1
        elif ch in ">)":
            depth -= 1
        if ch == "," and depth == 0:
            out.append(cur.strip())
            cur = ""
        else:
            cur += ch
    if cur.strip():
        out.append(cur.strip())
    return out


def map_param(raw, owner, ancestors, enums):
    """-> (bridge_type, closure_type, rust_value_type, convert_expr_fmt, kind) or (None, reason)."""
    p = raw.split("=", 1)[0].strip()
    p = re.sub(r"\b(Q_DECL_\w+)\b", "", p).strip()
    m = re.match(r"^(.*?)(\b[A-Za-z_]\w*)?$", p)
    ptype = p
    if (m and m.group(2) and m.group(1).strip() and not m.group(1).strip().endswith("::")
            and not re.fullmatch(r"(?:const|volatile|unsigned|signed|\s)*", m.group(1))):
        ptype = m.group(1).strip()
    t = re.sub(r"\bconst\b", "", ptype).strip()
    is_ref = t.endswith("&")
    t = t.rstrip("&").strip()
    if t.endswith("*"):
        return None, f"pointer argument `{ptype}` (guarded-view conversion not audited)"
    if t == "QPrivateSignal":
        return "private", None
    base = t.replace("DTK_GUI_NAMESPACE::", "").replace("DTK_CORE_NAMESPACE::", "").replace("DGUI_NAMESPACE::", "")
    compact = re.sub(r"\s+", "", base)
    if compact in LISTS:
        alias, elem, _h, how = LISTS[compact]
        conv = "{}.iter().copied().collect::<Vec<_>>()" if how == "copy" else "{}.iter().cloned().collect::<Vec<_>>()"
        return (f"&{alias}", f"&{alias}", f"Vec<{elem}>", conv, "list"), None
    if base in PRIM:
        r = PRIM[base]
        return (r, r, r, "{}", "prim"), None
    if base == "QString":
        return ("&QString", "&QString", "String", "String::from({})", "str"), None
    if base == "QStringList":
        return ("&QStringList", "&QStringList", "Vec<String>", "{}.iter().map(String::from).collect::<Vec<_>>()", "strlist"), None
    if base == "QByteArray":
        return ("&QByteArray", "&QByteArray", "Vec<u8>", "Vec::<u8>::from({})", "bytes"), None
    if base in CXXQT_VALUES:
        return (f"&{base}", f"&{base}", base, "{}.clone()", "qval"), None
    if base in HEAP_VALUES:
        return (f"&{base}", f"&{base}", base, base, "heap"), None
    # Unscoped enums convert implicitly to int in the functor; scoped/QFlags do not.
    parts = base.split("::")
    scopes = [parts[0]] if len(parts) == 2 else [owner, *ancestors]
    ename = parts[-1]
    for scope in scopes:
        kind = enums.get((scope, ename))
        if kind == "unscoped":
            return ("i32", "i32", "i32", "{}", "enum"), None
        if kind == "scoped":
            return None, f"scoped enum `{ptype}` (no implicit int conversion)"
    return None, f"unsupported argument type `{ptype}`"


def collect(wrappers):
    """wrappers: Rust name -> C++ class. Returns import model + per-wrapper method plans."""
    index = index_classes()
    enums = {}
    index_namespace_enums(enums)
    parsed = {}

    def ancestry(name, seen=None):
        seen = seen or []
        if name not in index or name in seen:
            return seen
        seen.append(name)
        for base in index[name][2]:
            ancestry(base, seen)
        return seen

    for cls in {c for w in wrappers.values() for c in ancestry(w)}:
        path = index[cls][0]
        parsed[cls] = parse_class(path, cls, enums)

    imports, report = {}, {}
    for cls, (signals, names) in sorted(parsed.items()):
        anc = ancestry(cls)[1:]
        for decl in signals:
            m = re.match(r"^(?:Q_SIGNAL\s+|Q_DECL_\w+\s+)*void\s+(\w+)\s*\((.*)\)\s*(?:const)?\s*(?:Q_DECL_\w+)?$", decl, re.S)
            if not m:
                report.setdefault(cls, []).append((decl, None, "signature parse failed"))
                continue
            sig, params = m.group(1), split_params(m.group(2))
            if (cls, sig) in EXCLUDED:
                report.setdefault(cls, []).append((decl, None, EXCLUDED[cls, sig]))
                continue
            if names.get(sig, 0) > 1:
                report.setdefault(cls, []).append((decl, None, f"overloaded name: `&{cls}::{sig}` is ambiguous"))
                continue
            mapped, reason = [], None
            for i, raw in enumerate(params):
                r, why = map_param(raw, cls, anc, enums)
                if r == "private":
                    if i != len(params) - 1:
                        reason = "QPrivateSignal not last"
                    continue
                if r is None:
                    reason = why
                    break
                mapped.append(r)
            if reason:
                report.setdefault(cls, []).append((decl, None, reason))
                continue
            imports.setdefault(cls, []).append((sig, mapped, decl))
            report.setdefault(cls, []).append((decl, f"{snake(cls)}_{snake(sig)}", None))
    for cls in CONST_METHODS:
        imports.setdefault(cls, [])

    plans = {}
    for rust, cpp in wrappers.items():
        seen, methods = set(), []
        for cls in ancestry(cpp):
            for sig, mapped, decl in imports.get(cls, []):
                method = f"on_{snake(sig)}"
                if method in seen:
                    continue  # hidden by a more-derived signal of the same name
                seen.add(method)
                methods.append((method, cls, sig, mapped, decl))
        plans[rust] = methods
    return index, imports, plans, report


def rust_value_type(kind, value):
    if kind == "list":
        inner = value[4:-1]
        return f"Vec<crate::{inner}>" if inner in CXXQT_VALUES else value
    if kind == "heap":
        return "crate::PersistentModelIndex" if value == "QModelIndex" else f"crate::{value}"
    if kind == "qval":
        return f"crate::{value}"
    return value


def emit(domain_wrappers):
    """domain_wrappers: {rust_path: cpp_class} for generated and hand wrappers."""
    index, imports, plans, report = collect(domain_wrappers)
    bridge = ["// auto-generated by tools/signals.py, do not edit\n",
              "//! CXX-Qt imports of existing Qt/DTK QObject classes: only their own\n",
              "//! signals (`pub(self)`, connect code only) and audited `const` getters,\n",
              "//! all with `self: &T` receivers. Never `Pin<&mut T>`/`&mut T` (audited by\n",
              "//! tools/audit_hand.py). Mutating methods stay dtk-sys raw-pointer shims.\n\n",
              "#[cxx_qt::bridge]\npub mod ffi {\n    unsafe extern \"C++\" {\n",
              "        include!(\"dtk_cxxqt_casts.h\");\n",
              "        type QString = cxx_qt_lib::QString;\n",
              "        type QStringList = cxx_qt_lib::QStringList;\n",
              "        type QByteArray = cxx_qt_lib::QByteArray;\n",
              ]
    for v in sorted(CXXQT_VALUES):
        bridge.append(f"        type {v} = cxx_qt_lib::{v};\n")
    for alias, elem, header, _how in sorted(set(LISTS.values())):
        bridge.append(f"        include!(\"cxx-qt-lib/core/qlist/{header}\");\n"
                      f"        type {alias} = cxx_qt_lib::QList<{elem}>;\n")
    heap_used = sorted({m[3] for sigs in imports.values() for _, ms, _ in sigs for m in ms if m[4] == "heap"})
    for v in heap_used:
        ns, _inc, _del = HEAP_VALUES[v]
        if ns:
            bridge.append(f"        #[namespace = \"{ns}\"]\n")
        bridge.append(f"        type {v};\n")
    casts_h = ["// auto-generated by tools/signals.py, do not edit\n#pragma once\n",
               "#include <QtCore/QObject>\n#include <QtCore/QPersistentModelIndex>\n"]
    includes = sorted({index[c][3] for c in imports} | {HEAP_VALUES[v][1] for v in heap_used})
    casts_h += [f"#include {inc}\n" for inc in includes]
    casts_h.append("#include \"rust/cxx.h\"\n#include \"cxx-qt-lib/qstring.h\"\n#include \"cxx-qt-lib/qstringlist.h\"\n"
                   "#include \"cxx-qt-lib/qbytearray.h\"\n")
    casts_h += [f"#include \"cxx-qt-lib/{v.lower()}.h\"\n" for v in sorted(CXXQT_VALUES)]
    casts_h.append("\nnamespace dtkrs_cxxqt {\n")
    for v in heap_used:
        ns = HEAP_VALUES[v][0]
        q = f"::{ns}::{v}" if ns else f"::{v}"
        if v == "QModelIndex":
            # A QModelIndex is only valid until the model changes; queue a
            # QPersistentModelIndex that Qt updates (or invalidates) instead.
            casts_h.append(f"inline ::QPersistentModelIndex *clone_{v}(const {q} &value) {{ return new ::QPersistentModelIndex(value); }}\n")
        else:
            casts_h.append(f"inline {q} *clone_{v}(const {q} &value) {{ return new {q}(value); }}\n")
    bridge.append("        #[namespace = \"dtkrs_cxxqt\"]\n        type QPersistentModelIndex;\n" if "QModelIndex" in heap_used else "")
    for v in heap_used:
        out = "QPersistentModelIndex" if v == "QModelIndex" else v
        bridge.append(f"        #[namespace = \"dtkrs_cxxqt\"]\n        fn clone_{v}(value: &{v}) -> *mut {out};\n")
    for cls in sorted(imports):
        ns = index[cls][1]
        q = f"::{ns}::{cls}" if ns else f"::{cls}"
        casts_h.append(f"inline const {q} *as_{cls}(const ::QObject *object) {{ return qobject_cast<const {q} *>(object); }}\n")
        bridge.append(f"        /// # Safety\n        /// `object` is null or a live QObject on its owner thread.\n        #[namespace = \"dtkrs_cxxqt\"]\n        unsafe fn as_{cls}(object: *const QObject) -> *const {cls};\n")
    if "QModelIndex" in heap_used:
        casts_h.append("using QPersistentModelIndex = ::QPersistentModelIndex;\n")
    casts_h.append("} // namespace dtkrs_cxxqt\n")
    bridge.append("    }\n")

    connectors = ["// auto-generated by tools/signals.py, do not edit\n",
                  "#![allow(clippy::type_complexity)]\n",
                  "// Every connector: `sender` is a live QObject on its owner thread inside a\n",
                  "// dtk-sys NativeCall; a failed qobject_cast returns None (no connection).\n",
                  "use crate::ffi;\n#[allow(unused_imports)]\nuse crate::ffi::*;\nuse dtk_sys::native_signal::{OwnedValue, SignalToken};\nuse std::any::Any;\n\n"]
    total = 0
    for cls in sorted(imports):
        ns = index[cls][1]
        bridge.append("\n    unsafe extern \"C++Qt\" {\n")
        if ns:
            bridge.append(f"        #[namespace = \"{ns}\"]\n")
        bridge.append(f"        #[qobject]\n        type {cls};\n")
        for sig, mapped, decl in imports[cls]:
            params = "".join(f", a{i}: {m[0]}" for i, m in enumerate(mapped))
            bridge.append(f"        /// `{' '.join(decl.split())}`\n        #[qsignal]\n        #[cxx_name = \"{sig}\"]\n"
                          f"        pub(self) fn {snake(sig)}(self: &{cls}{params});\n")
            fn = f"{snake(cls)}_{snake(sig)}"
            arg_ty = "(" + "".join(f"{rust_payload(m)}, " for m in mapped) + ")"
            closure = "".join(f", a{i}: {m[1]}" for i, m in enumerate(mapped))
            values = "(" + "".join(f"{convert_payload(m, f'a{i}')}, " for i, m in enumerate(mapped)) + ")"
            connectors.append(
                f"/// `{cls}::{sig}`\n/// # Safety\n/// See module header.\n"
                f"pub unsafe fn {fn}(sender: *const ffi::QObject, token: SignalToken<{arg_ty}>) -> Option<Box<dyn Any>> {{\n"
                f"    let sender = unsafe {{ ffi::as_{cls}(sender) }};\n"
                f"    if sender.is_null() {{\n        return None;\n    }}\n"
                f"    let sender = unsafe {{ &*sender }};\n"
                f"    Some(Box::new(sender.on_{snake(sig)}(move |_sender{closure}| token.emit({values}))))\n}}\n\n")
            total += 1
        for rust_name, cxx_name, ret in CONST_METHODS.get(cls, []):
            bridge.append(f"        #[cxx_name = \"{cxx_name}\"]\n        fn {rust_name}(self: &{cls}) -> {ret};\n")
            connectors.append(
                f"/// `{cls}::{cxx_name}() const` through a shared `&T` receiver.\n/// # Safety\n/// See module header.\n"
                f"pub unsafe fn {snake(cls)}_{rust_name}(object: *const ffi::QObject) -> Option<{ret}> {{\n"
                f"    let object = unsafe {{ ffi::as_{cls}(object) }};\n"
                f"    (!object.is_null()).then(|| unsafe {{ &*object }}.{rust_name}())\n}}\n\n")
        bridge.append("    }\n")
    bridge.append("}\n")

    wrappers = ["// auto-generated by tools/signals.py, do not edit\n",
                "//! Typed `on_*` signal connections for every wrapper, including signals\n",
                "//! inherited from Qt/DTK base classes. See dtk/src/signals.rs.\n",
                "#![allow(clippy::all, unused_imports)]\n",
                "use crate::signals::{Connection, connect_typed};\nuse dtk_sys::native_signal::OwnedValue;\n\n"]
    for rust in sorted(plans):
        methods = plans[rust]
        if not methods:
            continue
        wrappers.append(f"impl crate::{rust} {{\n")
        for method, cls, sig, mapped, decl in methods:
            fn = f"{snake(cls)}_{snake(sig)}"
            arg_names = ", ".join(f"a{i}" for i in range(len(mapped)))
            fn_args = ", ".join(rust_value_type(m[4], m[2]) for m in mapped)
            pattern = "(" + "".join(f"a{i}, " for i in range(len(mapped))) + ")"
            payload = "(" + "".join(f"{rust_payload(m)}, " for m in mapped) + ")"
            call = ", ".join(rust_unpack(m, f"a{i}") for i, m in enumerate(mapped))
            wrappers.append(
                f"    /// `{' '.join(decl.split())}` (declared in `{cls}`).\n"
                f"    pub fn {method}(&self, mut f: impl FnMut({fn_args}) + 'static) -> Connection {{\n"
                f"        connect_typed(self, move |{pattern}: {payload}| f({call}), dtk_cxxqt::connectors::{fn})\n"
                f"    }}\n")
            del arg_names
        wrappers.append("}\n\n")

    rep = ["# CXX-Qt typed signal coverage report\n\n",
           "Generated by `tools/signals.py` from `Q_SIGNALS:` sections of every exported Qt/DTK class "
           "reachable from a dtk-rs QObject wrapper. Each declaring class is imported once; wrappers "
           "connect inherited signals through a checked `qobject_cast`.\n\n"]
    skipped = sum(1 for rows in report.values() for r in rows if r[2])
    rep.append(f"imported classes: {len(imports)}, signals generated: {total}, skipped: {skipped}, "
               f"wrapper methods: {sum(len(v) for v in plans.values())}\n\n")
    from collections import Counter
    reasons = Counter(r[2].split("`")[0].strip() for rows in report.values() for r in rows if r[2])
    rep.append("## skip reasons\n" + "".join(f"- {k}: {v}\n" for k, v in reasons.most_common()) + "\n")
    for cls in sorted(report):
        rows = report[cls]
        ok = sum(1 for r in rows if not r[2])
        rep.append(f"## {cls} — {ok} generated, {len(rows) - ok} skipped\n")
        for decl, fn, why in rows:
            d = " ".join(decl.split())
            rep.append(f"- `{d}` → `{fn}`\n" if fn else f"- `{d}` ← {why}\n")
        rep.append("\n")
    outputs = {
        "dtk-cxxqt/src/bridge.rs": rustfmt("".join(bridge)),
        "dtk-cxxqt/src/connectors.rs": rustfmt("".join(connectors)),
        "dtk-cxxqt/include/dtk_cxxqt_casts.h": "".join(casts_h),
        "dtk/src/signals_gen.rs": rustfmt("".join(wrappers)),
        "GEN_SIGNAL_REPORT.md": "".join(rep),
    }
    for rel, content in outputs.items():
        path = os.path.join(REPO, rel)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        old = open(path, encoding="utf-8").read() if os.path.exists(path) else None
        if old != content:
            with open(path, "w", encoding="utf-8") as f:
                f.write(content)
    print(f"signals: classes {len(imports)}, signals generated {total}, skipped {skipped}")


def rust_payload(m):
    """Owned type queued by the token (built in the emission frame)."""
    kind, value = m[4], m[2]
    if kind == "heap":
        target = "QPersistentModelIndex" if value == "QModelIndex" else value
        return f"OwnedValue<dtk_sys::ffi::{target}>"
    if kind == "qval":
        return f"cxx_qt_lib::{value}"
    if kind == "list":
        inner = value[4:-1]
        return f"Vec<cxx_qt_lib::{inner}>" if inner in CXXQT_VALUES else value
    return value


def convert_payload(m, name):
    kind, value = m[4], m[2]
    if kind == "heap":
        ns_value = value
        _ns, _inc, deleter = HEAP_VALUES[ns_value]
        target = "QPersistentModelIndex" if value == "QModelIndex" else value
        return (f"unsafe {{ OwnedValue::new(ffi::clone_{value}({name}) as *mut dtk_sys::ffi::{target}, "
                f"dtk_sys::ffi::{deleter}) }}")
    return m[3].format(name)


def rust_unpack(m, name):
    kind, value = m[4], m[2]
    if kind == "heap":
        target = "PersistentModelIndex" if value == "QModelIndex" else value
        return f"crate::{target}::from_raw({name}.into_raw())"
    return name


def rustfmt(content):
    return subprocess.run(["rustfmt", "--edition", "2024"], input=content, capture_output=True,
                          text=True, check=True).stdout
