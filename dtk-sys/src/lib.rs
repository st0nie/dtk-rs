//! DTK6 (dtkwidget) FFI layer. See cpp/shim.cpp for the C++ shim.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;

pub mod core_ffi;
pub mod gen_ffi;
pub mod gui_ffi;

// All raw FFI functions share one safety contract: pointers must be live Qt/DTK
// objects with the stated dynamic type, accessed on their owning GUI thread.
// The painter callback is an ABI boundary with fixed fields, not a Rust API.
#[allow(clippy::missing_safety_doc, clippy::too_many_arguments)]
#[cxx::bridge(namespace = "dtkrs")]
pub mod ffi {
    extern "C++" {
        include!("dtk_shim.h");

        type QObject;
        type QWidget;
        type QObjectGuard;
        /// Construct a Qt guarded pointer from a *live*, GUI-thread QObject.
        unsafe fn guard_object(object: *mut QObject) -> UniquePtr<QObjectGuard>;
        /// Same for QWidget; C++ performs the correct base-class pointer adjustment.
        unsafe fn guard_widget(widget: *mut QWidget) -> UniquePtr<QObjectGuard>;
        /// QPointer clears on Qt destruction, even when Rust still holds its guard.
        unsafe fn guard_alive(guard: &QObjectGuard) -> bool;
        /// True once Qt has parented the object, even if it was later detached.
        unsafe fn guard_was_parented(guard: &QObjectGuard) -> bool;
        type QAbstractButton;
        type QLayout;
        type QBoxLayout;
        type QAbstractItemModel;
        type QStandardItemModel;
        type QVBoxLayout;
        type QHBoxLayout;
        type QTableWidget;
        type QTableWidgetItem;
        type QTimer;
        type QIcon;
        type QColor;
        type QFont;
        type QPalette;
        type DPalette;
        type QPixmap;
        type QImage;
        type QBrush;
        type QPainterPath;
        type QKeySequence;
        type QUrl;
        type QMargins;
        type QVariant;
        type QVariantList;
        type DDciIcon;
        type QPoint;
        type QRect;
        type QSize;
        type QPainter;
        type QModelIndex;
        type QSocketNotifier;
        type QStyledItemDelegate;

        type DApplication;
        type DMainWindow;
        type DTitlebar;
        type DLabel;
        type DSuggestButton;
        type DPushButton;
        type QMessageBox;
        type QMenu;
        type QAction;

        // DApplication
        // Raw argv bytes + lengths (incl. argv[0]); Qt stores writable char* entries.
        unsafe fn application_new(
            name: &str,
            argv: Vec<u8>,
            lengths: Vec<usize>,
        ) -> *mut DApplication;
        unsafe fn application_new_ex(
            name: &str,
            argv: Vec<u8>,
            lengths: Vec<usize>,
            quit_guard_id: usize,
        ) -> *mut DApplication;
        unsafe fn application_is_gui() -> bool;
        unsafe fn application_exec(app: *mut DApplication) -> i32;
        unsafe fn application_quit();
        unsafe fn application_set_quit_on_last_window_closed(quit: bool);
        unsafe fn application_set_application_display_name(name: &str);
        unsafe fn application_load_translator(app: *mut DApplication) -> bool;
        unsafe fn application_has_arg(arg: &str) -> bool;

        // QWidget common
        unsafe fn widget_show(w: *mut QWidget);
        unsafe fn widget_set_mouse_tracking(w: *mut QWidget, on: bool);
        unsafe fn widget_hide(w: *mut QWidget);
        unsafe fn widget_resize(w: *mut QWidget, w_px: i32, h_px: i32);
        unsafe fn widget_width(w: *mut QWidget) -> i32;
        unsafe fn widget_height(w: *mut QWidget) -> i32;
        unsafe fn widget_set_enabled(w: *mut QWidget, on: bool);
        unsafe fn widget_set_window_title(w: *mut QWidget, title: &str);
        unsafe fn widget_set_window_icon(w: *mut QWidget, icon: *mut QIcon);
        unsafe fn widget_set_fixed_size(w: *mut QWidget, w_px: i32, h_px: i32);
        unsafe fn widget_raise(w: *mut QWidget);
        unsafe fn widget_update(w: *mut QWidget);
        unsafe fn widget_set_focus(w: *mut QWidget);
        unsafe fn widget_move(w: *mut QWidget, x: i32, y: i32);
        unsafe fn widget_set_parent(child: *mut QWidget, parent: *mut QWidget);
        unsafe fn tabbar_install_style(tb: *mut QWidget);
        unsafe fn tabbar_unlatch_scroll_buttons(tb: *mut QWidget);
        unsafe fn tabbar_flush_layout(tb: *mut QWidget);
        unsafe fn scrollbar_new(parent: *mut QWidget) -> *mut QWidget;
        unsafe fn scrollbar_set_range(sb: *mut QWidget, minimum: i32, maximum: i32);
        unsafe fn scrollbar_maximum(sb: *mut QWidget) -> i32;
        unsafe fn scrollbar_set_value(sb: *mut QWidget, v: i32);
        unsafe fn scrollbar_value(sb: *mut QWidget) -> i32;
        unsafe fn scrollbar_set_page_step(sb: *mut QWidget, v: i32);
        unsafe fn paint_widget_set_ime_rect(
            w: *mut QWidget,
            x: i32,
            y: i32,
            width: i32,
            height: i32,
        );
        unsafe fn main_window_titlebar_set_tabbar(w: *mut QWidget, tabbar: *mut QWidget);
        unsafe fn main_window_titlebar_add_widget(w: *mut QWidget, child: *mut QWidget);
        unsafe fn widget_set_titlebar_icon(w: *mut QWidget, icon: *mut QIcon);
        unsafe fn app_popup_active() -> bool;
        unsafe fn app_palette_window_rgb() -> u32;
        unsafe fn app_palette_highlight_rgb() -> u32;
        unsafe fn widget_activate_window(w: *mut QWidget);
        unsafe fn widget_close(w: *mut QWidget);
        unsafe fn widget_is_visible(w: *mut QWidget) -> bool;
        unsafe fn widget_set_focus_policy(w: *mut QWidget, policy: i32);

        // QProgressBar common
        unsafe fn progressbar_set_value(w: *mut QWidget, value: i32);
        /// QLineEdit base-class getter (DLineEdit etc.); cast is the caller's responsibility
        unsafe fn line_edit_text(w: *mut QWidget) -> String;
        unsafe fn progressbar_set_range(w: *mut QWidget, minimum: i32, maximum: i32);
        unsafe fn progressbar_value(w: *mut QWidget) -> i32;
        unsafe fn widget_set_font(w: *mut QWidget, font: *mut QFont);
        /// Qt::CursorShape (qt::cursor::*)
        unsafe fn widget_set_cursor(w: *mut QWidget, shape: i32);
        unsafe fn widget_unset_cursor(w: *mut QWidget);
        unsafe fn widget_palette(w: *mut QWidget) -> *mut QPalette;
        unsafe fn widget_set_palette(w: *mut QWidget, pal: *mut QPalette);
        unsafe fn object_delete_later(o: *mut QObject);
        unsafe fn object_has_parent(o: *mut QObject) -> bool;

        // DMainWindow
        unsafe fn mainwindow_new() -> *mut DMainWindow;
        unsafe fn mainwindow_new_ex(show_cb_id: usize, close_cb_id: usize) -> *mut DMainWindow;
        unsafe fn mainwindow_titlebar(w: *mut DMainWindow) -> *mut DTitlebar;
        unsafe fn mainwindow_set_central_widget(w: *mut DMainWindow, central: *mut QWidget);
        unsafe fn mainwindow_take_central_widget(w: *mut DMainWindow) -> *mut QWidget;
        unsafe fn mainwindow_set_window_radius(w: *mut DMainWindow, radius: i32);
        unsafe fn mainwindow_set_enable_blur(w: *mut DMainWindow, enable: bool);

        // DTitlebar
        unsafe fn titlebar_set_title(tb: *mut DTitlebar, title: &str);
        unsafe fn titlebar_set_icon(tb: *mut DTitlebar, icon: &QIcon);

        // QIcon
        unsafe fn icon_from_theme(name: &str) -> *mut QIcon;
        unsafe fn icon_from_theme_fallback(name: &str, fallback: *mut QIcon) -> *mut QIcon;
        unsafe fn icon_from_file(path: &str) -> *mut QIcon;

        // DLabel
        unsafe fn label_new(text: &str) -> *mut DLabel;
        unsafe fn label_set_text(l: *mut DLabel, text: &str);
        unsafe fn label_set_word_wrap(l: *mut DLabel, wrap: bool);
        unsafe fn label_set_alignment(l: *mut DLabel, alignment: i32);
        unsafe fn label_set_pixmap(l: *mut DLabel, pm: *mut QPixmap);

        // buttons
        unsafe fn suggest_button_new(text: &str) -> *mut DSuggestButton;
        unsafe fn push_button_new(text: &str) -> *mut DPushButton;
        unsafe fn button_set_text(b: *mut DPushButton, text: &str);
        unsafe fn button_click(b: *mut DPushButton);

        // QMessageBox (DMessageBox typedef)
        unsafe fn qmessagebox_new() -> *mut QMessageBox;
        unsafe fn qmessagebox_new_with(
            icon: i32,
            title: &str,
            text: &str,
            buttons: i32,
            parent: *mut QWidget,
        ) -> *mut QMessageBox;
        unsafe fn qmessagebox_set_text(mb: *mut QMessageBox, text: &str);
        unsafe fn qmessagebox_set_icon(mb: *mut QMessageBox, icon: i32);
        unsafe fn qmessagebox_set_standard_buttons(mb: *mut QMessageBox, buttons: i32);
        unsafe fn qmessagebox_set_informative_text(mb: *mut QMessageBox, text: &str);
        unsafe fn qmessagebox_set_detailed_text(mb: *mut QMessageBox, text: &str);
        unsafe fn qmessagebox_add_button_text(
            mb: *mut QMessageBox,
            text: &str,
            role: i32,
        ) -> *mut DPushButton;
        unsafe fn qmessagebox_add_button_standard(
            mb: *mut QMessageBox,
            button: i32,
        ) -> *mut DPushButton;
        unsafe fn qmessagebox_set_default_button(mb: *mut QMessageBox, button: i32);
        unsafe fn qmessagebox_exec(mb: *mut QMessageBox) -> i32;
        unsafe fn qmessagebox_clicked_button(mb: *mut QMessageBox) -> i32;
        unsafe fn qmessagebox_text(mb: *mut QMessageBox) -> String;
        // static helpers
        unsafe fn qmessagebox_information(
            parent: *mut QWidget,
            title: &str,
            text: &str,
            buttons: i32,
            default_button: i32,
        ) -> i32;
        unsafe fn qmessagebox_warning(
            parent: *mut QWidget,
            title: &str,
            text: &str,
            buttons: i32,
            default_button: i32,
        ) -> i32;
        unsafe fn qmessagebox_critical(
            parent: *mut QWidget,
            title: &str,
            text: &str,
            buttons: i32,
            default_button: i32,
        ) -> i32;
        unsafe fn qmessagebox_question(
            parent: *mut QWidget,
            title: &str,
            text: &str,
            buttons: i32,
            default_button: i32,
        ) -> i32;
        unsafe fn qmessagebox_about(parent: *mut QWidget, title: &str, text: &str);

        // layouts
        unsafe fn vbox_new(parent: *mut QWidget) -> *mut QVBoxLayout;
        unsafe fn hbox_new(parent: *mut QWidget) -> *mut QHBoxLayout;
        unsafe fn widget_new(parent: *mut QWidget) -> *mut QWidget;
        unsafe fn layout_add_widget(l: *mut QLayout, w: *mut QWidget);
        unsafe fn layout_add_widget_ex(
            l: *mut QLayout,
            w: *mut QWidget,
            stretch: i32,
            alignment: i32,
        );
        unsafe fn layout_add_stretch(l: *mut QLayout, stretch: i32);
        unsafe fn layout_add_layout(l: *mut QLayout, child: *mut QLayout);
        unsafe fn layout_set_spacing(l: *mut QLayout, spacing: i32);
        unsafe fn layout_set_contents_margins(l: *mut QLayout, l_: i32, t: i32, r: i32, b: i32);

        // QTableWidget
        unsafe fn table_new(rows: i32, cols: i32) -> *mut QTableWidget;
        unsafe fn table_set_column_count(t: *mut QTableWidget, cols: i32);
        unsafe fn table_set_row_count(t: *mut QTableWidget, rows: i32);
        unsafe fn table_set_horizontal_header_labels(t: *mut QTableWidget, labels: Vec<String>);
        unsafe fn table_horizontal_header_label(t: *mut QTableWidget, col: i32) -> String;
        unsafe fn table_set_cell_text(t: *mut QTableWidget, row: i32, col: i32, text: &str)
        -> bool;
        unsafe fn table_set_cell_data(t: *mut QTableWidget, row: i32, col: i32, data: i64) -> bool;
        unsafe fn table_cell_data(t: *mut QTableWidget, row: i32, col: i32) -> i64;
        unsafe fn table_cell_text(t: *mut QTableWidget, row: i32, col: i32) -> String;
        unsafe fn table_current_row(t: *mut QTableWidget) -> i32;
        unsafe fn table_row_count(t: *mut QTableWidget) -> i32;
        unsafe fn table_select_rows_readonly(t: *mut QTableWidget);
        unsafe fn table_header_stretch_last(t: *mut QTableWidget, stretch: bool);
        unsafe fn table_set_column_width(t: *mut QTableWidget, col: i32, width: i32);

        // QTableWidget extras
        unsafe fn table_item(t: *mut QTableWidget, row: i32, col: i32) -> *mut QTableWidgetItem;
        unsafe fn table_select_row(t: *mut QTableWidget, row: i32);
        unsafe fn table_hide_headers(t: *mut QTableWidget, horizontal: bool, vertical: bool);
        unsafe fn table_set_section_resize_mode(t: *mut QTableWidget, col: i32, mode: i32);
        unsafe fn table_set_vertical_header_default_section_size(t: *mut QTableWidget, size: i32);
        unsafe fn table_set_show_grid(t: *mut QTableWidget, show: bool);
        unsafe fn table_set_frame_shape(t: *mut QTableWidget, shape: i32);
        unsafe fn table_set_icon_size(t: *mut QTableWidget, w: i32, h: i32);
        unsafe fn table_set_delegate_for_column(
            t: *mut QTableWidget,
            col: i32,
            delegate: *mut QStyledItemDelegate,
        );

        // QAbstractItemModel (QModelIndex values never outlive a C++ call)
        unsafe fn standard_item_model_new(parent: *mut QObject) -> *mut QStandardItemModel;
        unsafe fn standard_item_model_set_column_count(model: *mut QStandardItemModel, count: i32);
        unsafe fn standard_item_model_as_model(
            model: *mut QStandardItemModel,
        ) -> *mut QAbstractItemModel;
        unsafe fn model_set_parent(model: *mut QAbstractItemModel, parent: *mut QObject);
        unsafe fn model_row_count(model: *mut QAbstractItemModel) -> i32;
        unsafe fn model_column_count(model: *mut QAbstractItemModel) -> i32;
        unsafe fn model_data(
            model: *mut QAbstractItemModel,
            row: i32,
            col: i32,
            role: i32,
        ) -> *mut QVariant;
        unsafe fn model_set_data(
            model: *mut QAbstractItemModel,
            row: i32,
            col: i32,
            data: *mut QVariant,
            role: i32,
        ) -> bool;
        unsafe fn model_insert_rows(model: *mut QAbstractItemModel, row: i32, count: i32) -> bool;
        unsafe fn model_remove_rows(model: *mut QAbstractItemModel, row: i32, count: i32) -> bool;

        // QTableWidgetItem
        unsafe fn item_set_icon(it: *mut QTableWidgetItem, icon: *mut QIcon);
        unsafe fn item_set_text_alignment(it: *mut QTableWidgetItem, alignment: i32);
        unsafe fn item_set_foreground(it: *mut QTableWidgetItem, color: *mut QColor);
        unsafe fn item_set_data_string(it: *mut QTableWidgetItem, role: i32, value: &str);
        unsafe fn item_data_string(it: *mut QTableWidgetItem, role: i32) -> String;
        unsafe fn item_set_data_bool(it: *mut QTableWidgetItem, role: i32, value: bool);
        unsafe fn item_data_bool(it: *mut QTableWidgetItem, role: i32) -> bool;

        // value types
        unsafe fn variant_new() -> *mut QVariant;
        unsafe fn variant_from_bool(value: bool) -> *mut QVariant;
        unsafe fn variant_from_i64(value: i64) -> *mut QVariant;
        unsafe fn variant_from_f64(value: f64) -> *mut QVariant;
        unsafe fn variant_from_string(value: &str) -> *mut QVariant;
        unsafe fn variant_clone(value: *mut QVariant) -> *mut QVariant;
        unsafe fn variant_is_valid(value: *mut QVariant) -> bool;
        unsafe fn variant_to_bool(value: *mut QVariant) -> bool;
        unsafe fn variant_to_i64(value: *mut QVariant, ok: *mut bool) -> i64;
        unsafe fn variant_to_string(value: *mut QVariant) -> String;
        unsafe fn variant_delete(value: *mut QVariant);
        unsafe fn variant_list_new() -> *mut QVariantList;
        unsafe fn variant_list_clone(list: *mut QVariantList) -> *mut QVariantList;
        unsafe fn variant_list_push(list: *mut QVariantList, value: *mut QVariant);
        unsafe fn variant_list_len(list: *mut QVariantList) -> usize;
        unsafe fn variant_list_get(list: *mut QVariantList, index: usize) -> *mut QVariant;
        unsafe fn variant_list_delete(list: *mut QVariantList);
        unsafe fn color_new_rgb(r: i32, g: i32, b: i32, a: i32) -> *mut QColor;
        unsafe fn color_rgba(c: *mut QColor) -> i32;
        unsafe fn color_delete(c: *mut QColor);
        unsafe fn font_new() -> *mut QFont;
        unsafe fn font_set_point_size(f: *mut QFont, size: i32);
        unsafe fn font_set_bold(f: *mut QFont, bold: bool);
        unsafe fn font_set_italic(f: *mut QFont, italic: bool);
        unsafe fn fontmetrics_height(f: *mut QFont) -> i32;
        unsafe fn fontmetrics_ascent(f: *mut QFont) -> i32;
        unsafe fn fontmetrics_max_width(f: *mut QFont) -> i32;
        unsafe fn fontmetrics_advance(f: *mut QFont, text: &str) -> i32;
        unsafe fn font_force_integer_metrics(f: *mut QFont);
        unsafe fn font_set_monospace(f: *mut QFont);
        unsafe fn font_set_family(f: *mut QFont, name: &str);
        unsafe fn font_delete(f: *mut QFont);
        unsafe fn palette_new() -> *mut QPalette;
        unsafe fn palette_set_color(pal: *mut QPalette, group: i32, role: i32, color: *mut QColor);
        unsafe fn palette_color(pal: *mut QPalette, group: i32, role: i32) -> *mut QColor;
        unsafe fn palette_delete(pal: *mut QPalette);
        unsafe fn dpalette_new() -> *mut DPalette;
        unsafe fn dpalette_text_title(pal: *mut DPalette) -> *mut QBrush;
        unsafe fn dpalette_delete(pal: *mut DPalette);
        unsafe fn pixmap_new(path: &str) -> *mut QPixmap;
        unsafe fn pixmap_delete(pm: *mut QPixmap);
        unsafe fn image_new(width: i32, height: i32) -> *mut QImage;
        unsafe fn image_from_file(path: &str) -> *mut QImage;
        unsafe fn image_is_null(image: *mut QImage) -> bool;
        unsafe fn image_width(image: *mut QImage) -> i32;
        unsafe fn image_height(image: *mut QImage) -> i32;
        unsafe fn image_fill(image: *mut QImage, color: *mut QColor);
        unsafe fn image_set_text(image: *mut QImage, key: &str, value: &str);
        unsafe fn image_text(image: *mut QImage, key: &str) -> String;
        unsafe fn image_save(image: *mut QImage, path: &str) -> bool;
        unsafe fn image_pixel_color(image: *mut QImage, x: i32, y: i32) -> *mut QColor;
        unsafe fn image_delete(image: *mut QImage);
        unsafe fn painter_path_new() -> *mut QPainterPath;
        unsafe fn painter_path_add_rect(path: *mut QPainterPath, x: f64, y: f64, w: f64, h: f64);
        unsafe fn painter_path_is_empty(path: *mut QPainterPath) -> bool;
        unsafe fn painter_path_element_count(path: *mut QPainterPath) -> i32;
        unsafe fn painter_path_delete(path: *mut QPainterPath);
        unsafe fn brush_from_color(color: *mut QColor) -> *mut QBrush;
        unsafe fn brush_color(brush: *mut QBrush) -> *mut QColor;
        unsafe fn brush_delete(brush: *mut QBrush);
        unsafe fn key_sequence_from_string(text: &str) -> *mut QKeySequence;
        unsafe fn key_sequence_to_string(seq: *mut QKeySequence) -> String;
        unsafe fn key_sequence_delete(seq: *mut QKeySequence);
        unsafe fn url_from_string(url: &str) -> *mut QUrl;
        unsafe fn url_to_string(url: *mut QUrl) -> String;
        unsafe fn url_is_valid(url: *mut QUrl) -> bool;
        unsafe fn url_delete(url: *mut QUrl);
        // QMargins
        unsafe fn q_margins_new(left: i32, top: i32, right: i32, bottom: i32) -> *mut QMargins;
        unsafe fn margins_delete(m: *mut QMargins);
        // DDciIcon (dtkgui)
        unsafe fn ddci_icon_new() -> *mut DDciIcon;
        unsafe fn ddci_icon_from_file(path: &str) -> *mut DDciIcon;
        unsafe fn ddci_icon_delete(i: *mut DDciIcon);
        unsafe fn standard_icon_pixmap(w: *mut QWidget, icon: i32, size: i32) -> *mut QPixmap;
        unsafe fn size_new(w: i32, h: i32) -> *mut QSize;
        unsafe fn size_delete(s: *mut QSize);
        unsafe fn point_new(x: i32, y: i32) -> *mut QPoint;
        unsafe fn point_delete(p: *mut QPoint);
        unsafe fn rect_new(x: i32, y: i32, w: i32, h: i32) -> *mut QRect;
        unsafe fn rect_delete(r: *mut QRect);
        unsafe fn rect_width(r: *mut QRect) -> i32;
        unsafe fn rect_height(r: *mut QRect) -> i32;
        unsafe fn rect_x(r: *mut QRect) -> i32;
        unsafe fn rect_y(r: *mut QRect) -> i32;
        unsafe fn icon_delete(icon: *mut QIcon);

        // QSocketNotifier
        unsafe fn socket_notifier_new(fd: i32, parent: *mut QObject) -> *mut QSocketNotifier;
        unsafe fn socket_notifier_set_enabled(n: *mut QSocketNotifier, on: bool);

        // generic paint delegate
        unsafe fn rust_delegate_new(
            paint_cb_id: usize,
            parent: *mut QObject,
        ) -> *mut QStyledItemDelegate;

        // QPainter primitives
        unsafe fn painter_draw_text_at(p: *mut QPainter, x: i32, y: i32, text: &str);
        unsafe fn painter_draw_line(p: *mut QPainter, x1: i32, y1: i32, x2: i32, y2: i32);
        unsafe fn painter_save(p: *mut QPainter);
        unsafe fn painter_restore(p: *mut QPainter);
        unsafe fn painter_set_pen_color(p: *mut QPainter, color: *mut QColor);
        unsafe fn painter_set_font(p: *mut QPainter, font: *mut QFont);
        unsafe fn painter_draw_text(
            p: *mut QPainter,
            x: i32,
            y: i32,
            w: i32,
            h: i32,
            flags: i32,
            text: &str,
        );
        unsafe fn painter_draw_pixmap(
            p: *mut QPainter,
            x: i32,
            y: i32,
            w: i32,
            h: i32,
            pm: *mut QPixmap,
        );
        unsafe fn painter_draw_icon(
            p: *mut QPainter,
            x: i32,
            y: i32,
            w: i32,
            h: i32,
            icon: *mut QIcon,
        );
        unsafe fn painter_fill_rect(
            p: *mut QPainter,
            x: i32,
            y: i32,
            w: i32,
            h: i32,
            color: *mut QColor,
        );
        unsafe fn painter_set_clip_rect(p: *mut QPainter, x: i32, y: i32, w: i32, h: i32);
        unsafe fn painter_translate(p: *mut QPainter, x: f64, y: f64);
        unsafe fn painter_scale(p: *mut QPainter, sx: f64, sy: f64);
        unsafe fn painter_elided_text(
            p: *mut QPainter,
            text: &str,
            mode: i32,
            width: i32,
        ) -> String;

        // QModelIndex data access
        unsafe fn index_data_string(idx: *mut QModelIndex, role: i32) -> String;
        unsafe fn index_data_bool(idx: *mut QModelIndex, role: i32) -> bool;
        unsafe fn index_data_i64(idx: *mut QModelIndex, role: i32) -> i64;

        // QTimer
        unsafe fn timer_new(parent: *mut QObject) -> *mut QTimer;
        unsafe fn timer_start(t: *mut QTimer, msec: i32);
        unsafe fn timer_stop(t: *mut QTimer);
        unsafe fn timer_single_shot(msec: i32, cb_id: usize);

        // QAction / QMenu (DMenu typedef in DTK6)
        unsafe fn action_new(text: &str, parent: *mut QObject) -> *mut QAction;
        unsafe fn action_set_enabled(action: *mut QAction, enabled: bool);
        unsafe fn action_is_enabled(action: *mut QAction) -> bool;
        unsafe fn action_set_visible(action: *mut QAction, visible: bool);
        unsafe fn action_is_visible(action: *mut QAction) -> bool;
        unsafe fn action_set_text(action: *mut QAction, text: &str);
        unsafe fn action_text(action: *mut QAction) -> String;
        unsafe fn action_trigger(action: *mut QAction);
        unsafe fn menu_new(parent: *mut QWidget) -> *mut QMenu;
        unsafe fn menu_add_existing_action(menu: *mut QMenu, action: *mut QAction);
        unsafe fn menu_add_action_cb(m: *mut QMenu, text: &str, cb_id: usize);
        unsafe fn menu_add_separator(m: *mut QMenu);
        /// popup at (x, y) in ref widget coords; menu hides on close
        unsafe fn menu_popup(m: *mut QMenu, ref_: *mut QWidget, x: i32, y: i32);

        // signals; connect fns return false on failure (caller must roll back registration)
        unsafe fn relay_connect0(sender: *mut QObject, signal: &str, cb_id: usize) -> bool;
        unsafe fn relay_connect_i32(sender: *mut QObject, signal: &str, cb_id: usize) -> bool;
        unsafe fn relay_connect_bool(sender: *mut QObject, signal: &str, cb_id: usize) -> bool;
        unsafe fn relay_connect_i32_i32(sender: *mut QObject, signal: &str, cb_id: usize) -> bool;
        /// disconnect + schedule deletion of the relay for cb_id (no-op if unknown)
        unsafe fn relay_disconnect(cb_id: usize);

        // user-drawn widget + clipboard + shortcuts
        unsafe fn paint_widget_new(cb_id: usize, parent: *mut QWidget) -> *mut QWidget;
        unsafe fn paint_widget_inject_key(w: *mut QWidget, key: i32, mods: i32, text: &str);
        unsafe fn clipboard_set_text(text: &str, mode: i32);
        unsafe fn clipboard_text(mode: i32) -> String;
        unsafe fn shortcut_new(parent: *mut QWidget, key: &str, cb_id: usize);
    }

    extern "Rust" {
        fn dtk_cb_dispose(id: usize);
        fn dtk_cb0(id: usize);
        fn dtk_cb_i32(id: usize, v: i32);
        fn dtk_cb_bool(id: usize, v: bool);
        fn dtk_cb_i32_i32(id: usize, a: i32, b: i32);
        fn dtk_cb_guard(id: usize) -> bool;
        // DtkPaintWidget event callbacks
        unsafe fn dtk_cb_pw_paint(
            id: usize,
            widget: *mut QWidget,
            painter: *mut QPainter,
            w: i32,
            h: i32,
        );
        fn dtk_cb_pw_key(
            id: usize,
            key: i32,
            mods: i32,
            text: String,
            press: bool,
            autorepeat: bool,
        );
        fn dtk_cb_pw_mouse(id: usize, kind: i32, button: i32, x: i32, y: i32, mods: i32);
        fn dtk_cb_pw_wheel(id: usize, dy: i32, x: i32, y: i32, mods: i32);
        fn dtk_cb_pw_ime(id: usize, commit: String, preedit: String);
        fn dtk_cb_pw_resize(id: usize, w: i32, h: i32);
        fn dtk_cb_pw_focus(id: usize, focus_in: bool);
        unsafe fn dtk_cb_paint(
            id: usize,
            painter: *mut QPainter,
            index: *mut QModelIndex,
            x: i32,
            y: i32,
            w: i32,
            h: i32,
            state: i32,
        );
    }
}

/// Owns a Qt QPointer, not the QObject it watches. Must remain on its Qt thread.
/// `is_alive()` is safe even after Qt destroys the watched object.
pub struct ObjectGuard {
    inner: cxx::UniquePtr<ffi::QObjectGuard>,
    _not_send: PhantomData<*mut ()>,
}

impl ObjectGuard {
    /// # Safety
    /// `object` must be a live QObject on the current GUI thread. Its
    /// `QPointer` is invalidated by Qt before emitting `destroyed(QObject*)`.
    pub unsafe fn from_object(object: *mut ffi::QObject) -> Self {
        assert!(!object.is_null());
        let inner = unsafe { ffi::guard_object(object) };
        assert!(!inner.is_null());
        Self {
            inner,
            _not_send: PhantomData,
        }
    }

    /// # Safety
    /// `widget` must be a live QWidget on the current GUI thread. Qt will
    /// invalidate the guard when it destroys the object (including via a parent).
    pub unsafe fn from_widget(widget: *mut ffi::QWidget) -> Self {
        assert!(!widget.is_null());
        let inner = unsafe { ffi::guard_widget(widget) };
        assert!(!inner.is_null());
        Self {
            inner,
            _not_send: PhantomData,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.inner
            .as_ref()
            .is_some_and(|guard| unsafe { ffi::guard_alive(guard) })
    }

    pub fn was_parented(&self) -> bool {
        self.inner
            .as_ref()
            .is_some_and(|guard| unsafe { ffi::guard_was_parented(guard) })
    }
}

// ---- callback registry: Qt signals -> Rust closures ----
// callbacks only fire on the Qt main thread; thread_local is enough, no locks
/// events delivered to a PaintWidget callback (QPainter valid only during Paint)
pub enum PwEvent {
    Paint {
        widget: *mut ffi::QWidget,
        painter: *mut ffi::QPainter,
        w: i32,
        h: i32,
    },
    Key {
        key: i32,
        mods: i32,
        text: String,
        press: bool,
        autorepeat: bool,
    },
    Mouse {
        kind: i32,
        button: i32,
        x: i32,
        y: i32,
        mods: i32,
    },
    Wheel {
        dy: i32,
        x: i32,
        y: i32,
        mods: i32,
    },
    Ime {
        commit: String,
        preedit: String,
    },
    Resize {
        w: i32,
        h: i32,
    },
    Focus(bool),
}

/// Borrowed C++ paint data, valid only during the callback invocation.
pub struct PaintData {
    pub painter: *mut ffi::QPainter,
    pub index: *mut ffi::QModelIndex,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub state: i32,
}

enum Cb {
    C0(Box<dyn FnMut()>),
    Once0(Box<dyn FnOnce()>),
    I32(Box<dyn FnMut(i32)>),
    I32I32(Box<dyn FnMut(i32, i32)>),
    Bool(Box<dyn FnMut(bool)>),
    Guard(Box<dyn FnMut() -> bool>),
    Paint(Box<dyn FnMut(PaintData)>),
    Pw(Box<dyn FnMut(PwEvent)>),
}

thread_local! {
    static CALLBACKS: RefCell<HashMap<usize, Cb>> = RefCell::new(HashMap::new());
    // Checked-out callbacks are absent from CALLBACKS. Track only live dispatches,
    // so unregistering an unknown/already-fired id cannot grow an unbounded tombstone set.
    static IN_FLIGHT: RefCell<HashMap<usize, bool>> = RefCell::new(HashMap::new());
    // Only these ids have a C++ DtkRelay to disconnect. Thread-local so a
    // foreign-thread unregister never touches Qt's GUI-thread-only QHash.
    static RELAY_IDS: RefCell<HashSet<usize>> = RefCell::new(HashSet::new());
}

/// Record a successfully connected Qt relay on its GUI thread.
///
/// # Safety
/// The relay must have been created for `id` on the current Qt object's
/// thread by `ffi::relay_connect*`. Do not call from a worker thread.
pub unsafe fn register_qt_relay(id: usize) {
    if CALLBACKS.with(|c| c.borrow().contains_key(&id))
        || IN_FLIGHT.with(|active| active.borrow().contains_key(&id))
    {
        RELAY_IDS.with(|relays| {
            relays.borrow_mut().insert(id);
        });
    }
}

fn next_id() -> usize {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

pub fn register_cb0(f: impl FnMut() + 'static) -> usize {
    let id = next_id();
    CALLBACKS.with(|c| c.borrow_mut().insert(id, Cb::C0(Box::new(f))));
    id
}

/// A callback that is removed even if it panics when invoked from Qt.
pub fn register_cb_once(f: impl FnOnce() + 'static) -> usize {
    let id = next_id();
    CALLBACKS.with(|c| c.borrow_mut().insert(id, Cb::Once0(Box::new(f))));
    id
}

pub fn register_cb_i32(f: impl FnMut(i32) + 'static) -> usize {
    let id = next_id();
    CALLBACKS.with(|c| c.borrow_mut().insert(id, Cb::I32(Box::new(f))));
    id
}

pub fn register_cb_bool(f: impl FnMut(bool) + 'static) -> usize {
    let id = next_id();
    CALLBACKS.with(|c| c.borrow_mut().insert(id, Cb::Bool(Box::new(f))));
    id
}

pub fn register_cb_i32_i32(f: impl FnMut(i32, i32) + 'static) -> usize {
    let id = next_id();
    CALLBACKS.with(|c| c.borrow_mut().insert(id, Cb::I32I32(Box::new(f))));
    id
}

pub fn register_cb_pw(f: impl FnMut(PwEvent) + 'static) -> usize {
    let id = next_id();
    CALLBACKS.with(|c| c.borrow_mut().insert(id, Cb::Pw(Box::new(f))));
    id
}

pub fn register_cb_guard(f: impl FnMut() -> bool + 'static) -> usize {
    let id = next_id();
    CALLBACKS.with(|c| c.borrow_mut().insert(id, Cb::Guard(Box::new(f))));
    id
}

pub fn register_cb_paint(f: impl FnMut(PaintData) + 'static) -> usize {
    let id = next_id();
    CALLBACKS.with(|c| c.borrow_mut().insert(id, Cb::Paint(Box::new(f))));
    id
}

fn drop_callback(id: usize, cb: Cb) {
    // A user capture may panic in Drop as well as in the callback itself.
    if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(cb))).is_err() {
        eprintln!("dtk-rs: callback {id} panicked during Drop, swallowed at FFI boundary");
    }
}

fn remove_callback(id: usize) -> bool {
    if let Some(cb) = CALLBACKS.with(|c| c.borrow_mut().remove(&id)) {
        drop_callback(id, cb);
        true
    } else {
        IN_FLIGHT.with(|active| {
            if let Some(cancelled) = active.borrow_mut().get_mut(&id) {
                *cancelled = true;
                true
            } else {
                false
            }
        })
    }
}

/// Remove a callback, including one currently executing. Does not call back into Qt:
/// Qt invokes this while destroying the owning QObject or relay.
fn dtk_cb_dispose(id: usize) {
    RELAY_IDS.with(|relays| {
        relays.borrow_mut().remove(&id);
    });
    remove_callback(id);
}

/// Remove a callback and disconnect its Qt-side relay. Returns false for an
/// unknown, disposed or foreign-thread id. A worker call with a GUI-owned id
/// does not touch Qt; disconnect that callback on its owning GUI thread.
/// Safe to call from inside the callback itself.
pub fn unregister_cb(id: usize) -> bool {
    let removed = remove_callback(id);
    if RELAY_IDS.with(|relays| relays.borrow_mut().remove(&id)) {
        // This id was registered through Qt on *this* thread; only now may
        // we touch DtkRelay's GUI-thread-only global QHash.
        unsafe { ffi::relay_disconnect(id) };
    }
    removed
}

fn begin_dispatch(id: usize) -> Option<Cb> {
    let cb = CALLBACKS.with(|c| c.borrow_mut().remove(&id));
    if cb.is_some() {
        IN_FLIGHT.with(|active| active.borrow_mut().insert(id, false));
    }
    cb
}

/// Reinsert after dispatch unless Qt destroyed its owner or the user unregistered it.
fn finish_dispatch(id: usize, cb: Option<Cb>) {
    let cancelled = IN_FLIGHT.with(|active| active.borrow_mut().remove(&id));
    if let Some(cb) = cb {
        if cancelled == Some(false) {
            CALLBACKS.with(|c| c.borrow_mut().insert(id, cb));
        } else {
            drop_callback(id, cb);
        }
    }
}

// FnMut cannot be invoked again while its previous invocation is still on
// the stack. Qt can synchronously emit the same signal inside a Rust callback;
// queue its copied arguments and deliver them when the outer callback returns.
// Paint-widget events have their own queue because Paint holds stack pointers.
#[derive(Clone, Copy)]
enum SignalEvent {
    Zero,
    I32(i32),
    Bool(bool),
    I32I32(i32, i32),
}
thread_local! {
    static SIGNAL_QUEUED: RefCell<HashMap<usize, std::collections::VecDeque<SignalEvent>>> =
        RefCell::new(HashMap::new());
}

// remove-then-call so callbacks can safely (un)register callbacks; put back after.
// catch_unwind: a panicking Rust callback must never unwind across the FFI boundary (UB).
fn dispatch_signal(id: usize, event: SignalEvent) {
    let nested = SIGNAL_QUEUED.with(|pending| {
        let mut pending = pending.borrow_mut();
        if let Some(queue) = pending.get_mut(&id) {
            queue.push_back(event);
            true
        } else {
            pending.insert(id, Default::default());
            false
        }
    });
    if nested {
        return;
    }
    let mut cb = begin_dispatch(id);
    let mut next = Some(event);
    while let Some(event) = next
        .take()
        .or_else(|| SIGNAL_QUEUED.with(|pending| pending.borrow_mut().get_mut(&id)?.pop_front()))
    {
        if IN_FLIGHT.with(|active| active.borrow().get(&id).copied()) == Some(true) {
            break; // Qt destroyed the sender or user explicitly disconnected
        }
        let result = match event {
            SignalEvent::Zero => match &mut cb {
                Some(Cb::C0(f)) => Some(std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                    &mut *f,
                ))),
                Some(Cb::Once0(_)) => {
                    let Some(Cb::Once0(f)) = cb.take() else {
                        unreachable!()
                    };
                    Some(std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)))
                }
                _ => None,
            },
            SignalEvent::I32(v) => match &mut cb {
                Some(Cb::I32(f)) => Some(std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                    || f(v),
                ))),
                _ => None,
            },
            SignalEvent::Bool(v) => match &mut cb {
                Some(Cb::Bool(f)) => Some(std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                    || f(v),
                ))),
                _ => None,
            },
            SignalEvent::I32I32(a, b) => match &mut cb {
                Some(Cb::I32I32(f)) => Some(std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| f(a, b)),
                )),
                _ => None,
            },
        };
        if result.is_some_and(|r| r.is_err()) {
            eprintln!("dtk-rs: callback {id} panicked, swallowed at FFI boundary");
        }
    }
    SIGNAL_QUEUED.with(|pending| {
        pending.borrow_mut().remove(&id);
    });
    finish_dispatch(id, cb);
}

fn dtk_cb0(id: usize) {
    dispatch_signal(id, SignalEvent::Zero);
}
fn dtk_cb_i32(id: usize, v: i32) {
    dispatch_signal(id, SignalEvent::I32(v));
}

thread_local! {
    // per-id queue while a paint-widget dispatch is in flight: the callback
    // slot is checked out for the whole dispatch, so a reentrant event (a
    // split keybinding resizing its own pane) would find nothing and be
    // silently dropped — the lost Resize left the PTY winsize stale. Queue
    // reentrant events instead; the outermost dispatch drains them in order.
    static PW_QUEUED: RefCell<HashMap<usize, std::collections::VecDeque<PwEvent>>> =
        RefCell::new(HashMap::new());
}

fn dispatch_pw(id: usize, ev: PwEvent) {
    if PW_QUEUED.with(|q| q.borrow().contains_key(&id)) {
        match ev {
            // A QPainter is stack-scoped to this nested C++ paintEvent. Never
            // queue its pointer past the return of that event: schedule a new
            // paint for a later event-loop turn instead.
            PwEvent::Paint { widget, .. } => unsafe { ffi::widget_update(widget) },
            ev => PW_QUEUED.with(|q| {
                if let Some(queue) = q.borrow_mut().get_mut(&id) {
                    queue.push_back(ev);
                }
            }),
        }
        return;
    }
    PW_QUEUED.with(|q| q.borrow_mut().insert(id, Default::default()));
    let mut cb = begin_dispatch(id);
    let mut next = Some(ev);
    while let Some(ev) = next
        .take()
        .or_else(|| PW_QUEUED.with(|q| q.borrow_mut().get_mut(&id).and_then(|q| q.pop_front())))
    {
        if let Some(Cb::Pw(f)) = &mut cb
            && std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(ev))).is_err()
        {
            eprintln!("dtk-rs: paint-widget callback {id} panicked, swallowed at FFI boundary");
        }
    }
    PW_QUEUED.with(|q| q.borrow_mut().remove(&id));
    finish_dispatch(id, cb);
}

unsafe fn dtk_cb_pw_paint(
    id: usize,
    widget: *mut ffi::QWidget,
    painter: *mut ffi::QPainter,
    w: i32,
    h: i32,
) {
    dispatch_pw(
        id,
        PwEvent::Paint {
            widget,
            painter,
            w,
            h,
        },
    );
}

fn dtk_cb_pw_key(id: usize, key: i32, mods: i32, text: String, press: bool, autorepeat: bool) {
    dispatch_pw(
        id,
        PwEvent::Key {
            key,
            mods,
            text,
            press,
            autorepeat,
        },
    );
}

fn dtk_cb_pw_mouse(id: usize, kind: i32, button: i32, x: i32, y: i32, mods: i32) {
    dispatch_pw(
        id,
        PwEvent::Mouse {
            kind,
            button,
            x,
            y,
            mods,
        },
    );
}

fn dtk_cb_pw_wheel(id: usize, dy: i32, x: i32, y: i32, mods: i32) {
    dispatch_pw(id, PwEvent::Wheel { dy, x, y, mods });
}

fn dtk_cb_pw_ime(id: usize, commit: String, preedit: String) {
    dispatch_pw(id, PwEvent::Ime { commit, preedit });
}

fn dtk_cb_pw_resize(id: usize, w: i32, h: i32) {
    dispatch_pw(id, PwEvent::Resize { w, h });
}

fn dtk_cb_pw_focus(id: usize, focus_in: bool) {
    dispatch_pw(id, PwEvent::Focus(focus_in));
}

fn dtk_cb_bool(id: usize, v: bool) {
    dispatch_signal(id, SignalEvent::Bool(v));
}
fn dtk_cb_i32_i32(id: usize, a: i32, b: i32) {
    dispatch_signal(id, SignalEvent::I32I32(a, b));
}

fn dtk_cb_guard(id: usize) -> bool {
    let mut cb = begin_dispatch(id);
    let result = if let Some(Cb::Guard(f)) = &mut cb {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(&mut *f)) {
            Ok(r) => r,
            Err(_) => {
                eprintln!("dtk-rs: guard callback {id} panicked, defaulting to allow");
                true
            }
        }
    } else {
        eprintln!("dtk-rs: guard callback {id} missing, defaulting to allow");
        true
    };
    finish_dispatch(id, cb);
    result
}

#[allow(clippy::too_many_arguments)] // fixed C++ callback ABI
unsafe fn dtk_cb_paint(
    id: usize,
    painter: *mut ffi::QPainter,
    index: *mut ffi::QModelIndex,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    state: i32,
) {
    let mut cb = begin_dispatch(id);
    if let Some(Cb::Paint(f)) = &mut cb
        && std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            f(PaintData {
                painter,
                index,
                x,
                y,
                w,
                h,
                state,
            })
        }))
        .is_err()
    {
        eprintln!("dtk-rs: paint callback {id} panicked, swallowed at FFI boundary");
    }
    finish_dispatch(id, cb);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    struct DropCount(Rc<std::cell::Cell<usize>>);

    impl Drop for DropCount {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    #[test]
    fn qt_destruction_during_dispatch_does_not_resurrect_callback() {
        let drops = Rc::new(std::cell::Cell::new(0));
        let id_slot = Rc::new(std::cell::Cell::new(0));
        let probe = DropCount(drops.clone());
        let slot = id_slot.clone();
        let id = register_cb0(move || {
            let _ = &probe;
            dtk_cb_dispose(slot.get());
        });
        id_slot.set(id);
        dtk_cb0(id);
        dtk_cb0(id);
        assert_eq!(drops.get(), 1);
        assert!(!unregister_cb(id));
        assert!(!IN_FLIGHT.with(|active| active.borrow().contains_key(&id)));
    }

    #[test]
    fn unregister_unknown_id_does_not_accumulate_tombstones() {
        let before = IN_FLIGHT.with(|active| active.borrow().len());
        for _ in 0..100 {
            assert!(!unregister_cb(0));
            assert!(!unregister_cb(usize::MAX));
        }
        assert_eq!(IN_FLIGHT.with(|active| active.borrow().len()), before);
    }

    #[test]
    fn one_shot_panics_are_caught_and_closure_is_dropped() {
        let drops = Rc::new(std::cell::Cell::new(0));
        let probe = DropCount(drops.clone());
        let id = register_cb_once(move || {
            let _ = &probe;
            panic!("intentional callback panic");
        });
        dtk_cb0(id);
        dtk_cb0(id);
        assert_eq!(drops.get(), 1);
        assert!(!unregister_cb(id));
        assert!(!IN_FLIGHT.with(|active| active.borrow().contains_key(&id)));
    }

    #[test]
    fn wrong_signature_does_not_lose_callback() {
        let calls = Rc::new(std::cell::Cell::new(0));
        let calls2 = calls.clone();
        let id = register_cb0(move || calls2.set(calls2.get() + 1));
        dtk_cb_i32(id, 17);
        dtk_cb0(id);
        assert_eq!(calls.get(), 1);
        assert!(unregister_cb(id));
    }

    #[test]
    fn reentrant_signal_events_are_delivered_in_order() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let id_slot = Rc::new(std::cell::Cell::new(0));
        let (log_cb, id_cb) = (log.clone(), id_slot.clone());
        let id = register_cb_i32(move |value| {
            log_cb.borrow_mut().push(value);
            if value == 1 {
                dtk_cb_i32(id_cb.get(), 2);
                dtk_cb_i32(id_cb.get(), 3);
            }
        });
        id_slot.set(id);
        dtk_cb_i32(id, 1);
        assert_eq!(*log.borrow(), vec![1, 2, 3]);
        assert!(unregister_cb(id));
    }

    #[test]
    fn reentrant_one_shot_stays_one_shot() {
        let calls = Rc::new(std::cell::Cell::new(0));
        let id_slot = Rc::new(std::cell::Cell::new(0));
        let (calls_cb, id_cb) = (calls.clone(), id_slot.clone());
        let id = register_cb_once(move || {
            calls_cb.set(calls_cb.get() + 1);
            dtk_cb0(id_cb.get());
        });
        id_slot.set(id);
        dtk_cb0(id);
        assert_eq!(calls.get(), 1);
        assert!(!unregister_cb(id));
    }

    #[test]
    fn reentrant_two_argument_signals_preserve_order() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let id_slot = Rc::new(std::cell::Cell::new(0));
        let (log_cb, id_cb) = (log.clone(), id_slot.clone());
        let id = register_cb_i32_i32(move |a, b| {
            log_cb.borrow_mut().push((a, b));
            if a == 1 {
                dtk_cb_i32_i32(id_cb.get(), 2, 20);
                dtk_cb_i32_i32(id_cb.get(), 3, 30);
            }
        });
        id_slot.set(id);
        dtk_cb_i32_i32(id, 1, 10);
        assert_eq!(*log.borrow(), vec![(1, 10), (2, 20), (3, 30)]);
        assert!(unregister_cb(id));
    }

    #[test]
    fn destroying_owner_during_nested_signal_discards_pending_events() {
        let calls = Rc::new(std::cell::Cell::new(0));
        let id_slot = Rc::new(std::cell::Cell::new(0));
        let (calls_cb, id_cb) = (calls.clone(), id_slot.clone());
        let id = register_cb_bool(move |value| {
            calls_cb.set(calls_cb.get() + 1);
            if value {
                dtk_cb_bool(id_cb.get(), false);
                dtk_cb_dispose(id_cb.get());
            }
        });
        id_slot.set(id);
        dtk_cb_bool(id, true);
        assert_eq!(calls.get(), 1);
        assert!(!unregister_cb(id));
    }

    #[test]
    fn reentrant_pw_events_are_queued_not_dropped() {
        // a handler that re-triggers an event on its own widget (e.g. a split
        // keybinding resizing its own pane) must not lose the nested event
        let log = Rc::new(RefCell::new(Vec::new()));
        let id_slot = Rc::new(RefCell::new(0usize));
        let (l2, id2) = (log.clone(), id_slot.clone());
        let id = register_cb_pw(move |ev| {
            if let PwEvent::Resize { w, .. } = ev {
                let reenter = l2.borrow().is_empty();
                l2.borrow_mut().push(w);
                if reenter {
                    dispatch_pw(*id2.borrow(), PwEvent::Resize { w: 2, h: 2 });
                }
            }
        });
        *id_slot.borrow_mut() = id;
        dispatch_pw(id, PwEvent::Resize { w: 1, h: 1 });
        assert_eq!(*log.borrow(), vec![1, 2]);
        unregister_cb(id);
    }
}
