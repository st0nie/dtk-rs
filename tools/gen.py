#!/usr/bin/env python3
"""Scan DTK6 widget headers -> generate C++ shim, cxx::bridge, Rust wrappers.

Usage: tools/gen.py   (idempotent, overwrites generated files)
Outputs:
  dtk-sys/include/dtk_{gen,core,gui}_shim.h  shim declarations
  dtk-sys/cpp/dtk_{gen,core,gui}_shim.cpp    shim implementations
  dtk-sys/src/{gen,core,gui}_ffi.rs          cxx::bridges
  dtk/src/{widgets,core,gui}.rs              wrappers
  GEN_REPORT.md, GEN_CORE_REPORT.md, GEN_GUI_REPORT.md  coverage reports

Widget coverage scans all DWidget headers. DCore/DGui currently select only
explicitly audited pilot classes; their reports do not cover whole libraries.

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
# Keep one explicit source-backed contract table for all generator backends.
# Script execution and module execution must resolve the same package.
if REPO not in sys.path:
    sys.path.insert(0, REPO)
from tools.ownership import IMPLICIT_VALUE_CONSTRUCTORS, POINTER_RETURNS, REQUIRED_PARENT_CONSTRUCTORS, pointer_return_contract
from tools.pointer_inputs import pointer_input_contract
from tools.native_rules import method_body

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

# Named enums used by the new domain wrappers. Member names come from the
# installed native headers; values are returned by tiny generated C++ functions
# rather than guessing the C++ discriminants in Rust.
TYPED_ENUMS = {
    "widget": {
        "Qt::AnchorPoint": ("AnchorPoint", "QtCore/qnamespace.h"),
        "DPrintPreviewSettingInfo::SettingType": ("PrintSettingType", "/usr/include/dtk6/DWidget/dprintpreviewsettinginfo.h", ("PS_SettingsCount",)),
    },
    "core": {
        "DStandardPaths::XDG": ("XdgDirectory", "/usr/include/dtk6/DCore/dstandardpaths.h"),
        "DStandardPaths::DSG": ("DsgDirectory", "/usr/include/dtk6/DCore/dstandardpaths.h"),
        "QStandardPaths::StandardLocation": ("StandardLocation", "QtCore/qstandardpaths.h"),
        "QStandardPaths::LocateOption": ("LocateOption", "QtCore/qstandardpaths.h"),
    },
    "gui": {
        "DGuiApplicationHelper::ColorType": ("GuiColorType", "/usr/include/dtk6/DGui/dguiapplicationhelper.h"),
        "DGuiApplicationHelper::SizeMode": ("GuiSizeMode", "/usr/include/dtk6/DGui/dguiapplicationhelper.h"),
        "DGuiApplicationHelper::SingleScope": ("GuiSingleScope", "/usr/include/dtk6/DGui/dguiapplicationhelper.h"),
        "DFontManager::SizeType": ("FontSizeType", "/usr/include/dtk6/DGui/dfontmanager.h", ("NSizeTypes",)),
    },
}


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


VALUE_TYPES = {"QColor", "QSize", "QPoint", "QRect", "QFont", "QPixmap", "QImage", "QBrush", "QPainterPath", "QKeySequence", "QUrl", "QIcon", "QPalette", "QMargins", "QVariant", "QVariantList"}
# cross-namespace (dtkgui) value types: name -> (fully-qualified namespace, include header)
EXT_VALUE_TYPES = {"DDciIcon": ("Dtk::Gui", "DDciIcon"),
                   "DPalette": ("Dtk::Gui", "DPalette")}
# Qt-class enums usable as i32 (header pulled in via the DTK widget headers)
QT_ENUMS = {"QLineEdit::EchoMode", "QTabBar::Shape", "QTabBar::ButtonPosition",
            "QTabBar::SelectionBehavior", "QStandardPaths::StandardLocation"}
# DTK-outer-class enums (dtkgui) usable as i32
EXT_ENUMS = {"DPalette::ColorType", "DPalette::ColorRole"}
# Qt pointer types allowed across the bridge (QWidget-style `as _` casts; each needs
# `type X;` in both bridges + a hand-written wrapper in dtk/src/lib.rs)
QT_PTRS = {"QWidget", "QAbstractButton", "QLayout", "QBoxLayout", "QAction", "QAbstractItemModel"}
# types that are QFlags in Qt (fromInt/toInt conversion)
QT_QFLAGS = {"Qt::Alignment", "Qt::WindowFlags", "Qt::MouseButtons", "Qt::KeyboardModifiers",
             "Qt::Orientations", "Qt::ItemFlags", "Qt::MatchFlags", "Qt::ApplicationStates",
             "Qt::InputMethodHints", "Qt::DockWidgetAreas", "Qt::ToolBarAreas",
             "QStandardPaths::LocateOptions"}

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

CLASS_RE = re.compile(r"^class\s+(?:(?:LIBDTK(?:WIDGET|GUI|CORE)SHARED_EXPORT|D_DECL_DEPRECATED)\s+)*(\w+)\s*(?::\s*(.+?))?\s*$")
METHOD_RE = re.compile(
    r"^\s*(?:virtual\s+|inline\s+|Q_SLOT\s+|Q_INVOKABLE\s+|D_DECL_DEPRECATED\s+|explicit\s+)*"
    r"(static\s+)?([\w:<>,&*~ ]+?)\s*(~?\w+)\s*\((.*)\)\s*(const)?\s*(?:override\s*)?(?:=\s*\w+\s*)?;?\s*(?://.*)?$"
)


RUST_KEYWORDS = {"type", "ref", "self", "mod", "fn", "in", "match", "loop", "move", "crate", "super",
                 "where", "impl", "trait", "const", "static", "mut", "pub", "use", "let", "if", "else",
                 "for", "while", "return", "struct", "enum", "unsafe", "extern", "box", "dyn", "as",
                 "async", "await", "break", "continue", "do", "final", "macro", "override", "priv",
                 "typeof", "unsized", "virtual", "yield", "try", "gen"}


def snake(name: str) -> str:
    # Native enum members can already contain underscores (PS_Printer).
    # Inserting an uppercase boundary must not emit C++'s reserved `__`.
    separated = re.sub(r"(?<!^)(?=[A-Z])", "_", name).lower()
    return re.sub(r"_+", "_", separated)


def enum_getter_name(cpp: str, member: str) -> str:
    # Normalize the complete identifier too: a member may start with `_`.
    return re.sub(r"_+", "_", "gen_enum_" + "_".join(snake(piece) for piece in cpp.split("::")) + "_" + snake(member))


def rust_enum_variant(member: str) -> str:
    if "_" not in member:
        return member
    return "".join(part.title() if part.isupper() else part[:1].upper() + part[1:]
                   for part in member.split("_") if part)


def enum_members(source: str, name: str) -> list[str]:
    """Extract explicitly visible enum member names, never compute C++ values."""
    source = re.sub(r"/\*.*?\*/", "", source, flags=re.S)
    source = re.sub(r"//[^\n]*", "", source)
    m = re.search(rf"\benum\s+(?:class\s+)?{re.escape(name)}\s*\{{([^}}]*)\}}", source, re.S)
    if not m:
        raise ValueError(f"enum {name} missing from installed header")
    members, dt_k5 = [], []
    for line in m.group(1).splitlines():
        s = line.strip()
        if not s:
            continue
        if s.startswith("#if"):
            if not re.match(r"#if\s+DTK_VERSION\s*<\s*DTK_VERSION_CHECK\(\s*6", s):
                raise ValueError(f"enum {name}: unrecognised conditional {s}")
            dt_k5.append(True)
            continue
        if s.startswith("#else"):
            if not dt_k5:
                raise ValueError(f"enum {name}: unmatched #else")
            dt_k5[-1] = not dt_k5[-1]
            continue
        if s.startswith("#endif"):
            if not dt_k5:
                raise ValueError(f"enum {name}: unmatched #endif")
            dt_k5.pop()
            continue
        if any(dt_k5):
            continue
        for part in s.split(","):
            head = part.split("=", 1)[0].strip()
            if not head:
                continue
            if not re.fullmatch(r"[A-Za-z_]\w*", head):
                raise ValueError(f"enum {name}: unsupported member {head}")
            members.append(head)
    if dt_k5 or not members:
        raise ValueError(f"enum {name}: incomplete member list")
    return members


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
        if is_return and re.fullmatch(r"QHash\s*<\s*QString\s*,\s*QString\s*>", t):
            return ("Vec<String>", "rust::Vec<rust::String>", "strmap", None)
        if "<" in t:
            return None
        ptr = t.endswith("*")
        base = t.rstrip("*").strip()
        base = base.replace("DTK_CORE_NAMESPACE::", "").replace("DGUI_NAMESPACE::", "").replace("DTK_GUI_NAMESPACE::", "")
        # Qt enums / QFlags (some are nested under a Qt class rather than Qt::)
        if base in QT_QFLAGS:
            return ("i32", "int32_t", "qflags", base)
        if base.startswith("Qt::"):
            return ("i32", "int32_t", "qtenum", base)
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
        # WaterMarkData explicitly defines a copy constructor. Both returned
        # values and returned const& must be heap-copied into OwnedOpaque<T>,
        # never let a reference to Qt's internal state escape to safe Rust.
        # Parameters accept const& (native setter copies the value).
        if base == "WaterMarkData" and not ptr and (is_return or "&" in cpp):
            return ("*mut WaterMarkData", "WaterMarkData *",
                    "valcopy" if is_return else "valref", base)
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
            # Expand the three DTK namespace macros without guessing a class's
            # origin from its name (DGui and DCore can use the same prefixes).
            nm = re.match(r"^D(WIDGET|GUI|CORE)_BEGIN_NAMESPACE\b", s)
            if nm:
                ns_stack.append("Dtk::" + {"WIDGET": "Widget", "GUI": "Gui", "CORE": "Core"}[nm.group(1)])
                continue
            if re.match(r"^D(?:WIDGET|GUI|CORE)_END_NAMESPACE\b", s):
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
                    ctx.register_enum(current_ns() or "Dtk::Widget", em.group(1))
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
        elif pkind in ("val", "valref"):
            # C++ non-const reference mutates the value. Preserve Rust's
            # exclusive borrow rather than accepting a shared &T to mutable
            # native storage (DGui::generatePalette(DPalette&) is an example).
            kind = ("valmut" if "&" in ptype and not re.search(r"\bconst\b", ptype)
                    else pkind)
            args.append((f"{pname}: *mut {pcls}", f"{pcpp} {pname}", f"*{pname}", kind, pcls))
        elif pkind == "qflags":
            args.append((f"{pname}: i32", f"int32_t {pname}", f"{q[3]}::fromInt({pname})", "qflags", q[3]))
        elif pkind in ("enum", "qtenum"):
            args.append((f"{pname}: i32", f"int32_t {pname}", f"static_cast<{q[3]}>({pname})", pkind, q[3]))
        else:
            args.append((f"{pname}: {prs}", f"{pcpp} {pname}", pname, "prim", None))
    return (ret_rs, ret_cpp, ret_kind, ret_cls, is_static, name, args), None


def collect_models(headers, excluded=(), included=None):
    """Parse a domain once and feed every backend the same type/ownership rules."""
    excluded = set(excluded)
    included = None if included is None else set(included)
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
    ctx = Ctx((all_classes if included is None else all_classes & included) - excluded)

    # two passes: parse everything (collecting cross-class enum refs), then generate
    parsed = []
    for h in headers:
        for c in parse_header(h, ctx):
            if c["name"] in excluded or included is not None and c["name"] not in included:
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
                if c["name"] == "DFeatureDisplayDialog" and name == "removeItem":
                    g = ("*mut DFeatureItem", "DFeatureItem *", "ptr", "DFeatureItem", *g[4:])
                if g[2] in ("ptr", "qtptr"):
                    pointer_return_contract(c["name"], g[5])
                if any(a[3] in ("ptr", "qtptr", "valref") for a in g[6]):
                    pointer_input_contract(c["name"], g[5])
                gen_methods.append(g)
        if c["name"] in REQUIRED_PARENT_CONSTRUCTORS:
            source, reason = REQUIRED_PARENT_CONSTRUCTORS[c["name"]]
            skipped.append((f"{c['name']}(parent = nullptr)", f"required native parent: {source}: {reason}"))
        classes_out.append({
            "name": c["name"], "kind": kind, "bases": c["bases"], "header": c["header"], "ns": c.get("ns", ""),
            # exactly one all-default ctor: two would make `new X()` ambiguous
            "ctor_new": (c.get("all_default_ctors", 0) == 1 or c["name"] in IMPLICIT_VALUE_CONSTRUCTORS) and not c.get("abstract", False) and c["name"] not in REQUIRED_PARENT_CONSTRUCTORS, "methods": gen_methods, "skipped": skipped,
        })
    return classes_out


def main():
    widget_headers = sorted(
        os.path.join(HDR_DIR, f) for f in os.listdir(HDR_DIR)
        if f.endswith(".h") and not f.endswith("_p.h") and f != "dwidgetstype.h"
    )
    emit(collect_models(widget_headers, HAND_BOUND))
    # DCore QObject wrappers require the owning QCoreApplication thread;
    # they do not inherit the Widget/DGui QApplication requirement.
    core_headers = [
        "/usr/include/dtk6/DCore/dstandardpaths.h",
        "/usr/include/dtk6/DCore/dsysinfo.h",
        "/usr/include/dtk6/DCore/dconfig.h",
    ]
    emit(collect_models(core_headers, included={"DStandardPaths", "DSysInfo", "DConfig"}), domain="core")
    gui_headers = [
        "/usr/include/dtk6/DGui/dguiapplicationhelper.h",
        "/usr/include/dtk6/DGui/dimagehandler.h",
        "/usr/include/dtk6/DGui/dfontmanager.h",
    ]
    emit(collect_models(gui_headers, included={"DGuiApplicationHelper", "DImageHandler", "DFontManager"}), domain="gui")


def emit(classes, domain="widget"):
    """Emit a domain using the same checked type mapper and owning wrapper rules."""
    assert domain in ("widget", "core", "gui")
    stem = "gen" if domain == "widget" else domain
    header = f"dtk_{stem}_shim.h"
    shim_h, shim_cpp, bridge, wrapper, report, ownership_report = [], [], [], [], [], []
    owned_opaque_types = {c["name"] for c in classes if c["kind"] == "opaque" and c["ctor_new"]}
    for c in classes:
        for method in c["methods"]:
            if method[2] in ("ptr", "qtptr") and pointer_return_contract(c["name"], method[5])[0] == "opaque_transfer":
                owned_opaque_types.add(method[3])
    ownership_report.append(f"# DTK6 {domain} pointer/return ownership audit\n\n"
                            "Generated from explicit native contracts in tools/ownership.py.\n"
                            "Evidence: matching apt source dtkwidget 6.7.47.1-1, dtkcore/dtkgui 6.7.47; Qt 6.8.0+dfsg-0deepin31.\n"
                            "All mapped return signatures and emitted constructors appear below. No ownership is inferred from a method name.\n\n"
                            "| Native API | Native result | Rust ownership | Evidence |\n"
                            "|---|---|---|---|\n")
    shim_h.append("// auto-generated by tools/gen.py, do not edit\n#pragma once\n#include \"dtk_shim.h\"\n")
    shim_cpp.append(f'// auto-generated by tools/gen.py, do not edit\n#include "{header}"\n#include "object_relations.h"\n#include "anchor_relations.h"\n#include <algorithm>\n\nnamespace dtkrs {{\n')
    # cxx requires an unsafe extern block when it contains genuinely safe C++
    # helpers (enum getters). Ordinary object methods remain explicitly unsafe.
    extern_block = 'unsafe extern "C++"' if TYPED_ENUMS.get(domain) else 'extern "C++"'
    bridge.append("// auto-generated by tools/gen.py, do not edit\n"
                  "// Every unsafe declaration has the same contract: pointers must refer to live\n"
                  "// Qt/DTK objects on their owning thread, with the correct dynamic type.\n"
                  "#[allow(clippy::missing_safety_doc, clippy::too_many_arguments)]\n"
                  "#[cxx::bridge(namespace = \"dtkrs\")]\n"
                  f"pub mod genffi {{\n    {extern_block} {{\n        include!(\"{header}\");\n")
    # QObject is used only for checked base casts, not for mapping arbitrary
    # QObject* method parameters until the Rust API has a general QObject view.
    bridge.append("        type QObject;\n")
    for qp in sorted(QT_PTRS):
        bridge.append(f"        type {qp};\n")
    for vt in sorted(VALUE_TYPES):
        bridge.append(f"        type {vt};\n")
    for vt in sorted(EXT_VALUE_TYPES):
        bridge.append(f"        type {vt};\n")
    wrapper.append("// auto-generated by tools/gen.py, do not edit\n#![allow(clippy::all, non_snake_case, unused_imports)]\nuse crate::{QAbstractButton, QAbstractItemModel, QAction, QBoxLayout, QLayout, Signal0, SignalBool, SignalI32, SignalI32I32, QWidget};\nuse crate::{QBrush, QColor, QFont, QIcon, QImage, QKeySequence, QMargins, QPainterPath, QPalette, QPixmap, QPoint, QRect, QSize, QUrl, QVariant, QVariantList};\nuse crate::{DDciIcon, DPalette, Owned, OwnedOpaque, QtLifecycle};\nuse dtk_sys::ffi;\n"
                   f"use dtk_sys::{stem}_ffi::genffi;\nuse std::marker::PhantomData;\n")
    title = "widget" if domain == "widget" else f"{domain} pilot"
    report.append(f"# DTK6 {title} binding coverage report\n")
    if domain != "widget":
        report.append("\n> Pilot scope: only the listed classes/headers were scanned; this is NOT library-wide coverage.\n")

    total_ok, total_skip = 0, 0
    used_headers = sorted({c["header"] for c in classes})
    for h in used_headers:
        shim_h.append(f"#include <{h}>\n")
    # cross-namespace value types (e.g. dtkgui DDciIcon): include + namespaced using
    for vt, (ns, inc) in EXT_VALUE_TYPES.items():
        shim_h.append(f"#include <{inc}>\n")
    shim_h.append("\nnamespace dtkrs {\n")
    for vt in sorted(VALUE_TYPES):
        shim_h.append(f"using ::{vt};\n")
    for vt, (ns, _inc) in EXT_VALUE_TYPES.items():
        shim_h.append(f"using {ns}::{vt};\n")
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

    domain_enums = {}
    enum_getters = set()
    for cpp, spec in TYPED_ENUMS.get(domain, {}).items():
        rust_name, header_path, *ignored = spec
        ignored_members = set(ignored[0]) if ignored else set()
        if not os.path.isabs(header_path):
            qt_includes = subprocess.check_output(
                ["pkg-config", "--variable=includedir", "Qt6Core"], text=True
            ).strip()
            header_path = os.path.join(qt_includes, header_path)
        with open(header_path, encoding="utf-8") as source:
            members = enum_members(source.read(), cpp.rsplit("::", 1)[-1])
        if ignored_members - set(members):
            raise ValueError(f"enum {cpp}: excluded members missing from native header")
        members = [name for name in members if name not in ignored_members]
        values = []
        variants = set()
        for member in members:
            fn = enum_getter_name(cpp, member)
            if fn in enum_getters:
                raise ValueError(f"enum getter identifier collision: {cpp}::{member} -> {fn}")
            enum_getters.add(fn)
            variant = rust_enum_variant(member)
            if variant in variants or variant in RUST_KEYWORDS or variant == "Self":
                raise ValueError(f"invalid/colliding Rust enum variant: {cpp}::{member} -> {variant}")
            variants.add(variant)
            shim_h.append(f"int32_t {fn}();\n")
            shim_cpp.append(f"int32_t {fn}() {{ return static_cast<int32_t>({cpp}::{member}); }}\n")
            bridge.append(f"        fn {fn}() -> i32;\n")
            values.append((variant, fn))
        domain_enums[cpp] = (rust_name, values)
        wrapper.append(f"#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub enum {rust_name} {{\n")
        wrapper.extend(f"    {member},\n" for member, _ in values)
        wrapper.append("}\n")
        wrapper.append(f"impl {rust_name} {{\n    fn native_value(self) -> i32 {{\n        match self {{\n")
        wrapper.extend(f"            Self::{member} => genffi::{fn}(),\n" for member, fn in values)
        wrapper.append("        }\n    }\n}\n")
    if domain == "core":
        # LocateOptions is QFlags<LocateOption>, so either enumerator is a
        # valid argument. Reuse the generated native values, not guessed bits.
        domain_enums["QStandardPaths::LocateOptions"] = domain_enums["QStandardPaths::LocateOption"]

    bridge.append("\n")
    for c in classes:
        name, sname = c["name"], snake(c["name"])
        used_names = {}
        # ctor
        if c["ctor_new"]:
            fn = f"gen_{sname}_new"
            shim_h.append(f"{name} *{fn}();\n")
            allocation = "loading_indicator_new()" if name == "DLoadingIndicator" else f"new {name}{{}}" if name in IMPLICIT_VALUE_CONSTRUCTORS else f"new {name}"
            shim_cpp.append(f"{name} *{fn}() {{ return {allocation}; }}\n")
            bridge.append(f"        unsafe fn {fn}() -> *mut {name};\n")
            ownership_report.append(f"| `{name}()` | `new {name}` | {'unique `OwnedOpaque`' if c['kind'] == 'opaque' else 'constructor `Owned`'} | constructor; `{c['header']}` |\n")
        if name in owned_opaque_types:
            # Also emit a deleter for transferred factory results without a
            # default constructor (DPrintPreviewSettingInfo's destructor is virtual).
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
            if ret_kind in ("ptr", "qtptr"):
                contract, source, evidence = pointer_return_contract(name, meth)
                ownership_report.append(f"| `{name}::{meth}` | `{ret_cls}*` | `{contract}` | `{source}`: {evidence} |\n")
            else:
                contract = "owned heap copy" if ret_kind in ("val", "valcopy") else "owned Rust strings/container" if ret_kind in ("str", "strlist", "strmap", "qba") else "copied scalar / no result"
                ownership_report.append(f"| `{name}::{meth}` | `{ret_rs}` | {contract} | shim conversion; `{c['header']}` |\n")
            pointer_args = [a for a in args if a[3] in ("ptr", "qtptr", "valref")]
            if pointer_args:
                input_contract, rule = pointer_input_contract(name, meth)
                params = ", ".join(f"{a[4]}*" for a in pointer_args)
                ownership_report.append(f"| `{name}::{meth}` parameters | `{params}` | {input_contract} | native `{c['header']}` implementation: {rule} |\n")
            value_args = [a for a in args if a[3] in ("val", "valmut")]
            if value_args:
                params = ", ".join(a[1] for a in value_args)
                contract = "exclusive mutable value borrow" if any(a[3] == "valmut" for a in value_args) else "call borrow of independently owned Qt value"
                ownership_report.append(f"| `{name}::{meth}` Qt value inputs | `{params}` | {contract}; no QObject ownership transfer | explicit value mapping; `{c['header']}` native value/reference signature |\n")
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
            elif ret_kind == "strmap":
                call = f"to_rust_string_pairs({call})"
            elif ret_kind == "qba":
                call = f"to_rust_string(QString({call}))"
            elif ret_kind in ("val", "valcopy"):
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
            body = method_body(name, meth, args, body, ret_rs)
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
        mac = {"widget": "widget_wrapper", "object": "object_wrapper", "opaque": "opaque_wrapper"}[c["kind"]]
        if domain == "core" and c["kind"] == "object":
            mac = "core_object_wrapper"
        runtime_check = "require_core_application" if domain == "core" else "require_application"
        casts = (f", genffi::gen_{sname}_as_qobject" if c["kind"] in ("widget", "object") else "")
        if c["kind"] == "widget":
            casts += f", genffi::gen_{sname}_as_widget"
        wrapper.append(f"{mac}!({name}, genffi::{name}{casts});\nimpl {name} {{\n")
        if owns_opaque:
            # QPrinter requires a GUI application; independent opaque values
            # retain their old no-app behavior, but none may reenter shutdown.
            ctor_check = f"        crate::{runtime_check}();\n" if name in {"DPrinter", "DBlurEffectGroup"} else "        dtk_sys::assert_not_closing();\n"
            wrapper.append(f"    pub fn new() -> OwnedOpaque<Self> {{\n"
                           f"{ctor_check}"
                           f"        let _call = dtk_sys::NativeCall::new();\n"
                           f"        let ptr = unsafe {{ genffi::gen_{sname}_new() }};\n"
                           f"        OwnedOpaque::from_owned(Self::from_owned_raw(ptr), "
                           f"|value| unsafe {{ genffi::gen_{sname}_delete(value.ptr) }})\n    }}\n")
        elif c["ctor_new"]:
            wrapper.append(f"    pub fn new() -> Owned<Self> {{\n        crate::{runtime_check}();\n        let _call = dtk_sys::NativeCall::new();\n        "
                           f"Owned::new(Self::from_raw(unsafe {{ genffi::gen_{sname}_new() }}))\n    }}\n")
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
                    output = name == "DFloatingWidget" and meth == "initStyleOption" and pcls == "DStyleOptionFloatingWidget"
                    wr_args.append(f"{pname}: &mut OwnedOpaque<{pcls}>" if output else f"{pname}: &{pcls}")
                    call_args.append(f"{pname}.live_ptr()")
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
                elif pkind in ("val", "valref", "valmut"):
                    wr_args.append(f"{pname}: {'&mut ' if pkind == 'valmut' else '&'}{pcls}")
                    source = f"{pname}.live_ptr()" if kind_by_name.get(pcls) == "opaque" else f"{pname}.ptr"
                    call_args.append(f"{source} as _")  # same audit as qtptr above
                elif pkind in ("enum", "qtenum", "qflags") and pcls in domain_enums:
                    rust_enum, _members = domain_enums[pcls]
                    wr_args.append(f"{pname}: {rust_enum}")
                    call_args.append(f"{pname}.native_value()")
                else:
                    wr_args.append(rs)
                    call_args.append(pname)
            self_piece = [] if is_static else ["&self"]
            sig = ", ".join(self_piece + wr_args)
            self_call = [] if is_static else ["self.live_ptr()"]
            calls = ", ".join(self_call + call_args)
            # Returns: wrap pointer getters in borrowed, nullable views, NOT
            # in Owned<T> merely because the method name starts with `take`.
            # In local DTK6, DListView::takeHeaderWidget() returns a widget
            # still parented to the view that native code destroys on the
            # next event turn. Only individually audited methods may hand
            # ownership to Rust (QMainWindow::takeCentralWidget is bound by hand).
            return_contract = pointer_return_contract(name, meth)[0] if ret_kind in ("ptr", "qtptr") else None
            if return_contract == "object_transfer":
                ret_decl = f" -> Option<Owned<{ret_cls}>>"
                expr = (f"{{ let ptr = unsafe {{ genffi::{fn_name}({calls}) }}; "
                        f"(!ptr.is_null()).then(|| Owned::new({ret_cls}::from_raw(ptr))) }}")
            elif return_contract == "opaque_transfer":
                ret_decl = f" -> Option<OwnedOpaque<{ret_cls}>>"
                expr = (f"{{ let ptr = unsafe {{ genffi::{fn_name}({calls}) }}; "
                        f"(!ptr.is_null()).then(|| OwnedOpaque::from_owned({ret_cls}::from_owned_raw(ptr), "
                        f"|value| unsafe {{ genffi::gen_{snake(ret_cls)}_delete(value.ptr) }})) }}")
            elif ret_kind == "ptr":
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
            elif ret_kind == "strmap":
                ret_decl = " -> std::collections::BTreeMap<String, String>"
                expr = f"crate::string_map_from_pairs(unsafe {{ genffi::{fn_name}({calls}) }})"
            elif ret_kind == "valcopy":
                ret_decl = f" -> OwnedOpaque<{ret_cls}>"
                expr = (f"{{ let ptr = unsafe {{ genffi::{fn_name}({calls}) }}; "
                        f"OwnedOpaque::from_owned({ret_cls}::from_owned_raw(ptr), "
                        f"|value| unsafe {{ genffi::gen_{snake(ret_cls)}_delete(value.ptr) }}) }}")
            else:
                ret_decl = "" if ret_rs == "()" else f" -> {ret_rs}"
                expr = f"unsafe {{ genffi::{fn_name}({calls}) }}"
            # Native DConfig::setAppId MUST run before QCoreApplication exists.
            # Requiring an existing GUI app here would invert that contract.
            pre_app_call = domain == "core" and name == "DConfig" and meth == "setAppId"
            precheck = (f"        crate::{runtime_check}();\n"
                        if is_static and not pre_app_call and (
                            c["kind"] in ("widget", "object") or ret_kind == "ptr"
                        ) else "        dtk_sys::assert_not_closing();\n" if is_static and c["kind"] == "opaque" else "")
            # Process-static opaque returns are source-proven borrows with
            # scope invalidation; factory allocations are unique owners above.
            # Only genuinely foreign implementation-defined factories retain
            # an unsafe contract. Never infer any of these from spelling.
            unguarded_return = return_contract == "implementation_defined"
            safety = ("    /// # Safety\n"
                      "    /// This pure virtual factory has no uniform native ownership contract.\n"
                      "    /// Audit the foreign implementation's allocation/ownership rules; this\n"
                      "    /// result is a borrowed checked view, not a deletion capability.\n"
                      if unguarded_return else "")
            if return_contract and not unguarded_return:
                _contract, source, evidence = pointer_return_contract(name, meth)
                safety = f"    /// Native ownership: `{return_contract}`.\n    /// Evidence: `{source}`: {evidence}.\n"
            # New DCore/DGui surfaces must not accept arbitrary i32 C++ enum
            # discriminants as safe Rust. Until a typed enum is generated,
            # callers must validate those arguments explicitly.
            domain_enum_args = domain in ("core", "gui") and any(
                arg[3] in ("enum", "qtenum", "qflags") and arg[4] not in domain_enums
                for arg in args
            )
            if domain_enum_args:
                if not safety:
                    safety = "    /// # Safety\n"
                safety += ("    /// Integer enum/flags arguments must be valid values of the\n"
                           "    /// corresponding native Qt/DTK type.\n")
            if pre_app_call:
                if not safety:
                    safety = "    /// # Safety\n"
                safety += ("    /// Must run before any QCoreApplication is created, without a\n"
                           "    /// concurrent Qt application construction on another thread.\n")
            marker = "unsafe " if unguarded_return or domain_enum_args or pre_app_call else ""
            if not is_static and name in {"DPrinter", "DBlurEffectGroup"}:
                precheck = f"        crate::{runtime_check}();\n"
            wrapper.append(f"{safety}    pub {marker}fn {mname}({sig}){ret_decl} {{\n{precheck}        let _call = dtk_sys::NativeCall::new();\n        {expr}\n    }}\n")
        wrapper.append("}\n\n")
        total_skip_n = len(c["skipped"])
        total_skip += total_skip_n
        report.append(f"\n## {name} — {len(c['methods'])} methods generated, {total_skip_n} skipped\n")
        for raw, why in c["skipped"]:
            report.append(f"- `{raw}` ← {why}\n")

    defaultables = [c["name"] for c in classes if c["ctor_new"] and c["kind"] != "opaque"]
    if defaultables:
        wrapper.append(f"crate::impl_default_owned!({', '.join(defaultables)});\n")
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
        (f"dtk-sys/include/dtk_{stem}_shim.h", "".join(shim_h)),
        (f"dtk-sys/cpp/dtk_{stem}_shim.cpp", "".join(shim_cpp)),
        (f"dtk-sys/src/{stem}_ffi.rs", rustfmt_text("".join(bridge))),
        ({"widget": "dtk/src/widgets.rs", "core": "dtk/src/core.rs", "gui": "dtk/src/gui.rs"}[domain],
         rustfmt_text("".join(wrapper))),
        ({"widget": "GEN_REPORT.md", "core": "GEN_CORE_REPORT.md", "gui": "GEN_GUI_REPORT.md"}[domain],
         "".join(report)),
        (f"OWNERSHIP_{domain.upper()}_REPORT.md", "".join(ownership_report)),
    ]:
        write(os.path.join(REPO, path), content)
    print(f"{domain}: classes {len(classes)}, methods generated {total_ok}, skipped {total_skip}")


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
