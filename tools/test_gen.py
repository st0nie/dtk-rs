"""Regression tests for generated wrapper ancestry (run with python3 -m unittest)."""
import glob
import os
import re
import unittest
from pathlib import Path

from tools.gen import HAND_BOUND, HDR_DIR, REPO, METHOD_RE, classify_class, enum_members, parse_header, Ctx


class QtAncestryTests(unittest.TestCase):
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
        self.assertIn("pub unsafe fn global_provider() -> Option<DFileIconProvider>", generated)
        self.assertIn("opaque_wrapper!(DFileIconProvider, genffi::DFileIconProvider);", generated)
        self.assertTrue(
            re.search(
                r"pub unsafe fn create_dialog_setting_info\(\s*&self,\s*type_: i32,\s*\) -> Option<DPrintPreviewSettingInfo>",
                generated,
            ),
            "unguarded DPrintPreviewSettingInfo getter must require unsafe",
        )
        self.assertIn("pub unsafe fn instance() -> Option<DFontSizeManager>", generated)

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


if __name__ == "__main__":
    unittest.main()
