"""Regression tests for generated wrapper ancestry (run with python3 -m unittest)."""
import glob
import os
import re
import unittest
from pathlib import Path

from tools.gen import HAND_BOUND, HDR_DIR, REPO, METHOD_RE, classify_class, enum_members, enum_getter_name, parse_header, rust_enum_variant, snake, Ctx


class QtAncestryTests(unittest.TestCase):
    def test_cpp_identifiers_with_native_underscores(self):
        for name, expected in (("PS_Printer", "p_s_printer"), ("PS_PageRange", "p_s_page_range"),
                               ("DComboBox", "d_combo_box"), ("_Printer", "_printer")):
            with self.subTest(name=name):
                self.assertEqual(snake(name), expected)
        self.assertEqual(enum_getter_name("DPrintPreviewSettingInfo::SettingType", "PS_Printer"),
                         "gen_enum_d_print_preview_setting_info_setting_type_p_s_printer")
        self.assertNotIn("__", enum_getter_name("SomeClass::Kind", "_Printer"))
        self.assertEqual(rust_enum_variant("PS_Printer"), "PsPrinter")
        self.assertEqual(rust_enum_variant("PS_NUpPrinting"), "PsNUpPrinting")
        self.assertEqual(rust_enum_variant("ConfigHome"), "ConfigHome")
        for stem in ("gen", "core", "gui"):
            bridge = Path(REPO, f"dtk-sys/src/{stem}_ffi.rs").read_text()
            identifiers = re.findall(r"\b(?:fn|type)\s+(\w+)", bridge)
            self.assertTrue(identifiers)
            for identifier in identifiers:
                with self.subTest(bridge=stem, identifier=identifier):
                    self.assertNotIn("__", identifier)
                    self.assertFalse(re.match(r"_[A-Z]", identifier))

    def test_native_enum_members_exclude_dtk5_alias(self):
        source = '''enum class XDG {\nDataHome,\nConfigHome,\n#if DTK_VERSION < DTK_VERSION_CHECK(6, 0, 0, 0)\nLegacy [[deprecated]] = ConfigHome,\n#endif\nStateHome\n};'''
        self.assertEqual(enum_members(source, "XDG"), ["DataHome", "ConfigHome", "StateHome"])
        with self.assertRaises(ValueError):
            enum_members("enum class XDG {\n#if UNKNOWN\nStateHome,\n#endif\n};", "XDG")

    def test_core_gui_pilot_generated_bridges(self):
        root = Path(REPO)
        core = (root / "dtk/src/core.rs").read_text()
        gui = (root / "dtk/src/gui.rs").read_text()
        self.assertIn("pub fn home_path() -> String", core)
        self.assertIn("pub fn path(type_: XdgDirectory) -> String", core)
        self.assertIn("pub fn path_2(type_: DsgDirectory) -> String", core)
        self.assertIn("pub fn writable_location(type_: StandardLocation) -> String", core)
        self.assertIn("options: LocateOption", core)
        self.assertIn("Self::ConfigHome => genffi::gen_enum_d_standard_paths_x_d_g_config_home()", core)
        self.assertIn("pub unsafe fn set_mode(mode: i32)", core)
        self.assertIn("pub fn find_executable(executableName: &str, paths: Vec<String>)", core)
        self.assertIn("pub fn operating_system_name() -> String", core)
        self.assertIn("pub fn memory_total_size() -> i64", core)
        self.assertIn("opaque_wrapper!(DSysInfo, genffi::DSysInfo);", core)
        self.assertIn("core_object_wrapper!(DConfig, genffi::DConfig", core)
        self.assertNotIn("crate::require_application()", core)
        self.assertRegex(core, r"pub unsafe fn set_app_id\(appId: &str\) \{\s*let _call = dtk_sys::NativeCall::new\(\);\s*unsafe")
        self.assertIn("pub fn is_valid(&self) -> bool", core)
        self.assertIn("pub fn key_list(&self) -> Vec<String>", core)
        self.assertIn("core_config_create_generic", (root / "dtk/src/lib.rs").read_text())
        self.assertIn("pub fn instance() -> Option<DGuiApplicationHelper>", gui)
        self.assertIn("pub fn adjust_color_2(", gui)
        self.assertIn("pub unsafe fn set_attribute(attribute: i32, enable: bool)", gui)
        self.assertIn("pub fn set_palette_type(&self, paletteType: GuiColorType)", gui)
        self.assertIn("pub fn set_size_mode(&self, mode: GuiSizeMode)", gui)
        self.assertIn("pub fn application_palette(&self) -> DPalette", gui)
        self.assertIn("pub fn generate_palette(base: &mut DPalette, type_: GuiColorType)", gui)
        self.assertIn("value_wrapper!(DPalette, ffi::DPalette, dpalette_delete);", (root / "dtk/src/lib.rs").read_text())
        self.assertIn("pub fn set_single_instance(key: &str, singleScope: GuiSingleScope)", gui)
        self.assertIn("pub fn gray_scale_color_filter(img: &QImage) -> QImage", gui)
        self.assertIn("pub fn rotate_image(&self, image: &mut QImage, angle: i32) -> bool", gui)
        self.assertIn("gen_d_image_handler_as_qobject", gui)
        self.assertIn("pub enum FontSizeType", gui)
        self.assertNotIn("NSizeTypes,", gui)  # sentinel is not a valid input
        self.assertIn("pub fn font_manager(&self) -> Option<DFontManager>", gui)
        self.assertIn("pub fn font_pixel_size(&self, type_: FontSizeType) -> i32", gui)
        self.assertIn("pub fn set_font_pixel_size(&self, type_: FontSizeType, size: i32)", gui)
        self.assertIn("pub fn t6(&self) -> QFont", gui)
        self.assertIn("using Dtk::Core::DStandardPaths;", (root / "dtk-sys/include/dtk_core_shim.h").read_text())
        self.assertIn("using Dtk::Gui::DGuiApplicationHelper;", (root / "dtk-sys/include/dtk_gui_shim.h").read_text())
        for report in ("GEN_CORE_REPORT.md", "GEN_GUI_REPORT.md"):
            self.assertIn("NOT library-wide coverage", (root / report).read_text())

    def test_core_and_gui_header_namespaces(self):
        cases = (
            ("DCore", "dstandardpaths.h", "DStandardPaths", "Dtk::Core", "path"),
            ("DCore", "dconfig.h", "DConfig", "Dtk::Core", "value"),
            ("DCore", "dsysinfo.h", "DSysInfo", "Dtk::Core", "operatingSystemName"),
            ("DGui", "dguiapplicationhelper.h", "DGuiApplicationHelper", "Dtk::Gui", "instance"),
        )
        for domain, header, name, namespace, method in cases:
            with self.subTest(name=name):
                parsed = parse_header(f"/usr/include/dtk6/{domain}/{header}", Ctx(set()))
                cls = next(c for c in parsed if c["name"] == name)
                self.assertEqual(cls["ns"], namespace)
                self.assertIn(method, {m[2] for m in cls["methods"]})
        self.assertEqual(classify_class("DConfig", {"DConfig": ["QObject", "DObject"]}), "object")

    def test_inherited_widget_not_just_prefix(self):
        bases = {
            "DBoxWidget": ["QWidget", "DObject"],
            "DHBoxWidget": ["DBoxWidget"],
            "DComboBox": ["QComboBox"],
        }
        self.assertEqual(classify_class("DHBoxWidget", bases), "widget")
        self.assertEqual(classify_class("DComboBox", bases), "widget")

    def test_qobject_is_not_widget(self):
        bases = {
            "DMessageManager": ["QObject", "DObject"],
            "DSubObject": ["DMessageManager"],
            "DDelegate": ["QStyledItemDelegate", "DObject"],
        }
        for name in bases:
            self.assertEqual(classify_class(name, bases), "object")

    def test_non_qobject_must_not_have_qobject_or_widget_apis(self):
        bases = {
            "DStylePainter": ["QPainter"],
            "DPrinter": ["QPrinter"],
            "DStyleOption": [],
            "DStyleOptionIcon": ["QStyleOption", "DStyleOption"],
            "DUnknown": ["DUnknownBase"],
        }
        for name in bases:
            self.assertEqual(classify_class(name, bases), "opaque")

    def test_local_dtk_headers_match_qt_ancestry(self):
        if not os.path.isdir(HDR_DIR):
            self.skipTest("DTK development headers unavailable")
        ctx = Ctx(set())
        bases = {}
        for header in glob.glob(os.path.join(HDR_DIR, "*.h")):
            if header.endswith("_p.h") or header.endswith("dwidgetstype.h"):
                continue
            for cls in parse_header(header, ctx):
                if cls["name"] not in HAND_BOUND:
                    bases[cls["name"]] = cls["bases"]
        for name, kind in {
            "DMessageManager": "object", "DStyledItemDelegate": "object",
            "DPrinter": "opaque", "DStylePainter": "opaque",
            "DStyleOptionButton": "opaque", "DComboBox": "widget",
            "DFileDialog": "widget", "DImageViewer": "widget",
        }.items():
            with self.subTest(name=name):
                self.assertIn(name, bases)
                self.assertEqual(classify_class(name, bases), kind)

    def test_generated_opaque_owner_and_borrowed_getter(self):
        generated = Path(REPO, "dtk/src/widgets.rs").read_text()
        cpp = Path(REPO, "dtk-sys/cpp/dtk_gen_shim.cpp").read_text()
        self.assertIn("opaque_wrapper!(DPrinter, genffi::DPrinter);", generated)
        self.assertIn("impl Default for OwnedOpaque<DPrinter>", generated)
        self.assertIn("void gen_d_printer_delete(DPrinter *self) { delete self; }", cpp)
        self.assertIn("pub fn global_provider() -> Option<DFileIconProvider>", generated)
        self.assertIn("opaque_wrapper!(DFileIconProvider, genffi::DFileIconProvider);", generated)
        self.assertRegex(generated, r"pub fn create_dialog_setting_info\(\s*&self,\s*type_: PrintSettingType,\s*\) -> Option<OwnedOpaque<DPrintPreviewSettingInfo>>")
        self.assertIn("void gen_d_print_preview_setting_info_delete(DPrintPreviewSettingInfo *self) { delete self; }", cpp)
        self.assertIn("pub fn instance() -> Option<DFontSizeManager>", generated)
        self.assertIn("pub unsafe fn create_view(&self)", generated)
        self.assertIn("pub unsafe fn create_placeholder_view(&self)", generated)
        self.assertNotIn("gen_d_styled_item_delegate_new()", cpp)

    def test_retained_pointer_rules_and_new_contracts_fail_closed(self):
        from tools.pointer_inputs import pointer_input_contract
        from tools.audit_hand import render
        with self.assertRaisesRegex(ValueError, "unaudited pointer input"):
            pointer_input_contract("NewContainer", "setWidget")
        self.assertEqual(render(), Path(REPO, "OWNERSHIP_HAND_REPORT.md").read_text())
        cpp = Path(REPO, "dtk-sys/cpp/dtk_gen_shim.cpp").read_text()
        self.assertIn("return loading_indicator_new();", cpp)
        self.assertIn("loading_indicator_set_widget(self, widgetSource)", cpp)
        self.assertIn("return drawer_group_checked(self);", cpp)
        self.assertIn("return feature_dialog_take(self, item);", cpp)
        self.assertIn("!layout_can_adopt(self, layout)", cpp)
        self.assertIn("anchors_clear(w);", cpp)
        self.assertIn("return anchors_set(w, p, target, point);", cpp)
        self.assertIn("return anchors_fill(self, fill, false);", cpp)
        self.assertIn("self->transition()->animation()->stop(); self->clear();", cpp)
        self.assertIn("feature_dialog_clear(self);", cpp)
        generated = Path(REPO, "dtk/src/widgets.rs").read_text()
        self.assertIn("pub enum AnchorPoint", generated)
        self.assertIn("pub fn remove_item(&self, item: &DFeatureItem) -> Option<Owned<DFeatureItem>>", generated)

    def test_every_generated_operation_brackets_native_call_and_return_wrapping(self):
        for domain in ("widgets", "core", "gui"):
            generated = Path(REPO, f"dtk/src/{domain}.rs").read_text()
            # Only native enum discriminant getters are truly independent
            # scalar operations. Every emitted constructor/method has a guard.
            methods = re.findall(r"pub (?:unsafe )?fn [^{]+\{(.*?)(?=\n    \}|\Z)", generated, re.S)
            self.assertTrue(methods)
            for body in methods:
                self.assertIn("let _call = dtk_sys::NativeCall::new();", body)
                self.assertLess(body.index("NativeCall::new()"), body.index("unsafe"))

    def test_output_pointer_requires_exclusive_owned_opaque_view(self):
        generated = Path(REPO, "dtk/src/widgets.rs").read_text()
        hand = Path(REPO, "dtk/src/lib.rs").read_text()
        self.assertIn("pub fn init_style_option(&self, option: &mut OwnedOpaque<DStyleOptionFloatingWidget>)", generated)
        self.assertNotIn("std::ops::DerefMut for OwnedOpaque", hand)
        self.assertIn("new DStyleOptionFloatingWidget{}", Path(REPO, "dtk-sys/cpp/dtk_gen_shim.cpp").read_text())
        self.assertNotRegex(hand + generated, r"(?:new_unchecked|transmute)[^\n]*Pin")

    def test_pointer_contracts_fail_closed_and_reports_cover_all_methods(self):
        from tools.ownership import POINTER_RETURNS, pointer_return_contract
        with self.assertRaisesRegex(ValueError, "unaudited pointer return"):
            pointer_return_contract("UnknownFactory", "getThing")
        for domain in ("WIDGET", "CORE", "GUI"):
            report = Path(REPO, f"OWNERSHIP_{domain}_REPORT.md").read_text()
            self.assertIn("matching apt source", report)
            self.assertIn("copied scalar / no result", report)
        report = Path(REPO, "OWNERSHIP_WIDGET_REPORT.md").read_text()
        for (cls, method), (contract, source, _) in POINTER_RETURNS.items():
            if cls == "DGuiApplicationHelper":
                continue
            with self.subTest(api=f"{cls}::{method}"):
                self.assertIn(f"`{cls}::{method}`", report)
                self.assertIn(f"`{source}`", report)
                self.assertIn(f"`{contract}`", report)

    def test_opaque_borrows_expire_but_owned_copies_remain_independent(self):
        root = Path(REPO)
        generated = (root / "dtk/src/widgets.rs").read_text()
        self.assertIn("OwnedOpaque::from_owned(Self::from_owned_raw(ptr)", generated)
        self.assertIn("OwnedOpaque::from_owned(WaterMarkData::from_owned_raw(ptr)", generated)
        self.assertRegex(generated, r"pub fn instance\(\) -> Option<DFontSizeManager> \{\s*crate::require_application\(\);")
        self.assertRegex(generated, r"pub fn global_provider\(\) -> Option<DFileIconProvider> \{\s*crate::require_application\(\);")
        self.assertIn("genffi::gen_d_font_size_manager_font_pixel_size(self.live_ptr(), type_)", generated)
        self.assertNotIn("genffi::gen_d_font_size_manager_font_pixel_size(self.ptr,", generated)
        self.assertRegex(generated, r"impl DPrinter \{\s*pub fn new\(\) -> OwnedOpaque<Self> \{\s*crate::require_application\(\);")
        self.assertIn("scope_guard: Option<dtk_sys::ObjectGuard>", (root / "dtk/src/lib.rs").read_text())

    def test_qt_layout_pointer_binding(self):
        generated = Path(REPO, "dtk/src/widgets.rs").read_text()
        report = Path(REPO, "GEN_REPORT.md").read_text()
        self.assertIn("pub fn set_layout(&self, layout: &QLayout)", generated)
        self.assertIn("pub fn insert_layout(&self, index: i32, layout: &QLayout)", generated)
        self.assertNotIn("unsupported param type: QLayout *", report)

    def test_qobject_constructor_checks_application_before_native_new(self):
        generated = Path(REPO, "dtk/src/widgets.rs").read_text()
        self.assertIn(
            "pub fn new() -> Owned<Self> {\n        crate::require_application();\n        "
            "let _call = dtk_sys::NativeCall::new();\n        "
            "Owned::new(Self::from_raw(unsafe { genffi::gen_d_list_view_new() }))",
            generated,
        )
        self.assertIn(
            "pub fn instance() -> Option<DMessageManager> {\n        crate::require_application();",
            generated,
        )

    def test_constructor_owns_but_takes_remain_borrowed(self):
        generated = Path(REPO, "dtk/src/widgets.rs").read_text()
        # every widget/object ctor_new type returns the RAII-conditional Owned<Self>,
        # never a plain, always-non-owning Self (that would silently leak top-level
        # objects the caller never explicitly deletes or parents).
        self.assertIn("crate::impl_default_owned!(", generated)
        self.assertNotIn("crate::impl_default!(", generated)
        # Do not infer ownership from names: local DTK6 takeHeaderWidget()
        # returns a still-parented widget native code deletes on the next event.
        self.assertIn(
            "pub fn take_header_widget(&self, index: i32) -> Option<QWidget>",
            generated,
        )
        self.assertIn(
            "pub fn take_footer_widget(&self, index: i32) -> Option<QWidget>",
            generated,
        )
        # a plain (non-"take") pointer getter must stay a borrowed, non-owning view
        self.assertIn(
            "pub fn get_header_widget(&self, index: i32) -> Option<QWidget>",
            generated,
        )

    def test_list_view_without_model_fails_safely(self):
        cpp = Path(REPO, "dtk-sys/cpp/dtk_gen_shim.cpp").read_text()
        self.assertIn("if (!self->model()) return 0; return self->count();", cpp)
        self.assertIn(
            "if (!self->model() || self->model()->columnCount() == 0) return false; return self->addItems(*datas);",
            cpp,
        )

    def test_qt_box_layout_pointer_binding(self):
        generated = Path(REPO, "dtk/src/widgets.rs").read_text()
        report = Path(REPO, "GEN_REPORT.md").read_text()
        self.assertIn("pub fn left_layout(&self) -> Option<QBoxLayout>", generated)
        self.assertIn("pub fn right_layout(&self) -> Option<QBoxLayout>", generated)
        self.assertNotIn("unsupported return type: QBoxLayout *", report)

    def test_qt_model_pointer_binding(self):
        generated = Path(REPO, "dtk/src/widgets.rs").read_text()
        cpp = Path(REPO, "dtk-sys/cpp/dtk_gen_shim.cpp").read_text()
        report = Path(REPO, "GEN_REPORT.md").read_text()
        self.assertIn("pub fn set_model(&self, model: &QAbstractItemModel)", generated)
        self.assertIn("gen_d_variant_list_model_as_model", cpp)
        self.assertNotIn("unsupported param type: QAbstractItemModel *", report)

    def test_qimage_value_mapping(self):
        generated = Path(REPO, "dtk/src/widgets.rs").read_text()
        report = Path(REPO, "GEN_REPORT.md").read_text()
        self.assertIn("pub fn set_image(&self, image: &QImage)", generated)
        self.assertIn("pub fn image(&self) -> QImage", generated)
        self.assertIn("pub fn data(&self) -> OwnedOpaque<WaterMarkData>", generated)
        self.assertIn("pub fn set_data(&self, data: &WaterMarkData)", generated)
        self.assertIn("return new WaterMarkData(self->data());", Path(REPO, "dtk-sys/cpp/dtk_gen_shim.cpp").read_text())
        self.assertNotIn("unsupported param type: const QImage &", report)
        self.assertNotIn("unsupported return type: QImage", report)
        self.assertNotIn("unsupported return type: const WaterMarkData &", report)

    def test_qhash_string_map_getter_is_owned(self):
        signature = "QHash<QString, QString> findAllMetaData();"
        self.assertIsNotNone(METHOD_RE.match(signature))
        root = Path(REPO)
        gui = (root / "dtk/src/gui.rs").read_text()
        self.assertIn("pub fn find_all_meta_data(&self) -> std::collections::BTreeMap<String, String>", gui)
        self.assertIn("crate::string_map_from_pairs(", gui)
        self.assertIn("to_rust_string_pairs(self->findAllMetaData())", (root / "dtk-sys/cpp/dtk_gui_shim.cpp").read_text())
        self.assertNotIn("findAllMetaData", (root / "GEN_GUI_REPORT.md").read_text())

    def test_qt_graphics_and_url_value_mapping(self):
        generated = Path(REPO, "dtk/src/widgets.rs").read_text()
        for signature in (
            "pub fn clip_path(&self) -> QPainterPath",
            "pub fn set_clip_path(&self, path: &QPainterPath)",
            "pub fn background(&self) -> QBrush",
            "pub fn set_background(&self, background: &QBrush)",
            "pub fn set_directory_url(&self, directory: &QUrl)",
            "pub fn directory_url(&self) -> QUrl",
            "pub fn key_sequence(&self) -> QKeySequence",
            "pub fn set_key_sequence(&self, keySequence: &QKeySequence)",
        ):
            with self.subTest(signature=signature):
                self.assertIn(signature, generated)

    def test_qvariant_value_mapping(self):
        generated = Path(REPO, "dtk/src/widgets.rs").read_text()
        report = Path(REPO, "GEN_REPORT.md").read_text()
        self.assertIn("pub fn set_tab_data(&self, index: i32, data: &QVariant)", generated)
        self.assertIn("pub fn tab_data(&self, index: i32) -> QVariant", generated)
        self.assertIn("pub fn plugin_mime_data() -> QVariant", generated)
        self.assertIn("pub fn add_items(&self, datas: &QVariantList)", generated)
        self.assertNotIn("unsupported param type: const QVariant &", report)
        self.assertNotIn("unsupported param type: const QVariantList &", report)

    def test_hand_dtk_line_edit_is_a_container_not_qlineedit_cast(self):
        cpp = Path(REPO, "dtk-sys/cpp/shim.cpp").read_text()
        hand = Path(REPO, "dtk/src/lib.rs").read_text()
        self.assertNotIn("static_cast<QLineEdit *>(w)->text()", cpp)
        self.assertIn("qobject_cast<Dtk::Widget::DLineEdit *>(w)", cpp)
        self.assertIn("ffi::line_edit_text(self.widget_ptr())", hand)
        self.assertIn("ffi::progressbar_value(self.widget_ptr())", hand)

    def test_qobject_casts_use_cpp_and_methods_validate_liveness(self):
        generated = Path(REPO, "dtk/src/widgets.rs").read_text()
        cpp = Path(REPO, "dtk-sys/cpp/dtk_gen_shim.cpp").read_text()
        self.assertIn(
            "QObject *gen_d_combo_box_as_qobject(DComboBox *self) { return static_cast<QObject *>(self); }",
            cpp,
        )
        self.assertIn(
            "QWidget *gen_d_combo_box_as_widget(DComboBox *self) { return static_cast<QWidget *>(self); }",
            cpp,
        )
        self.assertIn("gen_d_message_manager_as_qobject", cpp)
        self.assertNotIn("gen_d_printer_as_qobject", cpp)
        self.assertIn("genffi::gen_d_combo_box_show_popup(self.live_ptr())", generated)

    def test_dtk_aliases_and_cycles(self):
        bases = {
            "ColorButton": ["DPushButton"],
            "DImageViewer": ["DGraphicsView"],
            "CycleA": ["CycleB"],
            "CycleB": ["CycleA"],
        }
        self.assertEqual(classify_class("ColorButton", bases), "widget")
        self.assertEqual(classify_class("DImageViewer", bases), "widget")
        self.assertEqual(classify_class("CycleA", bases), "opaque")



class ByValueAndSignalTests(unittest.TestCase):
    def test_cxxqt_trivial_values_cross_by_value(self):
        root = Path(REPO)
        generated = (root / "dtk/src/widgets.rs").read_text()
        bridge = (root / "dtk-sys/src/gen_ffi.rs").read_text()
        self.assertIn("type QColor = cxx_qt_lib::QColor;", bridge)
        self.assertIn("pub fn set_alert_color(&self, c: &QColor)", generated)
        self.assertNotIn("color_delete", (root / "dtk-sys/src/lib.rs").read_text())
        cpp = (root / "dtk-sys/cpp/dtk_gen_shim.cpp").read_text()
        self.assertNotIn("new QColor(", cpp)
        self.assertNotIn("new QUrl(", cpp)

    def test_signal_parameter_mapping(self):
        from tools.signals import map_param
        enums = {("QAbstractSlider", "SliderAction"): "unscoped", ("Qt", "DropAction"): "unscoped",
                 ("QIcon", "Mode"): "scoped"}
        cases = {
            "bool checked": "prim", "const QString &text": "str", "QColor color": "qval",
            "const QUrl &url": "qval", "const QFont &font": "heap", "const QModelIndex &parent": "heap",
            "const QList<int> &roles = QList<int>()": "list", "int action": "prim",
        }
        for raw, kind in cases.items():
            with self.subTest(raw=raw):
                mapped, why = map_param(raw, "QAbstractSlider", [], enums)
                self.assertIsNone(why)
                self.assertEqual(mapped[4], kind)
        self.assertEqual(map_param("SliderAction action", "QAbstractSlider", [], enums)[0][4], "enum")
        self.assertEqual(map_param("Qt::DropAction a", "X", [], enums)[0][4], "enum")
        for raw in ("QAction *action", "QIcon::Mode mode", "Qt::MouseButtons b", "const QTextCharFormat &f"):
            with self.subTest(raw=raw):
                mapped, why = map_param(raw, "X", [], enums)
                self.assertIsNone(mapped)
                self.assertTrue(why)

    def test_generated_signals_cover_inherited_and_values(self):
        root = Path(REPO)
        gen = (root / "dtk/src/signals_gen.rs").read_text()
        bridge = (root / "dtk-cxxqt/src/bridge.rs").read_text()
        # Inherited Qt signals on a DTK wrapper, typed values, model indexes.
        blocks = {b.split(" {", 1)[0]: b for b in gen.split("\nimpl crate::")[1:]}
        self.assertIn("pub fn on_toggled(", blocks["widgets::DSwitchButton"])
        self.assertIn("pub fn on_mask_color_changed(&self, mut f: impl FnMut(crate::QColor) + 'static)", gen)
        self.assertIn("FnMut(crate::PersistentModelIndex, i32, i32)", gen)
        self.assertIn("FnMut(crate::PersistentModelIndex, crate::PersistentModelIndex, Vec<i32>)", gen)
        # Typedef aliases resolve to the Qt class; optional feature regions skipped.
        self.assertIn("pub fn on_clicked(&self, mut f: impl FnMut(bool)", blocks["DPushButton"])
        self.assertNotIn("mouse_pos_moving", bridge)
        code = "\n".join(line.split("//")[0] for line in bridge.splitlines())
        self.assertNotIn("Pin<", code)
        self.assertNotIn("&mut", code)
        self.assertEqual(len(re.findall(r"#\[qsignal\]", bridge)), len(re.findall(r"pub\(self\) fn", bridge)))


if __name__ == "__main__":
    unittest.main()
