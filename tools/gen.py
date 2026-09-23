#!/usr/bin/env python3
"""Scan DTK6 widget headers -> generate C++ shim, cxx::bridge, Rust wrappers.

Usage: tools/gen.py   (idempotent, overwrites generated files)
Outputs:
  dtk-sys/include/dtk_gen_shim.h   shim declarations
  dtk-sys/cpp/dtk_gen_shim.cpp     shim implementations
  dtk-sys/src/gen_ffi.rs           cxx::bridge
  dtk/src/widgets.rs               safe wrappers
  GEN_REPORT.md                    coverage report (with skip reasons)

Rules:
  - only generate methods whose param/return types all map cleanly; the rest go to the report
  - signals are not generated (DtkRelay connects by name at runtime; wrappers impl Signal0/SignalI32)
  - constructors: generate new() when a ctor exists with all-default args
"""
import os
import re
import subprocess
import sys

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
HDR_DIR = "/usr/include/dtk6/DWidget"

# classes bound by hand; the generator skips these
HAND_BOUND = {"DApplication", "DMainWindow", "DTitlebar", "DLabel", "DSuggestButton", "DPushButton",
              "DHiDPIHelper"}  # ponytail: deprecated upstream, skip to silence -Wdeprecated-declarations
# Classify inherited DTK types by their actual Qt ancestor. Falling back to
# "anything starting with Q/D is a QWidget" exposed as_widget() on QPainter,
# QPrinter, QObject and even QStyleOption — a pointer cast followed by UB.
QT_WIDGET_BASES = {
    "QWidget", "QMainWindow", "QDialog", "QFrame", "QLabel", "QPushButton", "QAbstractButton",
    "QComboBox", "QLineEdit", "QTextEdit", "QAbstractScrollArea", "QScrollArea", "QListView",
    "QTableView", "QTreeView", "QSlider", "QAbstractSlider", "QSpinBox", "QAbstractSpinBox",
    "QProgressBar", "QTabWidget", "QTabBar", "QMenu", "QMenuBar", "QToolBar", "QStatusBar",
    "QSplitter", "QStackedWidget", "QGroupBox", "QCheckBox", "QRadioButton", "QToolButton",
    "QDateTimeEdit", "QDateEdit", "QTimeEdit", "QCalendarWidget", "QDial", "QLCDNumber",
    "QListWidget", "QTableWidget", "QTreeWidget", "QColumnView",
    "QGraphicsView", "QFileDialog", "QDoubleSpinBox", "DGraphicsView", "DWidget",
    "DPushButton", "DLabel",  # aliases in dwidgetstype.h / hand-bound wrappers
}
QT_OBJECT_BASES = {
    "QObject", "QLayout", "QGraphicsEffect", "QAbstractItemModel", "QAbstractListModel",
    "QAbstractTableModel", "QCommonStyle",
    "QAction", "QStyledItemDelegate", "QButtonGroup", "DPlatformHandle",  # DGui QObject base
}
QT_MODEL_BASES = {"QAbstractItemModel", "QAbstractListModel", "QAbstractTableModel"}


def classify_class(name: str, bases_by_class: dict[str, list[str]]) -> str:
    """Return widget/object/opaque; unknown Qt/DTK bases are opaque, never guessed."""
    def classify_base(base: str, visited: set[str]) -> str:
        if base in QT_WIDGET_BASES:
            return "widget"
        if base in QT_OBJECT_BASES:
            return "object"
        if base in visited or base not in bases_by_class:
            return "opaque"
        visited = visited | {base}
        inherited = (classify_base(parent, visited) for parent in bases_by_class[base])
        return max(inherited, key={"opaque": 0, "object": 1, "widget": 2}.get, default="opaque")

    return classify_base(name, set())


VALUE_TYPES = {"QColor", "QSize", "QPoint", "QRect", "QFont", "QPixmap", "QIcon", "QPalette", "QMargins", "QVariant", "QVariantList"}
# cross-namespace (dtkgui) value types: name -> (fully-qualified namespace, include header)
EXT_VALUE_TYPES = {"DDciIcon": ("Dtk::Gui", "DDciIcon")}
# cross-namespace classes we only need enums from (include + using in the gen shim header)
EXT_CLASSES = {"DPalette": ("Dtk::Gui", "DPalette")}
# Qt-class enums usable as i32 (header pulled in via the DTK widget headers)
QT_ENUMS = {"QLineEdit::EchoMode", "QTabBar::Shape", "QTabBar::ButtonPosition",
            "QTabBar::SelectionBehavior"}
# DTK-outer-class enums (dtkgui) usable as i32
EXT_ENUMS = {"DPalette::ColorType", "DPalette::ColorRole"}
# Qt pointer types allowed across the bridge (QWidget-style `as _` casts; each needs
# `type X;` in both bridges + a hand-written wrapper in dtk/src/lib.rs)
QT_PTRS = {"QWidget", "QAbstractButton", "QLayout", "QBoxLayout", "QAction", "QAbstractItemModel"}
# types that are QFlags in Qt (fromInt/toInt conversion)
QT_QFLAGS = {"Qt::Alignment", "Qt::WindowFlags", "Qt::MouseButtons", "Qt::KeyboardModifiers",
             "Qt::Orientations", "Qt::ItemFlags", "Qt::MatchFlags", "Qt::ApplicationStates",
             "Qt::InputMethodHints", "Qt::DockWidgetAreas", "Qt::ToolBarAreas"}

PRIM = {
    "void": "()", "bool": "bool", "int": "i32", "qint32": "i32", "short": "i16",
    "qint64": "i64", "qlonglong": "i64", "long": "i64",
    "quint32": "u32", "uint": "u32", "qulonglong": "u64", "quint64": "u64", "ulong": "u64",
    "qreal": "f64", "double": "f64", "float": "f32", "qint8": "i8", "quint8": "u8",
    "quint16": "u16",
}
CPP_OF_RUST = {"()": "void", "bool": "bool", "i32": "int32_t", "i16": "int16_t", "i64": "int64_t",
               "u32": "uint32_t", "u64": "uint64_t", "f64": "double", "f32": "float",
               "i8": "int8_t", "u8": "uint8_t", "u16": "uint16_t", "String": "rust::String"}

CLASS_RE = re.compile(r"^class\s+(?:LIBDTKWIDGETSHARED_EXPORT\s+|D_DECL_DEPRECATED\s+)*(\w+)\s*(?::\s*(.+?))?\s*$")
METHOD_RE = re.compile(
    r"^\s*(?:virtual\s+|inline\s+|Q_SLOT\s+|Q_INVOKABLE\s+|D_DECL_DEPRECATED\s+|explicit\s+)*"
    r"(static\s+)?([\w:<>&*~ ]+?)\s*(~?\w+)\s*\((.*)\)\s*(const)?\s*(?:override\s*)?(?:=\s*\w+\s*)?;?\s*(?://.*)?$"
)


RUST_KEYWORDS = {"type", "ref", "self", "mod", "fn", "in", "match", "loop", "move", "crate", "super",
                 "where", "impl", "trait", "const", "static", "mut", "pub", "use", "let", "if", "else",
                 "for", "while", "return", "struct", "enum", "unsafe", "extern", "box", "dyn", "as",
                 "async", "await", "break", "continue", "do", "final", "macro", "override", "priv",
                 "typeof", "unsized", "virtual", "yield", "try", "gen"}


def snake(name: str) -> str:
    return re.sub(r"(?<!^)(?=[A-Z])", "_", name).lower()


def split_params(s: str):
    """split params on commas, respecting <> () nesting"""
    out, depth, cur = [], 0, ""
    for ch in s:
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


class Ctx:
    """type-mapping context: knows all DTK class names and enums"""

    def __init__(self, classes):
        self.classes = classes  # set of DTK class names
        self.qenums = {}  # "Scope::Enum" -> True (includes non-exported nested classes)
        self.enums = {}  # unqualified enum name -> scope or "?" (ambiguous, disabled)

    def register_enum(self, scope: str, name: str):
        self.qenums[f"{scope}::{name}"] = True
        prev = self.enums.get(name)
        if prev is None:
            self.enums[name] = scope
        elif prev != scope:
            self.enums[name] = "?"  # ambiguous; unqualified use disabled

    def map_type(self, cpp: str, is_return: bool, scope: str | None = None):
        """returns (rust_type, cpp_shim_type, kind, info) or None (unsupported).
        kind: prim | str | ptr | qtptr | val | enum | qtenum | qflags"""
        t = cpp.strip()
        t = re.sub(r"\s+", " ", t)
        t = re.sub(r"\bconst\s+", "", t).replace("&", "").strip()
        if "<" in t:
            return None
        ptr = t.endswith("*")
        base = t.rstrip("*").strip()
        base = base.replace("DTK_CORE_NAMESPACE::", "").replace("DGUI_NAMESPACE::", "").replace("DTK_GUI_NAMESPACE::", "")
        # Qt enums / QFlags
        if base.startswith("Qt::"):
            kind = "qflags" if base in QT_QFLAGS else "qtenum"
            return ("i32", "int32_t", kind, base)
        # DTK enums: own class first -> qualified lookup -> unqualified global
        if scope and f"{scope}::{base}" in self.qenums:
            return ("i32", "int32_t", "enum", f"{scope}::{base}")
        if "::" in base:
            qual = base.replace("DTK_WIDGET_NAMESPACE::", "")
            if qual in QT_ENUMS:
                return ("i32", "int32_t", "qtenum", qual)
            if qual in EXT_ENUMS:
                return ("i32", "int32_t", "enum", qual)
            if qual in self.qenums and qual.split("::")[0] in self.classes:
                return ("i32", "int32_t", "enum", qual)
            return None  # non-exported nested-class enums etc; skip
        if base in self.enums and self.enums[base] != "?":
            sc = self.enums[base]
            if sc in self.classes or sc == "Dtk::Widget":
                return ("i32", "int32_t", "enum", f"{sc}::{base}")
            return None
        if ptr:
            if base in self.classes:
                return (f"*mut {base}", f"{base} *", "ptr", base)
            # only audited cross-bridge Qt classes (each needs `type X;` in both bridges)
            if base in QT_PTRS:
                return (f"*mut {base}", f"{base} *", "qtptr", base)
            return None
        # value types: heap-allocated opaque pointers
        if base in VALUE_TYPES | EXT_VALUE_TYPES.keys():
            return (f"*mut {base}", f"{base} *", "val", base)
        if base in PRIM:
            r = PRIM[base]
            if r == "()" and not is_return:
                return None
            return (r, CPP_OF_RUST[r], "prim", None)
        if base == "QString":
            return ("String", CPP_OF_RUST["String"], "str", None) if is_return else ("&str", "rust::Str", "str", None)
        if base == "QStringList":
            # by value both ways; shim converts via to_qstringlist / to_rust_string_vec
            return ("Vec<String>", "rust::Vec<rust::String>", "strlist", None)
        if base == "QByteArray":
            # QByteArray params need a QString->QByteArray hop; return as string via toUtf8
            return ("String", CPP_OF_RUST["String"], "qba", None) if is_return else ("&str", "rust::Str", "qba", None)
        return None


ENUM_RE = re.compile(r"^\s*enum\s+(?:class\s+)?(\w+)")


def parse_header(path, ctx):
    """parse one header -> [(class, bases, [method...], report_skip...)]"""
    classes = []
    cur = None
    nested = None  # non-exported nested class name (methods not generated, enums only registered)
    section = None  # None | 'pub' | 'other'
    prev_was_template = False  # True if previous non-blank line was `template<...>`
    # preprocessor stack: track `#if DTK_VERSION < DTK_VERSION_CHECK(6,...)` regions so we skip
    # DTK5-only classes/methods. Each frame is True if the region is DTK5-only (skip in DTK6).
    pp_stack = []
    ns_stack = []  # namespace scope: e.g. ["Dtk::Widget"] when inside DWIDGET_BEGIN_NAMESPACE
    pending = None  # multi-line declaration being accumulated
    body_depth = 0  # >0: skipping an inline method body
    def dtk5_active():
        return any(pp_stack)
    def current_ns():
        return "::".join(ns_stack)
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            line = line.rstrip("\n")
            s = line.strip()
            if not s:
                continue
            this_is_template = s.startswith("template") and "<" in s
            # track DTK5-only preprocessor regions
            if re.match(r"#if\s+DTK_VERSION\s*<\s*DTK_VERSION_CHECK\(\s*6", s):
                pp_stack.append(True)
                continue
            if s.startswith("#if"):
                pp_stack.append(False)
                continue
            if s.startswith("#else"):
                if pp_stack:
                    pp_stack[-1] = not pp_stack[-1]
                continue
            if s.startswith("#endif"):
                if pp_stack:
                    pp_stack.pop()
                continue
            # track namespace opens/closes (DWIDGET_BEGIN_NAMESPACE expands to namespace Dtk { namespace Widget {)
            if re.match(r"^DWIDGET_BEGIN_NAMESPACE\b", s):
                ns_stack.append("Dtk::Widget")
                continue
            if re.match(r"^DWIDGET_END_NAMESPACE\b", s):
                if ns_stack:
                    ns_stack.pop()
                continue
            if dtk5_active():
                prev_was_template = this_is_template
                continue
            # skip inline method bodies (opened by a previous declaration line)
            if body_depth > 0:
                body_depth = max(0, body_depth + s.count("{") - s.count("}"))
                continue
            # skip template classes: a `class X` immediately preceded by `template<...>` is a template
            # (cxx opaque types can't name template classes without args)
            is_template_cls = prev_was_template
            prev_was_template = this_is_template
            m = CLASS_RE.match(line)
            if m and cur is None and nested is None and not is_template_cls:
                bases = []
                if m.group(2):
                    bases = [b.strip().split("::")[-1] for b in re.findall(r"public\s+([\w:]+)", m.group(2))]
                cur = {"name": m.group(1), "bases": bases, "methods": [], "skipped": [], "ns": current_ns()}
                section = None
                continue
            # non-exported (nested/file-scope) class definition: track scope, do not generate methods
            nm = re.match(r"^\s*(?:class|struct)\s+(\w+)\b[^;{]*$", line)
            if nm and not m and nested is None:
                nested = nm.group(1)
                continue
            if nested is not None:
                if re.match(r"^\s*enum\s+(?:class\s+)?(\w+)", line):
                    ctx.register_enum(nested, re.match(r"^\s*enum\s+(?:class\s+)?(\w+)", line).group(1))
                if line.startswith("};"):
                    nested = None
                continue
            if cur is None:
                em = ENUM_RE.match(line)
                if em:
                    ctx.register_enum("Dtk::Widget", em.group(1))
                continue
            if line.startswith("};"):
                classes.append(cur)
                cur = None
                continue
            s = line.strip()
            em = ENUM_RE.match(s)
            if em and "(" not in s:
                ctx.register_enum(cur["name"], em.group(1))
                if "{" in s and "}" not in s:
                    body_depth = 1  # skip enum values until closing brace
                continue
            if re.match(r"^(public|protected|private)\s*(Q_SLOTS|slots)?:", s):
                section = "pub" if s.startswith("public") else "other"
                continue
            if re.match(r"^(Q_)?[Ss][Ii][Gg][Nn][Aa][Ll][Ss]", s) or s.startswith("Q_SIGNALS"):
                section = "other"
                continue
            # detect pure-virtual: mark class abstract regardless of access section
            if re.search(r"=\s*0\s*;", s):
                cur["abstract"] = True
            if section != "pub" or not s:
                continue
            # join multi-line declarations (signatures wrapped across lines)
            if pending is not None:
                s = pending + " " + s
                pending = None
            if s.count("(") > s.count(")"):
                pending = s
                continue
            if s.startswith(":"):
                continue  # ctor init list
            if s.startswith(("//", "/*", "*")):
                continue  # comment line
            # inline body on/after this line: keep only the declaration part.
            # braces in default args (`= {}`) are not bodies: only look after the param list
            if "{" in s:
                depth, end = 0, -1
                for i, ch in enumerate(s):
                    if ch == "(":
                        depth += 1
                    elif ch == ")":
                        depth -= 1
                        if depth == 0:
                            end = i
                            break
                brace = s.find("{", end + 1) if end >= 0 else s.find("{")
                lone_brace = s == "{"  # ctor body opening on its own line
                if (brace >= 0 and end >= 0) or lone_brace:  # body only follows a param list
                    rest = s[brace + 1:] if brace >= 0 else ""
                    d = 1 + rest.count("{") - rest.count("}")
                    if d > 0:
                        body_depth = d
                    s = "" if lone_brace else s[:brace].rstrip() + ";"
            if s.startswith(("{", "}", "~")):
                continue  # stray brace / destructor
            # destructor: `virtual ~Class()` or `~Class()` — METHOD_RE mis-absorbs the `~`
            if re.match(rf"^(?:virtual\s+)?~{cur['name']}\b", s):
                continue
            if any(k in s for k in ("Q_PROPERTY", "Q_DECLARE", "D_DECLARE", "typedef", "using ", "enum ", "struct ",
                                    "friend", "operator", "#", "D_DECL_DEPRECATED", "Q_OBJECT", "Q_ENUM", "Q_FLAG")):
                continue
            if "(" not in s:
                continue
            s = re.sub(r"\s*Q_DECL_\w+", "", s)  # strip noexcept/override macros
            # constructor: no return type, name == class name
            cm = re.match(rf"^\s*(?:explicit\s+)?{cur['name']}\s*\((.*)\)\s*;?\s*$", s)
            if cm:
                ps = split_params(cm.group(1))
                if not ps or all("=" in p for p in ps):
                    cur["all_default_ctors"] = cur.get("all_default_ctors", 0) + 1
                continue
            m = METHOD_RE.match(s)
            if not m:
                cur["skipped"].append((s[:80], "signature parse failed"))
                continue
            is_static, ret, name, params = bool(m.group(1)), m.group(2).strip(), m.group(3), m.group(4)
            if name.startswith("~") or name == cur["name"] and not ret:
                continue  # destructor / false match
            if name == cur["name"]:
                # constructor
                ps = split_params(params)
                if not ps or all("=" in p for p in ps):
                    cur["all_default_ctors"] = cur.get("all_default_ctors", 0) + 1
                continue
            cur["methods"].append((is_static, ret, name, split_params(params), s[:80]))
    return classes


def gen_method(ctx, cls, is_static, ret, name, params):
    """map one method -> emit code in three places. Returns failure reason string on error"""
    r = ctx.map_type(ret, is_return=True, scope=cls)
    if r is None:
        return None, f"unsupported return type: {ret}"
    ret_rs, ret_cpp, ret_kind, ret_cls = r
    args = []  # (rust_sig_piece, cpp_sig_piece, call_piece)
    for i, p in enumerate(params):
        p_no_default = p.split("=")[0].strip()
        # pull trailing identifier as name; everything before (incl. * or &) is the type.
        # rsplit on space mishandles `Type *name` (star sticks to name, type loses ptr).
        pm = re.match(r"^(.*?)(\b[A-Za-z_]\w*)\s*$", p_no_default)
        if pm and pm.group(1).strip():
            ptype, pname = pm.group(1).strip(), pm.group(2)
        else:
            ptype, pname = p_no_default, f"arg{i}"
        if pname in RUST_KEYWORDS:
            pname += "_"
        q = ctx.map_type(ptype, is_return=False, scope=cls)
        if q is None:
            return None, f"unsupported param type: {ptype}"
        prs, pcpp, pkind, pcls = q
        if pkind == "str":
            args.append((f"{pname}: &str", f"rust::Str {pname}", f"from_rust_str({pname})", "str", None))
        elif pkind == "strlist":
            args.append((f"{pname}: Vec<String>", f"rust::Vec<rust::String> {pname}",
                         f"to_qstringlist(std::move({pname}))", "strlist", None))
        elif pkind == "qba":
            args.append((f"{pname}: &str", f"rust::Str {pname}", f"QByteArray(from_rust_str({pname}).toUtf8())", "qba", None))
        elif pkind in ("ptr", "qtptr"):
            args.append((f"{pname}: *mut {pcls}", f"{pcpp} {pname}", pname, pkind, pcls))
        elif pkind == "val":
            args.append((f"{pname}: *mut {pcls}", f"{pcpp} {pname}", f"*{pname}", "val", pcls))
        elif pkind == "qflags":
            args.append((f"{pname}: i32", f"int32_t {pname}", f"{q[3]}::fromInt({pname})", "qflags", None))
        elif pkind in ("enum", "qtenum"):
            args.append((f"{pname}: i32", f"int32_t {pname}", f"static_cast<{q[3]}>({pname})", pkind, None))
        else:
            args.append((f"{pname}: {prs}", f"{pcpp} {pname}", pname, "prim", None))
    return (ret_rs, ret_cpp, ret_kind, ret_cls, is_static, name, args), None


def main():
    headers = sorted(
        os.path.join(HDR_DIR, f) for f in os.listdir(HDR_DIR)
        if f.endswith(".h") and not f.endswith("_p.h") and f != "dwidgetstype.h"
    )
    # first pass: collect class names to build the context
    all_classes = set()
    for h in headers:
        prev_t = False
        with open(h, encoding="utf-8", errors="replace") as f:
            for line in f:
                s = line.strip()
                if not s:
                    continue
                m = CLASS_RE.match(line)
                if m and not prev_t:
                    all_classes.add(m.group(1))
                prev_t = s.startswith("template") and "<" in s
    ctx = Ctx(all_classes - HAND_BOUND)

    # two passes: parse everything (collecting cross-class enum refs), then generate
    parsed = []
    for h in headers:
        for c in parse_header(h, ctx):
            if c["name"] in HAND_BOUND:
                continue
            c["header"] = os.path.basename(h)
            parsed.append(c)

    bases_by_class = {c["name"]: c["bases"] for c in parsed}
    classes_out = []
    for c in parsed:
            kind = classify_class(c["name"], bases_by_class)
            gen_methods = []
            skipped = c["skipped"]
            for is_static, ret, name, params, raw in c["methods"]:
                g, why = gen_method(ctx, c["name"], is_static, ret, name, params)
                if g is None:
                    skipped.append((raw, why))
                else:
                    gen_methods.append(g)
            classes_out.append({
                "name": c["name"], "kind": kind, "bases": c["bases"], "header": c["header"], "ns": c.get("ns", ""),
                # exactly one all-default ctor: two would make `new X()` ambiguous
                "ctor_new": c.get("all_default_ctors", 0) == 1 and not c.get("abstract", False), "methods": gen_methods, "skipped": skipped,
            })

    emit(classes_out)


def emit(classes):
    shim_h, shim_cpp, bridge, wrapper, report = [], [], [], [], []
    shim_h.append("// auto-generated by tools/gen.py, do not edit\n#pragma once\n#include \"dtk_shim.h\"\n")
    shim_cpp.append('// auto-generated by tools/gen.py, do not edit\n#include "dtk_gen_shim.h"\n\nnamespace dtkrs {\n')
    bridge.append("// auto-generated by tools/gen.py, do not edit\n"
                  "// Every unsafe declaration has the same contract: pointers must refer to live\n"
                  "// Qt/DTK objects on their owning thread, with the correct dynamic type.\n"
                  "#[allow(clippy::missing_safety_doc, clippy::too_many_arguments)]\n"
                  "#[cxx::bridge(namespace = \"dtkrs\")]\n"
                  "pub mod genffi {\n    extern \"C++\" {\n        include!(\"dtk_gen_shim.h\");\n")
    # QObject is used only for checked base casts, not for mapping arbitrary
    # QObject* method parameters until the Rust API has a general QObject view.
    bridge.append("        type QObject;\n")
    for qp in sorted(QT_PTRS):
        bridge.append(f"        type {qp};\n")
    for vt in sorted(VALUE_TYPES):
        bridge.append(f"        type {vt};\n")
    for vt in sorted(EXT_VALUE_TYPES):
        bridge.append(f"        type {vt};\n")
    wrapper.append("// auto-generated by tools/gen.py, do not edit\n#![allow(clippy::all, non_snake_case, unused_imports)]\nuse crate::{QAbstractButton, QAbstractItemModel, QAction, QBoxLayout, QLayout, Signal0, SignalBool, SignalI32, SignalI32I32, QWidget};\nuse crate::{QColor, QFont, QIcon, QMargins, QPalette, QPixmap, QPoint, QRect, QSize, QVariant, QVariantList};\nuse crate::{DDciIcon, OwnedOpaque};\nuse dtk_sys::ffi;\nuse dtk_sys::gen_ffi::genffi;\nuse std::marker::PhantomData;\n")
    report.append("# DTK6 widget binding coverage report\n")

    total_ok, total_skip = 0, 0
    used_headers = sorted({c["header"] for c in classes})
    for h in used_headers:
        shim_h.append(f"#include <{h}>\n")
    # cross-namespace value types (e.g. dtkgui DDciIcon): include + namespaced using
    for vt, (ns, inc) in EXT_VALUE_TYPES.items():
        shim_h.append(f"#include <{inc}>\n")
    for ec, (ns, inc) in EXT_CLASSES.items():
        shim_h.append(f"#include <{inc}>\n")
    shim_h.append("\nnamespace dtkrs {\n")
    for vt in sorted(VALUE_TYPES):
        shim_h.append(f"using ::{vt};\n")
    for vt, (ns, _inc) in EXT_VALUE_TYPES.items():
        shim_h.append(f"using {ns}::{vt};\n")
    for ec, (ns, inc) in EXT_CLASSES.items():
        shim_h.append(f"using {ns}::{ec};\n")
    for qp in sorted(QT_PTRS - {"QWidget"}):  # QWidget already via dtk_shim.h
        shim_h.append(f"using ::{qp};\n")
    for c in classes:
        name = c["name"]
        ns = c.get("ns", "Dtk::Widget")
        if ns:
            shim_h.append(f"using {ns}::{name};\n")
        else:
            shim_h.append(f"using ::{name};\n")
        bridge.append(f"        type {name};\n")

    bridge.append("\n")
    for c in classes:
        name, sname = c["name"], snake(c["name"])
        used_names = {}
        # ctor
        if c["ctor_new"]:
            fn = f"gen_{sname}_new"
            shim_h.append(f"{name} *{fn}();\n")
            shim_cpp.append(f"{name} *{fn}() {{ return new {name}; }}\n")
            bridge.append(f"        unsafe fn {fn}() -> *mut {name};\n")
            if c["kind"] == "opaque":
                # Unlike QObject parent-owned objects, opaque allocations need an
                # explicit deleter. Only the constructor's OwnedOpaque may call it.
                delete = f"gen_{sname}_delete"
                shim_h.append(f"void {delete}({name} *self);\n")
                shim_cpp.append(f"void {delete}({name} *self) {{ delete self; }}\n")
                bridge.append(f"        unsafe fn {delete}(self_: *mut {name});\n")
        if c["kind"] in ("object", "widget"):
            # C++ static_cast adjusts secondary base pointers; a Rust raw cast
            # is only safe for coincident primary-base layouts. Compute once per
            # wrapper and keep a QPointer to the adjusted QObject*.
            fn = f"gen_{sname}_as_qobject"
            shim_h.append(f"QObject *{fn}({name} *self);\n")
            shim_cpp.append(f"QObject *{fn}({name} *self) {{ return static_cast<QObject *>(self); }}\n")
            bridge.append(f"        unsafe fn {fn}(self_: *mut {name}) -> *mut QObject;\n")
        if c["kind"] == "widget":
            fn = f"gen_{sname}_as_widget"
            shim_h.append(f"QWidget *{fn}({name} *self);\n")
            shim_cpp.append(f"QWidget *{fn}({name} *self) {{ return static_cast<QWidget *>(self); }}\n")
            bridge.append(f"        unsafe fn {fn}(self_: *mut {name}) -> *mut QWidget;\n")
        if any(base in QT_MODEL_BASES for base in c["bases"]):
            fn = f"gen_{sname}_as_model"
            shim_h.append(f"QAbstractItemModel *{fn}({name} *self);\n")
            shim_cpp.append(f"QAbstractItemModel *{fn}({name} *self) {{ return static_cast<QAbstractItemModel *>(self); }}\n")
            bridge.append(f"        unsafe fn {fn}(self_: *mut {name}) -> *mut QAbstractItemModel;\n")
        for (ret_rs, ret_cpp, ret_kind, ret_cls, is_static, meth, args) in c["methods"]:
            base_fn = f"gen_{sname}_{snake(meth)}"
            if snake(meth) in RUST_KEYWORDS:
                base_fn += "_"  # same rule as the wrapper side, avoids Rust keywords
            n = used_names.get(base_fn, 0)
            used_names[base_fn] = n + 1
            # avoid `__` (reserved in C++) when the name already ends with `_` (keyword escape)
            fn = base_fn if n == 0 else f"{base_fn}{'' if base_fn.endswith('_') else '_'}{n + 1}"
            # shim signature
            self_arg = [] if is_static else [f"{name} *self"]
            cpp_sig = ", ".join(self_arg + [a[1] for a in args])
            call_args = ", ".join(a[2] for a in args)
            call = f"{name}::{meth}({call_args})" if is_static else f"self->{meth}({call_args})"
            if ret_kind == "str":
                call = f"to_rust_string({call})"
            elif ret_kind == "strlist":
                call = f"to_rust_string_vec({call})"
            elif ret_kind == "qba":
                call = f"to_rust_string(QString({call}))"
            elif ret_kind == "val":
                call = f"new {ret_cls}({call})"
            elif ret_kind in ("enum", "qtenum"):
                call = f"static_cast<int32_t>({call})"
            elif ret_kind == "qflags":
                call = f"({call}).toInt()"
            # const-correctness: wrap const returns in const_cast so the shim compiles
            if ret_kind == "ptr":
                call = f"const_cast<{ret_cls} *>({call})"
            body = f"return {call};" if ret_rs != "()" else f"{call};"
            # Native DListView::count/add*/insert*/remove* dereference a null
            # model on a newly constructed view (observed SIGSEGV in DTK6).
            # With a zero-column QStandardItemModel, addItems returns true but
            # cannot store data at (row, 0). Reject both unsafe/incoherent cases.
            if name == "DListView" and meth in {
                "count", "addItem", "addItems", "insertItem", "insertItems",
                "removeItem", "removeItems",
            }:
                fallback = "0" if meth == "count" else "false"
                invalid = "!self->model()" if meth == "count" else \
                    "!self->model() || self->model()->columnCount() == 0"
                body = f"if ({invalid}) return {fallback}; {body}"
            shim_h.append(f"{ret_cpp} {fn}({cpp_sig});\n")
            shim_cpp.append(f"{ret_cpp} {fn}({cpp_sig}) {{ {body} }}\n")
            # bridge
            rust_self = [] if is_static else [f"self_: *mut {name}"]
            rust_args = ", ".join(rust_self + [a[0] for a in args])
            ret_decl = "" if ret_rs == "()" else f" -> {ret_rs}"
            bridge.append(f"        unsafe fn {fn}({rust_args}){ret_decl};\n")
            total_ok += 1

    shim_h.append("\n} // namespace dtkrs\n")
    shim_cpp.append("\n} // namespace dtkrs\n")
    bridge.append("    }\n}\n")

    # ---- Rust wrapper ----
    kind_by_name = {c["name"]: c["kind"] for c in classes}
    for c in classes:
        name, sname = c["name"], snake(c["name"])
        owns_opaque = c["kind"] == "opaque" and c["ctor_new"]
        mac = ("owned_opaque_wrapper" if owns_opaque else
               {"widget": "widget_wrapper", "object": "object_wrapper", "opaque": "opaque_wrapper"}[c["kind"]])
        casts = (f", genffi::gen_{sname}_as_qobject" if c["kind"] in ("widget", "object") else "")
        if c["kind"] == "widget":
            casts += f", genffi::gen_{sname}_as_widget"
        wrapper.append(f"{mac}!({name}, genffi::{name}{casts});\nimpl {name} {{\n")
        if owns_opaque:
            wrapper.append(f"    pub fn new() -> OwnedOpaque<Self> {{\n"
                           f"        let ptr = unsafe {{ genffi::gen_{sname}_new() }};\n"
                           f"        OwnedOpaque::from_owned(Self::from_raw(ptr), "
                           f"|value| unsafe {{ genffi::gen_{sname}_delete(value.ptr) }})\n    }}\n")
        elif c["ctor_new"]:
            precheck = "crate::require_application();\n        " if c["kind"] != "opaque" else ""
            wrapper.append(f"    pub fn new() -> Self {{\n        {precheck}Self::from_raw(unsafe {{ genffi::gen_{sname}_new() }})\n    }}\n")
        used = {}
        # widget_wrapper macro already provides these; skip to avoid duplicate definitions
        MACRO_METHODS = {"show", "resize", "set_enabled", "set_window_title", "as_widget", "from_raw", "as_qobject", "new", "default"}
        for (ret_rs, _, ret_kind, ret_cls, is_static, meth, args) in c["methods"]:
            mname = snake(meth)
            if mname in RUST_KEYWORDS:
                mname += "_"
            n = used.get(mname, 0)
            used[mname] = n + 1
            mname = mname if n == 0 else f"{mname}{'' if mname.endswith('_') else '_'}{n + 1}"
            if mname in MACRO_METHODS:
                continue  # already provided by the macro
            fn_name = f"gen_{sname}_{mname}"  # same naming order/rule as shim/bridge
            # wrapper params: *mut X -> &X
            wr_args, call_args = [], []
            for rs, _cpp, _call, pkind, pcls in args:
                pname, ptype = rs.split(": ", 1)
                if pkind == "ptr":
                    wr_args.append(f"{pname}: &{pcls}")
                    source = f"{pname}.ptr" if kind_by_name[pcls] == "opaque" else f"{pname}.live_ptr()"
                    call_args.append(source)
                elif pkind == "qtptr":
                    wr_args.append(f"{pname}: &{pcls}")
                    # AUDIT(2026-02): casting between ffi::QWidget and genffi::QWidget is sound.
                    # cxx opaque C++ types are zero-sized Rust structs that are never
                    # dereferenced in Rust; both name the same C++ type ::QWidget via
                    # `using ::QWidget;` in namespace dtkrs (dtk_shim.h / dtk_gen_shim.h).
                    # A raw pointer cast moves no bits and assumes no layout.
                    # QWidget is now a guarded view: validate the live Qt object
                    # before passing its pointer into generated C++ code.
                    call_args.append(f"{pname}.live_ptr() as _")
                elif pkind == "val":
                    wr_args.append(f"{pname}: &{pcls}")
                    call_args.append(f"{pname}.ptr as _")  # same audit as qtptr above
                else:
                    wr_args.append(rs)
                    call_args.append(pname)
            self_piece = [] if is_static else ["&self"]
            sig = ", ".join(self_piece + wr_args)
            self_call = [] if is_static else ["self.ptr" if c["kind"] == "opaque" else "self.live_ptr()"]
            calls = ", ".join(self_call + call_args)
            # returns: wrap pointers in wrappers, pass values through
            if ret_kind == "ptr":
                # DTK/Qt pointer getters are often nullable (e.g. button(-1),
                # currentWidget() before adding a page). Never manufacture a
                # non-null wrapper for such a return.
                ret_decl = f" -> Option<{ret_cls}>"
                expr = (f"{{ let ptr = unsafe {{ genffi::{fn_name}({calls}) }}; "
                        f"(!ptr.is_null()).then(|| {ret_cls}::from_raw(ptr)) }}")
            elif ret_kind == "qtptr":
                ret_decl = f" -> Option<{ret_cls}>"
                expr = (f"{{ let ptr = unsafe {{ genffi::{fn_name}({calls}) }}; "
                        f"(!ptr.is_null()).then(|| {ret_cls}::from_raw(ptr as _)) }}")  # see qtptr audit above
            elif ret_kind == "val":
                ret_decl = f" -> {ret_cls}"
                expr = f"{ret_cls}::from_raw(unsafe {{ genffi::{fn_name}({calls}) }} as _)"  # see qtptr audit comment above
            else:
                ret_decl = "" if ret_rs == "()" else f" -> {ret_rs}"
                expr = f"unsafe {{ genffi::{fn_name}({calls}) }}"
            precheck = ("        crate::require_application();\n"
                        if is_static and c["kind"] in ("widget", "object") else "")
            wrapper.append(f"    pub fn {mname}({sig}){ret_decl} {{\n{precheck}        {expr}\n    }}\n")
        wrapper.append("}\n\n")
        total_skip_n = len(c["skipped"])
        total_skip += total_skip_n
        report.append(f"\n## {name} — {len(c['methods'])} methods generated, {total_skip_n} skipped\n")
        for raw, why in c["skipped"]:
            report.append(f"- `{raw}` ← {why}\n")

    defaultables = [c["name"] for c in classes if c["ctor_new"] and c["kind"] != "opaque"]
    if defaultables:
        wrapper.append(f"crate::impl_default!({', '.join(defaultables)});\n")
    for c in classes:
        if c["ctor_new"] and c["kind"] == "opaque":
            name = c["name"]
            wrapper.append(f"impl Default for OwnedOpaque<{name}> {{\n"
                           f"    fn default() -> Self {{ {name}::new() }}\n}}\n")

    kinds = {kind: sum(c["kind"] == kind for c in classes) for kind in ("widget", "object", "opaque")}
    report.insert(1, f"\nclasses: {len(classes)}, methods generated: {total_ok}, skipped: {total_skip}\n"
                     f"\nQt ancestry: {kinds['widget']} widgets, {kinds['object']} QObjects, {kinds['opaque']} opaque types\n")
    from collections import Counter
    reasons = Counter(why.split(":")[0] for c in classes for _, why in c["skipped"])
    report.insert(2, "\n## skip reasons\n" + "".join(f"- {k}: {v}\n" for k, v in reasons.most_common()))

    for path, content in [
        ("dtk-sys/include/dtk_gen_shim.h", "".join(shim_h)),
        ("dtk-sys/cpp/dtk_gen_shim.cpp", "".join(shim_cpp)),
        ("dtk-sys/src/gen_ffi.rs", rustfmt_text("".join(bridge))),
        ("dtk/src/widgets.rs", rustfmt_text("".join(wrapper))),
        ("GEN_REPORT.md", "".join(report)),
    ]:
        write(os.path.join(REPO, path), content)
    print(f"classes {len(classes)}, methods generated {total_ok}, skipped {total_skip}")


def write(path, content):
    """write only when changed (keeps mtime stable -> no pointless cargo rebuild). Returns changed."""
    try:
        with open(path, encoding="utf-8") as f:
            if f.read() == content:
                return False
    except FileNotFoundError:
        pass
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)
    return True


def rustfmt_text(content):
    """format Rust source in-memory (stdin/stdout), so write() can skip unchanged files"""
    return subprocess.run(
        ["rustfmt", "--edition", "2024"],
        input=content,
        capture_output=True,
        text=True,
        check=True,
    ).stdout


if __name__ == "__main__":
    main()
