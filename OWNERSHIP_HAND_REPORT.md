# Hand-written FFI pointer/ownership audit

Source authority: matching apt source Qt 6.8.0+dfsg-0deepin31, dtkwidget 6.7.47.1-1, dtkcore/dtkgui 6.7.47.
Regenerate with `uv run --no-project python tools/audit_hand.py`. New pointer-return functions fail closed until reviewed.

| Bridge API | Pointer type | Ownership | Evidence / lifetime |
|---|---|---|---|
| `application_runtime_application` | `QObject*` | scope borrow | application_runtime.cpp: runtime.application(); no public owning/Clone token |
| `application_runtime_gui_application` | `DApplication*` | scope borrow | shim.cpp: qobject_cast of private runtime application |
| `core_config_create_generic` | `DConfig*` | unique Owned<DConfig> | dtkcore/src/dconfig.cpp createGeneric: explicitly released by caller; parent nullptr |
| `core_config_create` | `DConfig*` | unique Owned<DConfig> | dtkcore/src/dconfig.cpp create: explicitly released by caller; parent nullptr |
| `scrollbar_new` | `QWidget*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `scrollbar_new`: verified new allocation; registered after guard construction |
| `widget_palette` | `QPalette*` | unique heap value/copy; matching *_delete | shim.cpp `widget_palette`: new allocation/copy; Qt storage never borrowed into Rust |
| `mainwindow_new` | `DMainWindow*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `mainwindow_new`: verified new allocation; registered after guard construction |
| `mainwindow_new_ex` | `DMainWindow*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `mainwindow_new_ex`: verified new allocation; registered after guard construction |
| `mainwindow_titlebar` | `DTitlebar*` | borrowed guarded view | DMainWindow owns titlebar; no deletion authority from getter |
| `mainwindow_take_central_widget` | `QWidget*` | unique transferred Owned<QWidget> | qt6-base/src/widgets/widgets/qmainwindow.cpp takeCentralWidget: setParent(nullptr), caller ownership |
| `icon_from_theme` | `QIcon*` | unique heap value/copy; matching *_delete | shim.cpp `icon_from_theme`: new allocation/copy; Qt storage never borrowed into Rust |
| `icon_from_theme_fallback` | `QIcon*` | unique heap value/copy; matching *_delete | shim.cpp `icon_from_theme_fallback`: new allocation/copy; Qt storage never borrowed into Rust |
| `icon_from_file` | `QIcon*` | unique heap value/copy; matching *_delete | shim.cpp `icon_from_file`: new allocation/copy; Qt storage never borrowed into Rust |
| `label_new` | `DLabel*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `label_new`: verified new allocation; registered after guard construction |
| `suggest_button_new` | `DSuggestButton*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `suggest_button_new`: verified new allocation; registered after guard construction |
| `push_button_new` | `DPushButton*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `push_button_new`: verified new allocation; registered after guard construction |
| `qmessagebox_new` | `QMessageBox*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `qmessagebox_new`: verified new allocation; registered after guard construction |
| `qmessagebox_new_with` | `QMessageBox*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `qmessagebox_new_with`: verified new allocation; registered after guard construction |
| `qmessagebox_add_button_text` | `DPushButton*` | Qt-owned guarded view | QMessageBox::addButton creates/owns the button, not a Rust constructor owner |
| `qmessagebox_add_button_standard` | `DPushButton*` | Qt-owned guarded view | QMessageBox::addButton owns or returns existing standard button |
| `vbox_new` | `QVBoxLayout*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `vbox_new`: verified new allocation; registered after guard construction |
| `hbox_new` | `QHBoxLayout*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `hbox_new`: verified new allocation; registered after guard construction |
| `widget_new` | `QWidget*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `widget_new`: verified new allocation; registered after guard construction |
| `table_new` | `QTableWidget*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `table_new`: verified new allocation; registered after guard construction |
| `table_item` | `QTableWidgetItem*` | call-scoped item borrow | TableCell resolves coordinates under one transaction; QTableWidget owns/deletes items; item* never escapes |
| `standard_item_model_new` | `QStandardItemModel*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `standard_item_model_new`: verified new allocation; registered after guard construction |
| `standard_item_model_as_model` | `QAbstractItemModel*` | shared adjusted base view | C++ static_cast; shares original QObject guard, not a new allocation |
| `model_data` | `QVariant*` | unique heap value/copy; matching *_delete | shim.cpp `model_data`: new allocation/copy; Qt storage never borrowed into Rust |
| `variant_new` | `QVariant*` | unique heap value/copy; matching *_delete | shim.cpp `variant_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `variant_from_bool` | `QVariant*` | unique heap value/copy; matching *_delete | shim.cpp `variant_from_bool`: new allocation/copy; Qt storage never borrowed into Rust |
| `variant_from_i64` | `QVariant*` | unique heap value/copy; matching *_delete | shim.cpp `variant_from_i64`: new allocation/copy; Qt storage never borrowed into Rust |
| `variant_from_f64` | `QVariant*` | unique heap value/copy; matching *_delete | shim.cpp `variant_from_f64`: new allocation/copy; Qt storage never borrowed into Rust |
| `variant_from_string` | `QVariant*` | unique heap value/copy; matching *_delete | shim.cpp `variant_from_string`: new allocation/copy; Qt storage never borrowed into Rust |
| `variant_clone` | `QVariant*` | unique heap value/copy; matching *_delete | shim.cpp `variant_clone`: new allocation/copy; Qt storage never borrowed into Rust |
| `variant_list_new` | `QVariantList*` | unique heap value/copy; matching *_delete | shim.cpp `variant_list_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `variant_list_clone` | `QVariantList*` | unique heap value/copy; matching *_delete | shim.cpp `variant_list_clone`: new allocation/copy; Qt storage never borrowed into Rust |
| `variant_list_get` | `QVariant*` | unique heap value/copy; matching *_delete | shim.cpp `variant_list_get`: new allocation/copy; Qt storage never borrowed into Rust |
| `color_new_rgb` | `QColor*` | unique heap value/copy; matching *_delete | shim.cpp `color_new_rgb`: new allocation/copy; Qt storage never borrowed into Rust |
| `font_new` | `QFont*` | unique heap value/copy; matching *_delete | shim.cpp `font_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `palette_new` | `QPalette*` | unique heap value/copy; matching *_delete | shim.cpp `palette_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `palette_color` | `QColor*` | unique heap value/copy; matching *_delete | shim.cpp `palette_color`: new allocation/copy; Qt storage never borrowed into Rust |
| `dpalette_new` | `DPalette*` | unique heap value/copy; matching *_delete | shim.cpp `dpalette_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `dpalette_text_title` | `QBrush*` | unique heap value/copy; matching *_delete | shim.cpp `dpalette_text_title`: new allocation/copy; Qt storage never borrowed into Rust |
| `pixmap_new` | `QPixmap*` | unique heap value/copy; matching *_delete | shim.cpp `pixmap_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `image_new` | `QImage*` | unique heap value/copy; matching *_delete | shim.cpp `image_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `image_from_file` | `QImage*` | unique heap value/copy; matching *_delete | shim.cpp `image_from_file`: new allocation/copy; Qt storage never borrowed into Rust |
| `image_pixel_color` | `QColor*` | unique heap value/copy; matching *_delete | shim.cpp `image_pixel_color`: new allocation/copy; Qt storage never borrowed into Rust |
| `painter_path_new` | `QPainterPath*` | unique heap value/copy; matching *_delete | shim.cpp `painter_path_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `brush_from_color` | `QBrush*` | unique heap value/copy; matching *_delete | shim.cpp `brush_from_color`: new allocation/copy; Qt storage never borrowed into Rust |
| `brush_color` | `QColor*` | unique heap value/copy; matching *_delete | shim.cpp `brush_color`: new allocation/copy; Qt storage never borrowed into Rust |
| `key_sequence_from_string` | `QKeySequence*` | unique heap value/copy; matching *_delete | shim.cpp `key_sequence_from_string`: new allocation/copy; Qt storage never borrowed into Rust |
| `url_from_string` | `QUrl*` | unique heap value/copy; matching *_delete | shim.cpp `url_from_string`: new allocation/copy; Qt storage never borrowed into Rust |
| `q_margins_new` | `QMargins*` | unique heap value/copy; matching *_delete | shim.cpp `q_margins_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `ddci_icon_new` | `DDciIcon*` | unique heap value/copy; matching *_delete | shim.cpp `ddci_icon_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `ddci_icon_from_file` | `DDciIcon*` | unique heap value/copy; matching *_delete | shim.cpp `ddci_icon_from_file`: new allocation/copy; Qt storage never borrowed into Rust |
| `standard_icon_pixmap` | `QPixmap*` | unique heap value/copy; matching *_delete | shim.cpp `standard_icon_pixmap`: new allocation/copy; Qt storage never borrowed into Rust |
| `size_new` | `QSize*` | unique heap value/copy; matching *_delete | shim.cpp `size_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `point_new` | `QPoint*` | unique heap value/copy; matching *_delete | shim.cpp `point_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `rect_new` | `QRect*` | unique heap value/copy; matching *_delete | shim.cpp `rect_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `socket_notifier_new` | `QSocketNotifier*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `socket_notifier_new`: verified new allocation; registered after guard construction |
| `rust_delegate_new` | `QStyledItemDelegate*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `rust_delegate_new`: verified new allocation; registered after guard construction |
| `timer_new` | `QTimer*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `timer_new`: verified new allocation; registered after guard construction |
| `action_new` | `QAction*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `action_new`: verified new allocation; registered after guard construction |
| `menu_new` | `QMenu*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `menu_new`: verified new allocation; registered after guard construction |
| `paint_widget_new` | `QWidget*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `paint_widget_new`: verified new allocation; registered after guard construction |

Raw pointer-return declarations audited: **65**.

## CXX unique native state owners

- `runtime_call_new` → `UniquePtr<NativeCallFrame>`: state-only transaction token; no Qt ownership or mutable pinned reference.
- `guard_object` → `UniquePtr<QObjectGuard>`: owns only guard/event filter/QPointer, never the watched QObject.
- `guard_widget` → `UniquePtr<QObjectGuard>`: owns only guard/event filter/QPointer, never the watched QObject.
- `guard_current_application` → `UniquePtr<QObjectGuard>`: owns only guard/event filter/QPointer, never the watched QObject.
- `application_runtime_new` → `UniquePtr<ApplicationRuntime>`: private outer application owner; no user escape or callback deletion.
- `config_worker_probe_new` → `UniquePtr<ConfigWorkerProbe>`: native-only worker regression fixture; no DTK public API coverage.

## Every hand-written raw pointer input

Each row identifies a receiver, retained/call borrow, unique destructor or output, not inferred transfer by spelling.

| Native operation | Parameter | Native pointer | Ownership / lifetime rule |
|---|---|---|---|
| `runtime_can_close` | `object` | `QObject*` | borrowed QObject; only dialog close admitted in suspended native frames |
| `guard_object` | `object` | `QObject*` | live owning-thread QObject borrow; only QPointer/filter state allocated |
| `guard_widget` | `widget` | `QWidget*` | live QWidget borrow; adjusted QObject base, never ownership |
| `object_grant_constructor_authority` | `object` | `QObject*` | unsafe verified constructor/transfer-only provenance; not a borrowed getter |
| `guard_mark_adopted` | `object` | `QObject*` | record original-owner relinquishment, never claim native ownership |
| `object_can_reparent` | `child` | `QObject*` | borrow; native ancestor-cycle validation |
| `object_can_reparent` | `parent` | `QObject*` | borrow; native ancestor-cycle validation |
| `layout_can_adopt` | `parent` | `QLayout*` | borrow destination; unique unparented layout/provenance required |
| `layout_can_adopt` | `child` | `QLayout*` | borrow transfer candidate; cannot duplicate native layout ownership |
| `application_runtime_exec` | `runtime` | `ApplicationRuntime*` | private application state borrow through with; no public destruction inside exec |
| `application_runtime_track_object` | `object` | `QObject*` | unsafe verified allocation-only QPointer registration, no reparenting |
| `application_load_translator` | `app` | `DApplication*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_show` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_set_mouse_tracking` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_hide` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_resize` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_width` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_height` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_set_enabled` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_set_window_title` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_set_window_icon` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_set_window_icon` | `icon` | `QIcon*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `widget_set_fixed_size` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_raise` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_update` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_set_focus` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_move` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_set_parent` | `child` | `QWidget*` | verified constructor/transfer view; reject cycle, retained cross-owner theft and pending deletion |
| `widget_set_parent` | `parent` | `QWidget*` | borrow destination; Qt adopts child, permanent parent history |
| `tabbar_install_style` | `tb` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `tabbar_unlatch_scroll_buttons` | `tb` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `tabbar_flush_layout` | `tb` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `scrollbar_new` | `parent` | `QWidget*` | borrow existing parent; constructor creates a NEW parent-owned allocation, not ownership of the parent |
| `scrollbar_set_range` | `sb` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `scrollbar_maximum` | `sb` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `scrollbar_set_value` | `sb` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `scrollbar_value` | `sb` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `scrollbar_set_page_step` | `sb` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `paint_widget_set_ime_rect` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `main_window_titlebar_set_tabbar` | `w` | `QWidget*` | borrow main window; native qobject_cast checked |
| `main_window_titlebar_set_tabbar` | `tabbar` | `QWidget*` | Qt/native-container adoption; constructor provenance + ancestor/cross-owner rejection |
| `main_window_titlebar_add_widget` | `w` | `QWidget*` | borrow main window; native qobject_cast checked |
| `main_window_titlebar_add_widget` | `child` | `QWidget*` | Qt/native-container adoption; constructor provenance + ancestor/cross-owner rejection |
| `widget_set_titlebar_icon` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_set_titlebar_icon` | `icon` | `QIcon*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `widget_activate_window` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_close` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_is_visible` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_set_focus_policy` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `progressbar_set_value` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `line_edit_text` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `progressbar_set_range` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `progressbar_value` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_set_font` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_set_font` | `font` | `QFont*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `widget_set_cursor` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_unset_cursor` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_palette` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_set_palette` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `widget_set_palette` | `pal` | `QPalette*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `object_delete_later` | `o` | `QObject*` | deletion REQUEST only; safe wrappers require provenance; active frames retain DeferredDelete |
| `object_has_parent` | `o` | `QObject*` | borrow; diagnostic parent query, not a complete ownership proof |
| `mainwindow_titlebar` | `w` | `DMainWindow*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `mainwindow_set_central_widget` | `w` | `DMainWindow*` | borrow destination; replacement is native-managed |
| `mainwindow_set_central_widget` | `central` | `QWidget*` | Qt/native-container adoption; verified take is the only release path |
| `mainwindow_take_central_widget` | `w` | `DMainWindow*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `mainwindow_set_window_radius` | `w` | `DMainWindow*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `mainwindow_set_enable_blur` | `w` | `DMainWindow*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `titlebar_set_title` | `tb` | `DTitlebar*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `titlebar_set_icon` | `tb` | `DTitlebar*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `icon_from_theme_fallback` | `fallback` | `QIcon*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `label_set_text` | `l` | `DLabel*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `label_set_word_wrap` | `l` | `DLabel*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `label_set_alignment` | `l` | `DLabel*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `label_set_pixmap` | `l` | `DLabel*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `label_set_pixmap` | `pm` | `QPixmap*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `button_set_text` | `b` | `DPushButton*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `button_click` | `b` | `DPushButton*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `qmessagebox_new_with` | `parent` | `QWidget*` | borrow existing parent; constructor creates a NEW parent-owned allocation, not ownership of the parent |
| `qmessagebox_set_text` | `mb` | `QMessageBox*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `qmessagebox_set_icon` | `mb` | `QMessageBox*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `qmessagebox_set_standard_buttons` | `mb` | `QMessageBox*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `qmessagebox_set_informative_text` | `mb` | `QMessageBox*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `qmessagebox_set_detailed_text` | `mb` | `QMessageBox*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `qmessagebox_add_button_text` | `mb` | `QMessageBox*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `qmessagebox_add_button_standard` | `mb` | `QMessageBox*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `qmessagebox_set_default_button` | `mb` | `QMessageBox*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `qmessagebox_exec` | `mb` | `QMessageBox*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `qmessagebox_clicked_button` | `mb` | `QMessageBox*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `qmessagebox_text` | `mb` | `QMessageBox*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `qmessagebox_information` | `parent` | `QWidget*` | call-scoped parent borrow through modal native helper; transaction/DeferredDelete protection, no parent transfer |
| `qmessagebox_warning` | `parent` | `QWidget*` | call-scoped parent borrow through modal native helper; transaction/DeferredDelete protection, no parent transfer |
| `qmessagebox_critical` | `parent` | `QWidget*` | call-scoped parent borrow through modal native helper; transaction/DeferredDelete protection, no parent transfer |
| `qmessagebox_question` | `parent` | `QWidget*` | call-scoped parent borrow through modal native helper; transaction/DeferredDelete protection, no parent transfer |
| `qmessagebox_about` | `parent` | `QWidget*` | call-scoped parent borrow through modal native helper; transaction/DeferredDelete protection, no parent transfer |
| `vbox_new` | `parent` | `QWidget*` | borrow existing parent; constructor creates a NEW parent-owned allocation, not ownership of the parent |
| `hbox_new` | `parent` | `QWidget*` | borrow existing parent; constructor creates a NEW parent-owned allocation, not ownership of the parent |
| `widget_new` | `parent` | `QWidget*` | borrow existing parent; constructor creates a NEW parent-owned allocation, not ownership of the parent |
| `layout_add_widget` | `l` | `QLayout*` | borrow destination layout; managed-widget retention recorded even before installation |
| `layout_add_widget` | `w` | `QWidget*` | layout/native retention plus QWidget parenting when installed; reject second raw item |
| `layout_add_widget_ex` | `l` | `QLayout*` | borrow destination layout; managed-widget retention recorded even before installation |
| `layout_add_widget_ex` | `w` | `QWidget*` | layout/native retention plus QWidget parenting when installed; reject second raw item |
| `layout_add_stretch` | `l` | `QLayout*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `layout_add_layout` | `l` | `QLayout*` | borrow destination; no cyclic/duplicate/parented child layout |
| `layout_add_layout` | `child` | `QLayout*` | native ownership adoption; addChildLayout validated before insertion |
| `layout_set_spacing` | `l` | `QLayout*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `layout_set_contents_margins` | `l` | `QLayout*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `table_set_column_count` | `t` | `QTableWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `table_set_row_count` | `t` | `QTableWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `table_set_horizontal_header_labels` | `t` | `QTableWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `table_horizontal_header_label` | `t` | `QTableWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `table_set_cell_text` | `t` | `QTableWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `table_set_cell_data` | `t` | `QTableWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `table_cell_data` | `t` | `QTableWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `table_cell_text` | `t` | `QTableWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `table_current_row` | `t` | `QTableWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `table_row_count` | `t` | `QTableWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `table_select_rows_readonly` | `t` | `QTableWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `table_header_stretch_last` | `t` | `QTableWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `table_set_column_width` | `t` | `QTableWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `table_item` | `t` | `QTableWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `table_select_row` | `t` | `QTableWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `table_hide_headers` | `t` | `QTableWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `table_set_section_resize_mode` | `t` | `QTableWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `table_set_vertical_header_default_section_size` | `t` | `QTableWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `table_set_show_grid` | `t` | `QTableWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `table_set_frame_shape` | `t` | `QTableWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `table_set_icon_size` | `t` | `QTableWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `table_set_delegate_for_column` | `t` | `QTableWidget*` | borrow table; model/paint frame transaction includes setter |
| `table_set_delegate_for_column` | `delegate` | `QStyledItemDelegate*` | retained borrow, NOT transfer; qabstractitemview_p.h stores QPointer delegates |
| `standard_item_model_new` | `parent` | `QObject*` | borrow existing parent; constructor creates a NEW parent-owned allocation, not ownership of the parent |
| `standard_item_model_set_column_count` | `model` | `QStandardItemModel*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `standard_item_model_as_model` | `model` | `QStandardItemModel*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `model_set_parent` | `model` | `QAbstractItemModel*` | verified model constructor view; reject cyclic/stolen native-parent handoff |
| `model_set_parent` | `parent` | `QObject*` | borrow destination; parent owns model after accepted QObject handoff |
| `model_row_count` | `model` | `QAbstractItemModel*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `model_column_count` | `model` | `QAbstractItemModel*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `model_data` | `model` | `QAbstractItemModel*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `model_set_data` | `model` | `QAbstractItemModel*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `model_set_data` | `data` | `QVariant*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `model_insert_rows` | `model` | `QAbstractItemModel*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `model_remove_rows` | `model` | `QAbstractItemModel*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `item_set_icon` | `it` | `QTableWidgetItem*` | call-scoped table-owned item borrow resolved by TableCell under same transaction; never stored |
| `item_set_icon` | `icon` | `QIcon*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `item_set_text_alignment` | `it` | `QTableWidgetItem*` | call-scoped table-owned item borrow resolved by TableCell under same transaction; never stored |
| `item_set_foreground` | `it` | `QTableWidgetItem*` | call-scoped table-owned item borrow resolved by TableCell under same transaction; never stored |
| `item_set_foreground` | `color` | `QColor*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `item_set_data_string` | `it` | `QTableWidgetItem*` | call-scoped table-owned item borrow resolved by TableCell under same transaction; never stored |
| `item_data_string` | `it` | `QTableWidgetItem*` | call-scoped table-owned item borrow resolved by TableCell under same transaction; never stored |
| `item_set_data_bool` | `it` | `QTableWidgetItem*` | call-scoped table-owned item borrow resolved by TableCell under same transaction; never stored |
| `item_data_bool` | `it` | `QTableWidgetItem*` | call-scoped table-owned item borrow resolved by TableCell under same transaction; never stored |
| `variant_clone` | `value` | `QVariant*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `variant_is_valid` | `value` | `QVariant*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `variant_to_bool` | `value` | `QVariant*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `variant_to_i64` | `value` | `QVariant*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `variant_to_i64` | `ok` | `bool*` | call-scoped initialized Rust bool output pointer; no escape/retention |
| `variant_to_string` | `value` | `QVariant*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `variant_delete` | `value` | `QVariant*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `variant_list_clone` | `list` | `QVariantList*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `variant_list_push` | `list` | `QVariantList*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `variant_list_push` | `value` | `QVariant*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `variant_list_len` | `list` | `QVariantList*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `variant_list_get` | `list` | `QVariantList*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `variant_list_delete` | `list` | `QVariantList*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `color_rgba` | `c` | `QColor*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `color_delete` | `c` | `QColor*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `font_set_point_size` | `f` | `QFont*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `font_set_bold` | `f` | `QFont*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `font_set_italic` | `f` | `QFont*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `fontmetrics_height` | `f` | `QFont*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `fontmetrics_ascent` | `f` | `QFont*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `fontmetrics_max_width` | `f` | `QFont*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `fontmetrics_advance` | `f` | `QFont*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `font_force_integer_metrics` | `f` | `QFont*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `font_set_monospace` | `f` | `QFont*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `font_set_family` | `f` | `QFont*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `font_delete` | `f` | `QFont*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `palette_set_color` | `pal` | `QPalette*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `palette_set_color` | `color` | `QColor*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `palette_color` | `pal` | `QPalette*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `palette_delete` | `pal` | `QPalette*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `dpalette_text_title` | `pal` | `DPalette*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `dpalette_delete` | `pal` | `DPalette*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `pixmap_delete` | `pm` | `QPixmap*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `image_is_null` | `image` | `QImage*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `image_width` | `image` | `QImage*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `image_height` | `image` | `QImage*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `image_fill` | `image` | `QImage*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `image_fill` | `color` | `QColor*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `image_set_text` | `image` | `QImage*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `image_text` | `image` | `QImage*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `image_save` | `image` | `QImage*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `image_pixel_color` | `image` | `QImage*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `image_delete` | `image` | `QImage*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `painter_path_add_rect` | `path` | `QPainterPath*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `painter_path_is_empty` | `path` | `QPainterPath*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `painter_path_element_count` | `path` | `QPainterPath*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `painter_path_delete` | `path` | `QPainterPath*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `brush_from_color` | `color` | `QColor*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `brush_color` | `brush` | `QBrush*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `brush_delete` | `brush` | `QBrush*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `key_sequence_to_string` | `seq` | `QKeySequence*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `key_sequence_delete` | `seq` | `QKeySequence*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `url_to_string` | `url` | `QUrl*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `url_is_valid` | `url` | `QUrl*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `url_delete` | `url` | `QUrl*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `margins_delete` | `m` | `QMargins*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `ddci_icon_delete` | `i` | `DDciIcon*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `standard_icon_pixmap` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `size_delete` | `s` | `QSize*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `point_delete` | `p` | `QPoint*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `rect_delete` | `r` | `QRect*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `rect_width` | `r` | `QRect*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `rect_height` | `r` | `QRect*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `rect_x` | `r` | `QRect*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `rect_y` | `r` | `QRect*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `icon_delete` | `icon` | `QIcon*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `socket_notifier_new` | `parent` | `QObject*` | borrow existing parent; constructor creates a NEW parent-owned allocation, not ownership of the parent |
| `socket_notifier_set_enabled` | `n` | `QSocketNotifier*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `rust_delegate_new` | `parent` | `QObject*` | borrow existing parent; constructor creates a NEW parent-owned allocation, not ownership of the parent |
| `painter_draw_text_at` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_draw_line` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_save` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_restore` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_set_pen_color` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_set_pen_color` | `color` | `QColor*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `painter_set_font` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_set_font` | `font` | `QFont*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `painter_draw_text` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_draw_pixmap` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_draw_pixmap` | `pm` | `QPixmap*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `painter_draw_icon` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_draw_icon` | `icon` | `QIcon*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `painter_fill_rect` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_fill_rect` | `color` | `QColor*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `painter_set_clip_rect` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_translate` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_scale` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_elided_text` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `index_data_string` | `idx` | `QModelIndex*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `index_data_bool` | `idx` | `QModelIndex*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `index_data_i64` | `idx` | `QModelIndex*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `timer_new` | `parent` | `QObject*` | borrow existing parent; constructor creates a NEW parent-owned allocation, not ownership of the parent |
| `timer_start` | `t` | `QTimer*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `timer_stop` | `t` | `QTimer*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `action_new` | `parent` | `QObject*` | borrow existing parent; constructor creates a NEW parent-owned allocation, not ownership of the parent |
| `action_set_enabled` | `action` | `QAction*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `action_is_enabled` | `action` | `QAction*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `action_set_visible` | `action` | `QAction*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `action_is_visible` | `action` | `QAction*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `action_set_text` | `action` | `QAction*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `action_text` | `action` | `QAction*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `action_trigger` | `action` | `QAction*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `menu_new` | `parent` | `QWidget*` | borrow existing parent; constructor creates a NEW parent-owned allocation, not ownership of the parent |
| `menu_add_existing_action` | `menu` | `QMenu*` | borrow menu destination |
| `menu_add_existing_action` | `action` | `QAction*` | retained Qt action borrow, existing QObject parent unchanged |
| `menu_add_action_cb` | `m` | `QMenu*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `menu_add_separator` | `m` | `QMenu*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `menu_popup` | `m` | `QMenu*` | borrow receiver; popup does not take widget-reference ownership |
| `menu_popup` | `ref_` | `QWidget*` | borrow for mapToGlobal only, not retained |
| `relay_connect0` | `sender` | `QObject*` | borrow sender; binding relay is created as child/scope-tracked; no ownership of sender |
| `relay_connect_i32` | `sender` | `QObject*` | borrow sender; binding relay is created as child/scope-tracked; no ownership of sender |
| `relay_connect_bool` | `sender` | `QObject*` | borrow sender; binding relay is created as child/scope-tracked; no ownership of sender |
| `relay_connect_i32_i32` | `sender` | `QObject*` | borrow sender; binding relay is created as child/scope-tracked; no ownership of sender |
| `paint_widget_new` | `parent` | `QWidget*` | borrow existing parent; constructor creates a NEW parent-owned allocation, not ownership of the parent |
| `paint_widget_inject_key` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `shortcut_new` | `parent` | `QWidget*` | borrow widget owner; new native shortcut scope/parent-owned; callback capture cancellation |

Raw pointer input parameters audited: **267**.

## Callback and retained borrows

- `QPainter*` / `QModelIndex*`: callback-lifetime `&Painter` / `&ModelIndex`; never Clone/Copy/owned pointers. Synchronous native borrowing rejects QObject access and nested loops; independent Qt values and painter/index operations remain usable.
- QObject parameters/base views: non-owning, owning-thread QPointer checks plus native transaction spanning arguments, call, and result wrapping. Shared views never manufacture exclusive/pinned references.
- QWidget/layout adoption: cycle/duplicate-parent checks, permanent adoption history (including QObject ChildAdded and parentless native transfers); old constructor owners cannot reclaim transferred allocations.
- Model/delegate setters: Qt retains borrowed QObject pointers; Qt ownership is not inferred from setter spelling. Callback-time mutation cannot invalidate a currently borrowed paint/index/model frame.
- Relays, timers, shortcuts and actions: native owner destruction cancels pending events immediately; capture destructors are retired to a safe Rust boundary, with scope cleanup outside registry borrows.
