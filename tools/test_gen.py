"""Regression tests for generated wrapper ancestry (run with python3 -m unittest)."""
import glob
import os
import unittest
from pathlib import Path

from tools.gen import HAND_BOUND, HDR_DIR, REPO, classify_class, parse_header, Ctx


class QtAncestryTests(unittest.TestCase):
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
        self.assertIn("owned_opaque_wrapper!(DPrinter, genffi::DPrinter);", generated)
        self.assertIn("impl Default for OwnedOpaque<DPrinter>", generated)
        self.assertIn("void gen_d_printer_delete(DPrinter *self) { delete self; }", cpp)
        self.assertIn("pub fn global_provider() -> Option<DFileIconProvider>", generated)
        self.assertIn("owned_opaque_wrapper!(DFileIconProvider, genffi::DFileIconProvider);", generated)

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
