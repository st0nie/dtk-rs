# Hand-written FFI pointer/ownership audit

Source authority: matching apt source Qt 6.8.0+dfsg-0deepin31, dtkwidget 6.7.47.1-1, dtkcore/dtkgui 6.7.47.
Regenerate with `uv run --no-project python tools/audit_hand.py`. New pointer-return functions fail closed until reviewed.

| Bridge API | Pointer type | Ownership | Evidence / lifetime |
|---|---|---|---|
| `application_runtime_application` | `QObject*` | scope borrow | application_runtime.cpp: runtime.application(); no public owning/Clone token |
| `application_runtime_gui_application` | `DApplication*` | scope borrow | shim.cpp: qobject_cast of private runtime application |
| `core_config_create_generic` | `DConfig*` | unique Owned<DConfig> | dtkcore/src/dconfig.cpp createGeneric: explicitly released by caller; parent nullptr |
| `core_config_create` | `DConfig*` | unique Owned<DConfig> | dtkcore/src/dconfig.cpp create: explicitly released by caller; parent nullptr |
| `core_settings_from_json` | `DSettings*` | unique Owned<DSettings> | dtkcore/src/settings/dsettings.cpp fromJson: QPointer(new DSettings) without parent, documented as caller-released |
| `core_settings_from_json_file` | `DSettings*` | unique Owned<DSettings> | dtkcore/src/settings/dsettings.cpp fromJsonFile -> fromJson: parentless new DSettings |
| `core_file_watcher_new` | `DFileWatcher*` | constructor Owned; Qt adoption revokes original deletion | dtkcore/src/filesystem/dfilewatcher.cpp: DFileWatcher(path, parent = 0) allocation |
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
| `model_index` | `QPersistentModelIndex*` | unique heap value/copy; matching *_delete | shim.cpp `model_index`: new allocation/copy; Qt storage never borrowed into Rust |
| `variant_new` | `QVariant*` | unique heap value/copy; matching *_delete | shim.cpp `variant_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `variant_from_bool` | `QVariant*` | unique heap value/copy; matching *_delete | shim.cpp `variant_from_bool`: new allocation/copy; Qt storage never borrowed into Rust |
| `variant_from_i64` | `QVariant*` | unique heap value/copy; matching *_delete | shim.cpp `variant_from_i64`: new allocation/copy; Qt storage never borrowed into Rust |
| `variant_from_f64` | `QVariant*` | unique heap value/copy; matching *_delete | shim.cpp `variant_from_f64`: new allocation/copy; Qt storage never borrowed into Rust |
| `variant_from_string` | `QVariant*` | unique heap value/copy; matching *_delete | shim.cpp `variant_from_string`: new allocation/copy; Qt storage never borrowed into Rust |
| `variant_clone` | `QVariant*` | unique heap value/copy; matching *_delete | shim.cpp `variant_clone`: new allocation/copy; Qt storage never borrowed into Rust |
| `variant_list_new` | `QVariantList*` | unique heap value/copy; matching *_delete | shim.cpp `variant_list_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `variant_list_clone` | `QVariantList*` | unique heap value/copy; matching *_delete | shim.cpp `variant_list_clone`: new allocation/copy; Qt storage never borrowed into Rust |
| `variant_list_get` | `QVariant*` | unique heap value/copy; matching *_delete | shim.cpp `variant_list_get`: new allocation/copy; Qt storage never borrowed into Rust |
| `font_new` | `QFont*` | unique heap value/copy; matching *_delete | shim.cpp `font_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `palette_new` | `QPalette*` | unique heap value/copy; matching *_delete | shim.cpp `palette_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `dpalette_new` | `DPalette*` | unique heap value/copy; matching *_delete | shim.cpp `dpalette_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `dpalette_text_title` | `QBrush*` | unique heap value/copy; matching *_delete | shim.cpp `dpalette_text_title`: new allocation/copy; Qt storage never borrowed into Rust |
| `pixmap_new` | `QPixmap*` | unique heap value/copy; matching *_delete | shim.cpp `pixmap_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `image_new` | `QImage*` | unique heap value/copy; matching *_delete | shim.cpp `image_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `image_from_file` | `QImage*` | unique heap value/copy; matching *_delete | shim.cpp `image_from_file`: new allocation/copy; Qt storage never borrowed into Rust |
| `painter_path_new` | `QPainterPath*` | unique heap value/copy; matching *_delete | shim.cpp `painter_path_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `brush_from_color` | `QBrush*` | unique heap value/copy; matching *_delete | shim.cpp `brush_from_color`: new allocation/copy; Qt storage never borrowed into Rust |
| `key_sequence_from_string` | `QKeySequence*` | unique heap value/copy; matching *_delete | shim.cpp `key_sequence_from_string`: new allocation/copy; Qt storage never borrowed into Rust |
| `ddci_icon_new` | `DDciIcon*` | unique heap value/copy; matching *_delete | shim.cpp `ddci_icon_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `ddci_icon_from_file` | `DDciIcon*` | unique heap value/copy; matching *_delete | shim.cpp `ddci_icon_from_file`: new allocation/copy; Qt storage never borrowed into Rust |
| `standard_icon_pixmap` | `QPixmap*` | unique heap value/copy; matching *_delete | shim.cpp `standard_icon_pixmap`: new allocation/copy; Qt storage never borrowed into Rust |
| `socket_notifier_new` | `QSocketNotifier*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `socket_notifier_new`: verified new allocation; registered after guard construction |
| `rust_delegate_new` | `QStyledItemDelegate*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `rust_delegate_new`: verified new allocation; registered after guard construction |
| `widget_style` | `QStyle*` | borrowed guarded view | qt6-base qwidget.cpp style(): widget's own style or QApplication::style(); never transferred |
| `style_as_dstyle` | `DStyle*` | borrowed guarded view | qobject_cast of the same borrowed style object; no allocation |
| `style_option_new` | `QStyleOption*` | unique heap value/copy; matching *_delete | shim.cpp `style_option_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `style_option_clone` | `QStyleOption*` | unique heap value/copy; matching *_delete | shim.cpp `style_option_clone`: new allocation/copy; Qt storage never borrowed into Rust |
| `view_item_option_new` | `QStyleOptionViewItem*` | unique heap value/copy; matching *_delete | shim.cpp `view_item_option_new`: new allocation/copy; Qt storage never borrowed into Rust |
| `view_item_option_clone` | `QStyleOptionViewItem*` | unique heap value/copy; matching *_delete | shim.cpp `view_item_option_clone`: new allocation/copy; Qt storage never borrowed into Rust |
| `view_item_option_base` | `QStyleOption*` | borrowed base view | static upcast into the same option allocation; deleted only through the owning QStyleOptionViewItem |
| `painter_begin_image` | `QPainter*` | closure-scoped unique owner | new QPainter(image); Painter::on_image ends and deletes it (painter_finish) before returning, image borrowed &mut |
| `rust_list_model_new` | `QAbstractItemModel*` | constructor Owned; parent adoption | shim.cpp RustListModel(cb_id, editable, parent): new parent-owned QObject holding only a callback id |
| `persistent_index_from` | `QPersistentModelIndex*` | unique heap value/copy; matching *_delete | shim.cpp `persistent_index_from`: new allocation/copy; Qt storage never borrowed into Rust |
| `persistent_index_clone` | `QPersistentModelIndex*` | unique heap value/copy; matching *_delete | shim.cpp `persistent_index_clone`: new allocation/copy; Qt storage never borrowed into Rust |
| `persistent_index_data` | `QVariant*` | unique heap value/copy; matching *_delete | shim.cpp `persistent_index_data`: new allocation/copy; Qt storage never borrowed into Rust |
| `persistent_index_parent` | `QPersistentModelIndex*` | unique heap value/copy; matching *_delete | shim.cpp `persistent_index_parent`: new allocation/copy; Qt storage never borrowed into Rust |
| `timer_new` | `QTimer*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `timer_new`: verified new allocation; registered after guard construction |
| `action_new` | `QAction*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `action_new`: verified new allocation; registered after guard construction |
| `menu_new` | `QMenu*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `menu_new`: verified new allocation; registered after guard construction |
| `paint_widget_new` | `QWidget*` | constructor Owned; Qt adoption revokes original deletion | shim.cpp `paint_widget_new`: verified new allocation; registered after guard construction |

Raw pointer-return declarations audited: **73**.

## CXX unique native state owners

- `runtime_call_new` → `UniquePtr<NativeCallFrame>`: state-only transaction token; no Qt ownership or mutable pinned reference.
- `runtime_paint_call_new` → `UniquePtr<NativeCallFrame>`: state-only transaction token; no Qt ownership or mutable pinned reference.
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
| `core_settings_set_file_backend` | `settings` | `DSettings*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
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
| `abstract_button_set_checked` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
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
| `model_index` | `model` | `QAbstractItemModel*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `model_index` | `parent` | `QPersistentModelIndex*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `model_set_index_data` | `model` | `QAbstractItemModel*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `model_set_index_data` | `index` | `QPersistentModelIndex*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `model_set_index_data` | `data` | `QVariant*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `item_set_icon` | `it` | `QTableWidgetItem*` | call-scoped table-owned item borrow resolved by TableCell under same transaction; never stored |
| `item_set_icon` | `icon` | `QIcon*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `item_set_text_alignment` | `it` | `QTableWidgetItem*` | call-scoped table-owned item borrow resolved by TableCell under same transaction; never stored |
| `item_set_foreground` | `it` | `QTableWidgetItem*` | call-scoped table-owned item borrow resolved by TableCell under same transaction; never stored |
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
| `palette_color` | `pal` | `QPalette*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `palette_delete` | `pal` | `QPalette*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `dpalette_text_title` | `pal` | `DPalette*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `dpalette_delete` | `pal` | `DPalette*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `pixmap_delete` | `pm` | `QPixmap*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `image_is_null` | `image` | `QImage*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `image_width` | `image` | `QImage*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `image_height` | `image` | `QImage*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `image_fill` | `image` | `QImage*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `image_set_text` | `image` | `QImage*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `image_text` | `image` | `QImage*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `image_save` | `image` | `QImage*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `image_pixel_color` | `image` | `QImage*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `image_delete` | `image` | `QImage*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `painter_path_add_rect` | `path` | `QPainterPath*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `painter_path_is_empty` | `path` | `QPainterPath*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `painter_path_element_count` | `path` | `QPainterPath*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `painter_path_delete` | `path` | `QPainterPath*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `brush_color` | `brush` | `QBrush*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `brush_delete` | `brush` | `QBrush*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `key_sequence_to_string` | `seq` | `QKeySequence*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `key_sequence_delete` | `seq` | `QKeySequence*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `ddci_icon_delete` | `i` | `DDciIcon*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `standard_icon_pixmap` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `icon_delete` | `icon` | `QIcon*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `socket_notifier_new` | `parent` | `QObject*` | borrow existing parent; constructor creates a NEW parent-owned allocation, not ownership of the parent |
| `socket_notifier_set_enabled` | `n` | `QSocketNotifier*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `rust_delegate_new` | `parent` | `QObject*` | borrow existing parent; constructor creates a NEW parent-owned allocation, not ownership of the parent |
| `widget_style` | `w` | `QWidget*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `style_as_dstyle` | `style` | `QStyle*` | receiver borrow, no ownership transfer; guarded owning-thread native transaction through result wrapping |
| `style_option_init_from` | `opt` | `QStyleOption*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `style_option_init_from` | `w` | `QWidget*` | call borrow; initFrom copies state, styleObject cleared (no retained QObject) |
| `style_option_set_rect` | `opt` | `QStyleOption*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `style_option_set_state` | `opt` | `QStyleOption*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `style_option_delete` | `opt` | `QStyleOption*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `view_item_option_base` | `opt` | `QStyleOptionViewItem*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `view_item_option_set_text` | `opt` | `QStyleOptionViewItem*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `view_item_option_delete` | `opt` | `QStyleOptionViewItem*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `painter_begin_image` | `image` | `QImage*` | exclusive &mut QImage borrow for the closure; painter ended before return |
| `painter_finish` | `p` | `QPainter*` | closure-scoped painter created by painter_begin_image; finished exactly once |
| `painter_is_active` | `p` | `QPainter*` | closure-scoped painter created by painter_begin_image; finished exactly once |
| `rust_list_model_new` | `parent` | `QObject*` | borrow existing parent; constructor creates a NEW parent-owned model |
| `rust_list_model_begin_insert` | `m` | `QAbstractItemModel*` | receiver: ListModel's own guarded RustListModel (dynamic_cast checked) inside one NativeCall |
| `rust_list_model_end_insert` | `m` | `QAbstractItemModel*` | receiver: ListModel's own guarded RustListModel (dynamic_cast checked) inside one NativeCall |
| `rust_list_model_begin_remove` | `m` | `QAbstractItemModel*` | receiver: ListModel's own guarded RustListModel (dynamic_cast checked) inside one NativeCall |
| `rust_list_model_end_remove` | `m` | `QAbstractItemModel*` | receiver: ListModel's own guarded RustListModel (dynamic_cast checked) inside one NativeCall |
| `rust_list_model_begin_reset` | `m` | `QAbstractItemModel*` | receiver: ListModel's own guarded RustListModel (dynamic_cast checked) inside one NativeCall |
| `rust_list_model_end_reset` | `m` | `QAbstractItemModel*` | receiver: ListModel's own guarded RustListModel (dynamic_cast checked) inside one NativeCall |
| `rust_list_model_changed` | `m` | `QAbstractItemModel*` | receiver: ListModel's own guarded RustListModel (dynamic_cast checked) inside one NativeCall |
| `painter_draw_text_at` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_draw_line` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_save` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_restore` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_set_pen_color` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_set_font` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_set_font` | `font` | `QFont*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `painter_draw_text` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_draw_pixmap` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_draw_pixmap` | `pm` | `QPixmap*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `painter_draw_icon` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_draw_icon` | `icon` | `QIcon*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `painter_fill_rect` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_set_clip_rect` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_translate` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_scale` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `painter_elided_text` | `p` | `QPainter*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `index_data_string` | `idx` | `QModelIndex*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `index_data_bool` | `idx` | `QModelIndex*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `index_data_i64` | `idx` | `QModelIndex*` | synchronous callback-scoped borrow; never owned/Clone/Copy; related native mutation rejected |
| `persistent_index_from` | `idx` | `QModelIndex*` | synchronous callback-scoped QModelIndex borrow; copied into a new persistent index |
| `persistent_index_clone` | `idx` | `QPersistentModelIndex*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `persistent_index_delete` | `idx` | `QPersistentModelIndex*` | consumes unique Rust-owned heap value; matching Drop only, never a QObject borrow |
| `persistent_index_is_valid` | `idx` | `QPersistentModelIndex*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `persistent_index_row` | `idx` | `QPersistentModelIndex*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `persistent_index_column` | `idx` | `QPersistentModelIndex*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `persistent_index_data` | `idx` | `QPersistentModelIndex*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
| `persistent_index_parent` | `idx` | `QPersistentModelIndex*` | borrow of independently Rust-owned Qt value; native value/reference conversion, no allocation transfer |
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

Raw pointer input parameters audited: **284**.

## Callback and retained borrows

- `QPainter*` / `QModelIndex*`: callback-lifetime `&Painter` / `&ModelIndex`; never Clone/Copy/owned pointers. Synchronous native borrowing rejects QObject access and nested loops; independent Qt values and painter/index operations remain usable.
- QObject parameters/base views: non-owning, owning-thread QPointer checks plus native transaction spanning arguments, call, and result wrapping. Shared views never manufacture exclusive/pinned references.
- QWidget/layout adoption: cycle/duplicate-parent checks, permanent adoption history (including QObject ChildAdded and parentless native transfers); old constructor owners cannot reclaim transferred allocations.
- Model/delegate setters: Qt retains borrowed QObject pointers; Qt ownership is not inferred from setter spelling. Callback-time mutation cannot invalidate a currently borrowed paint/index/model frame.
- Relays, timers, shortcuts and actions: native owner destruction cancels pending events immediately; capture destructors are retired to a safe Rust boundary, with scope cleanup outside registry borrows.

## By-value Qt types (cxx-qt-lib)

No heap allocation, deleter or pointer crosses FFI: returns are C++ copies moved into Rust, inputs are `const T &` borrows of Rust-owned values. These are plain Rust values with cxx-qt-lib's Clone/Drop (no QObject reference, no scope guard); independent of the application scope.

| Type | Bridges | Copy semantics |
|---|---|---|
| `QColor` | lib.rs, gen_ffi.rs, core_ffi.rs, gui_ffi.rs | trivially copyable (Qt6 static_assert in cxx-qt-lib qcolor.cpp) |
| `QMargins` | lib.rs, gen_ffi.rs, core_ffi.rs, gui_ffi.rs | trivially copyable POD |
| `QMarginsF` | lib.rs, gen_ffi.rs, core_ffi.rs, gui_ffi.rs | trivially copyable POD |
| `QPoint` | lib.rs, gen_ffi.rs, core_ffi.rs, gui_ffi.rs | trivially copyable POD |
| `QPointF` | lib.rs, gen_ffi.rs, core_ffi.rs, gui_ffi.rs | trivially copyable POD |
| `QRect` | lib.rs, gen_ffi.rs, core_ffi.rs, gui_ffi.rs | trivially copyable POD |
| `QRectF` | lib.rs, gen_ffi.rs, core_ffi.rs, gui_ffi.rs | trivially copyable POD |
| `QSize` | lib.rs, gen_ffi.rs, core_ffi.rs, gui_ffi.rs | trivially copyable POD |
| `QSizeF` | lib.rs, gen_ffi.rs, core_ffi.rs, gui_ffi.rs | trivially copyable POD |
| `QUrl` | lib.rs, gen_ffi.rs, core_ffi.rs, gui_ffi.rs | implicitly shared d-pointer; `rust::IsRelocatable<QUrl>`; Rust Clone/Drop call the C++ copy ctor/destructor |

## CXX-Qt imported QObjects (dtk-cxxqt)

Imports only: no construction, ownership, deletion or `Pin<&mut T>`/`&mut T` receiver. Senders are guarded live QObject pointers inside a `NativeCall`, cast with C++ `dynamic_cast` to the declaring class (generated by tools/signals.py); functors own a thread-checked `SignalToken` that queues owned values (by-value copies, `OwnedValue` heap copies with their matching deleter, `QPersistentModelIndex` for `QModelIndex`) and cancels/retires the callback when Qt destroys it.

| C++ type | Item | Receiver | Kind | Rust-facing glue |
|---|---|---|---|---|
| `ColorButton` | `select_color_button` | `&ColorButton` | typed signal connection | `connectors::color_button_select_color_button` |
| `ColorButton` | `btn_is_checked` | `&ColorButton` | typed signal connection | `connectors::color_button_btn_is_checked` |
| `ColorLabel` | `clicked` | `&ColorLabel` | typed signal connection | `connectors::color_label_clicked` |
| `ColorLabel` | `picked_color` | `&ColorLabel` | typed signal connection | `connectors::color_label_picked_color` |
| `DAboutDialog` | `feature_activated` | `&DAboutDialog` | typed signal connection | `connectors::d_about_dialog_feature_activated` |
| `DAboutDialog` | `license_activated` | `&DAboutDialog` | typed signal connection | `connectors::d_about_dialog_license_activated` |
| `DAbstractDialog` | `size_changed` | `&DAbstractDialog` | typed signal connection | `connectors::d_abstract_dialog_size_changed` |
| `DAlertControl` | `alert_changed` | `&DAlertControl` | typed signal connection | `connectors::d_alert_control_alert_changed` |
| `DAnchorsBase` | `enabled_changed` | `&DAnchorsBase` | typed signal connection | `connectors::d_anchors_base_enabled_changed` |
| `DAnchorsBase` | `margins_changed` | `&DAnchorsBase` | typed signal connection | `connectors::d_anchors_base_margins_changed` |
| `DAnchorsBase` | `top_margin_changed` | `&DAnchorsBase` | typed signal connection | `connectors::d_anchors_base_top_margin_changed` |
| `DAnchorsBase` | `bottom_margin_changed` | `&DAnchorsBase` | typed signal connection | `connectors::d_anchors_base_bottom_margin_changed` |
| `DAnchorsBase` | `left_margin_changed` | `&DAnchorsBase` | typed signal connection | `connectors::d_anchors_base_left_margin_changed` |
| `DAnchorsBase` | `right_margin_changed` | `&DAnchorsBase` | typed signal connection | `connectors::d_anchors_base_right_margin_changed` |
| `DAnchorsBase` | `horizontal_center_offset_changed` | `&DAnchorsBase` | typed signal connection | `connectors::d_anchors_base_horizontal_center_offset_changed` |
| `DAnchorsBase` | `vertical_center_offset_changed` | `&DAnchorsBase` | typed signal connection | `connectors::d_anchors_base_vertical_center_offset_changed` |
| `DAnchorsBase` | `align_when_centered_changed` | `&DAnchorsBase` | typed signal connection | `connectors::d_anchors_base_align_when_centered_changed` |
| `DArrowButton` | `mouse_press` | `&DArrowButton` | typed signal connection | `connectors::d_arrow_button_mouse_press` |
| `DArrowButton` | `mouse_release` | `&DArrowButton` | typed signal connection | `connectors::d_arrow_button_mouse_release` |
| `DArrowButton` | `mouse_enter` | `&DArrowButton` | typed signal connection | `connectors::d_arrow_button_mouse_enter` |
| `DArrowButton` | `mouse_leave` | `&DArrowButton` | typed signal connection | `connectors::d_arrow_button_mouse_leave` |
| `DArrowRectangle` | `window_deactivate` | `&DArrowRectangle` | typed signal connection | `connectors::d_arrow_rectangle_window_deactivate` |
| `DBackgroundGroup` | `use_widget_background_changed` | `&DBackgroundGroup` | typed signal connection | `connectors::d_background_group_use_widget_background_changed` |
| `DBaseFileWatcher` | `file_deleted` | `&DBaseFileWatcher` | typed signal connection | `connectors::d_base_file_watcher_file_deleted` |
| `DBaseFileWatcher` | `file_attribute_changed` | `&DBaseFileWatcher` | typed signal connection | `connectors::d_base_file_watcher_file_attribute_changed` |
| `DBaseFileWatcher` | `file_moved` | `&DBaseFileWatcher` | typed signal connection | `connectors::d_base_file_watcher_file_moved` |
| `DBaseFileWatcher` | `subfile_created` | `&DBaseFileWatcher` | typed signal connection | `connectors::d_base_file_watcher_subfile_created` |
| `DBaseFileWatcher` | `file_modified` | `&DBaseFileWatcher` | typed signal connection | `connectors::d_base_file_watcher_file_modified` |
| `DBaseFileWatcher` | `file_closed` | `&DBaseFileWatcher` | typed signal connection | `connectors::d_base_file_watcher_file_closed` |
| `DBlurEffectWidget` | `radius_changed` | `&DBlurEffectWidget` | typed signal connection | `connectors::d_blur_effect_widget_radius_changed` |
| `DBlurEffectWidget` | `mode_changed` | `&DBlurEffectWidget` | typed signal connection | `connectors::d_blur_effect_widget_mode_changed` |
| `DBlurEffectWidget` | `blend_mode_changed` | `&DBlurEffectWidget` | typed signal connection | `connectors::d_blur_effect_widget_blend_mode_changed` |
| `DBlurEffectWidget` | `blur_rect_x_radius_changed` | `&DBlurEffectWidget` | typed signal connection | `connectors::d_blur_effect_widget_blur_rect_x_radius_changed` |
| `DBlurEffectWidget` | `blur_rect_y_radius_changed` | `&DBlurEffectWidget` | typed signal connection | `connectors::d_blur_effect_widget_blur_rect_y_radius_changed` |
| `DBlurEffectWidget` | `mask_alpha_changed` | `&DBlurEffectWidget` | typed signal connection | `connectors::d_blur_effect_widget_mask_alpha_changed` |
| `DBlurEffectWidget` | `mask_color_changed` | `&DBlurEffectWidget` | typed signal connection | `connectors::d_blur_effect_widget_mask_color_changed` |
| `DBlurEffectWidget` | `full_changed` | `&DBlurEffectWidget` | typed signal connection | `connectors::d_blur_effect_widget_full_changed` |
| `DBlurEffectWidget` | `blur_enabled_changed` | `&DBlurEffectWidget` | typed signal connection | `connectors::d_blur_effect_widget_blur_enabled_changed` |
| `DBlurEffectWidget` | `blur_source_image_dirtied` | `&DBlurEffectWidget` | typed signal connection | `connectors::d_blur_effect_widget_blur_source_image_dirtied` |
| `DBoxWidget` | `size_changed` | `&DBoxWidget` | typed signal connection | `connectors::d_box_widget_size_changed` |
| `DBoxWidget` | `direction_changed` | `&DBoxWidget` | typed signal connection | `connectors::d_box_widget_direction_changed` |
| `DCircleProgress` | `clicked` | `&DCircleProgress` | typed signal connection | `connectors::d_circle_progress_clicked` |
| `DCircleProgress` | `mouse_entered` | `&DCircleProgress` | typed signal connection | `connectors::d_circle_progress_mouse_entered` |
| `DCircleProgress` | `mouse_leaved` | `&DCircleProgress` | typed signal connection | `connectors::d_circle_progress_mouse_leaved` |
| `DCircleProgress` | `value_changed` | `&DCircleProgress` | typed signal connection | `connectors::d_circle_progress_value_changed` |
| `DClipEffectWidget` | `margins_changed` | `&DClipEffectWidget` | typed signal connection | `connectors::d_clip_effect_widget_margins_changed` |
| `DClipEffectWidget` | `clip_path_changed` | `&DClipEffectWidget` | typed signal connection | `connectors::d_clip_effect_widget_clip_path_changed` |
| `DConfig` | `value_changed` | `&DConfig` | typed signal connection | `connectors::d_config_value_changed` |
| `DCrumbEdit` | `crumb_added` | `&DCrumbEdit` | typed signal connection | `connectors::d_crumb_edit_crumb_added` |
| `DCrumbEdit` | `crumb_removed` | `&DCrumbEdit` | typed signal connection | `connectors::d_crumb_edit_crumb_removed` |
| `DCrumbEdit` | `crumb_list_changed` | `&DCrumbEdit` | typed signal connection | `connectors::d_crumb_edit_crumb_list_changed` |
| `DDialog` | `about_to_close` | `&DDialog` | typed signal connection | `connectors::d_dialog_about_to_close` |
| `DDialog` | `closed` | `&DDialog` | typed signal connection | `connectors::d_dialog_closed` |
| `DDialog` | `button_clicked` | `&DDialog` | typed signal connection | `connectors::d_dialog_button_clicked` |
| `DDialog` | `title_changed` | `&DDialog` | typed signal connection | `connectors::d_dialog_title_changed` |
| `DDialog` | `message_changed` | `&DDialog` | typed signal connection | `connectors::d_dialog_message_changed` |
| `DDialog` | `text_format_changed` | `&DDialog` | typed signal connection | `connectors::d_dialog_text_format_changed` |
| `DDialog` | `size_changed` | `&DDialog` | typed signal connection | `connectors::d_dialog_size_changed` |
| `DDialog` | `visible_changed` | `&DDialog` | typed signal connection | `connectors::d_dialog_visible_changed` |
| `DDoubleSpinBox` | `alert_changed` | `&DDoubleSpinBox` | typed signal connection | `connectors::d_double_spin_box_alert_changed` |
| `DDrawer` | `expand_change` | `&DDrawer` | typed signal connection | `connectors::d_drawer_expand_change` |
| `DDrawer` | `size_changed` | `&DDrawer` | typed signal connection | `connectors::d_drawer_size_changed` |
| `DEnhancedWidget` | `x_changed` | `&DEnhancedWidget` | typed signal connection | `connectors::d_enhanced_widget_x_changed` |
| `DEnhancedWidget` | `y_changed` | `&DEnhancedWidget` | typed signal connection | `connectors::d_enhanced_widget_y_changed` |
| `DEnhancedWidget` | `position_changed` | `&DEnhancedWidget` | typed signal connection | `connectors::d_enhanced_widget_position_changed` |
| `DEnhancedWidget` | `width_changed` | `&DEnhancedWidget` | typed signal connection | `connectors::d_enhanced_widget_width_changed` |
| `DEnhancedWidget` | `height_changed` | `&DEnhancedWidget` | typed signal connection | `connectors::d_enhanced_widget_height_changed` |
| `DEnhancedWidget` | `size_changed` | `&DEnhancedWidget` | typed signal connection | `connectors::d_enhanced_widget_size_changed` |
| `DEnhancedWidget` | `enabled_changed` | `&DEnhancedWidget` | typed signal connection | `connectors::d_enhanced_widget_enabled_changed` |
| `DEnhancedWidget` | `showed` | `&DEnhancedWidget` | typed signal connection | `connectors::d_enhanced_widget_showed` |
| `DFileChooserEdit` | `file_choosed` | `&DFileChooserEdit` | typed signal connection | `connectors::d_file_chooser_edit_file_choosed` |
| `DFileChooserEdit` | `dialog_opened` | `&DFileChooserEdit` | typed signal connection | `connectors::d_file_chooser_edit_dialog_opened` |
| `DFileChooserEdit` | `dialog_closed` | `&DFileChooserEdit` | typed signal connection | `connectors::d_file_chooser_edit_dialog_closed` |
| `DFileSystemWatcher` | `file_deleted` | `&DFileSystemWatcher` | typed signal connection | `connectors::d_file_system_watcher_file_deleted` |
| `DFileSystemWatcher` | `file_attribute_changed` | `&DFileSystemWatcher` | typed signal connection | `connectors::d_file_system_watcher_file_attribute_changed` |
| `DFileSystemWatcher` | `file_closed` | `&DFileSystemWatcher` | typed signal connection | `connectors::d_file_system_watcher_file_closed` |
| `DFileSystemWatcher` | `file_moved` | `&DFileSystemWatcher` | typed signal connection | `connectors::d_file_system_watcher_file_moved` |
| `DFileSystemWatcher` | `file_created` | `&DFileSystemWatcher` | typed signal connection | `connectors::d_file_system_watcher_file_created` |
| `DFileSystemWatcher` | `file_modified` | `&DFileSystemWatcher` | typed signal connection | `connectors::d_file_system_watcher_file_modified` |
| `DFileWatcherManager` | `file_deleted` | `&DFileWatcherManager` | typed signal connection | `connectors::d_file_watcher_manager_file_deleted` |
| `DFileWatcherManager` | `file_attribute_changed` | `&DFileWatcherManager` | typed signal connection | `connectors::d_file_watcher_manager_file_attribute_changed` |
| `DFileWatcherManager` | `file_moved` | `&DFileWatcherManager` | typed signal connection | `connectors::d_file_watcher_manager_file_moved` |
| `DFileWatcherManager` | `subfile_created` | `&DFileWatcherManager` | typed signal connection | `connectors::d_file_watcher_manager_subfile_created` |
| `DFileWatcherManager` | `file_modified` | `&DFileWatcherManager` | typed signal connection | `connectors::d_file_watcher_manager_file_modified` |
| `DFileWatcherManager` | `file_closed` | `&DFileWatcherManager` | typed signal connection | `connectors::d_file_watcher_manager_file_closed` |
| `DFloatingMessage` | `close_button_clicked` | `&DFloatingMessage` | typed signal connection | `connectors::d_floating_message_close_button_clicked` |
| `DFloatingMessage` | `message_closed` | `&DFloatingMessage` | typed signal connection | `connectors::d_floating_message_message_closed` |
| `DFlowLayout` | `horizontal_spacing_changed` | `&DFlowLayout` | typed signal connection | `connectors::d_flow_layout_horizontal_spacing_changed` |
| `DFlowLayout` | `vertical_spacing_changed` | `&DFlowLayout` | typed signal connection | `connectors::d_flow_layout_vertical_spacing_changed` |
| `DFlowLayout` | `count_changed` | `&DFlowLayout` | typed signal connection | `connectors::d_flow_layout_count_changed` |
| `DFlowLayout` | `size_hint_changed` | `&DFlowLayout` | typed signal connection | `connectors::d_flow_layout_size_hint_changed` |
| `DFontComboBox` | `current_font_changed` | `&DFontComboBox` | typed signal connection | `connectors::d_font_combo_box_current_font_changed` |
| `DFontManager` | `font_changed` | `&DFontManager` | typed signal connection | `connectors::d_font_manager_font_changed` |
| `DGraphicsClipEffect` | `margins_changed` | `&DGraphicsClipEffect` | typed signal connection | `connectors::d_graphics_clip_effect_margins_changed` |
| `DGraphicsClipEffect` | `clip_path_changed` | `&DGraphicsClipEffect` | typed signal connection | `connectors::d_graphics_clip_effect_clip_path_changed` |
| `DGuiApplicationHelper` | `theme_type_changed` | `&DGuiApplicationHelper` | typed signal connection | `connectors::d_gui_application_helper_theme_type_changed` |
| `DGuiApplicationHelper` | `palette_type_changed` | `&DGuiApplicationHelper` | typed signal connection | `connectors::d_gui_application_helper_palette_type_changed` |
| `DGuiApplicationHelper` | `new_process_instance` | `&DGuiApplicationHelper` | typed signal connection | `connectors::d_gui_application_helper_new_process_instance` |
| `DGuiApplicationHelper` | `font_changed` | `&DGuiApplicationHelper` | typed signal connection | `connectors::d_gui_application_helper_font_changed` |
| `DGuiApplicationHelper` | `application_palette_changed` | `&DGuiApplicationHelper` | typed signal connection | `connectors::d_gui_application_helper_application_palette_changed` |
| `DGuiApplicationHelper` | `size_mode_changed` | `&DGuiApplicationHelper` | typed signal connection | `connectors::d_gui_application_helper_size_mode_changed` |
| `DImageViewer` | `image_changed` | `&DImageViewer` | typed signal connection | `connectors::d_image_viewer_image_changed` |
| `DImageViewer` | `file_name_changed` | `&DImageViewer` | typed signal connection | `connectors::d_image_viewer_file_name_changed` |
| `DImageViewer` | `scale_factor_changed` | `&DImageViewer` | typed signal connection | `connectors::d_image_viewer_scale_factor_changed` |
| `DImageViewer` | `rotate_angle_changed` | `&DImageViewer` | typed signal connection | `connectors::d_image_viewer_rotate_angle_changed` |
| `DImageViewer` | `transform_changed` | `&DImageViewer` | typed signal connection | `connectors::d_image_viewer_transform_changed` |
| `DImageViewer` | `request_previous_image` | `&DImageViewer` | typed signal connection | `connectors::d_image_viewer_request_previous_image` |
| `DImageViewer` | `request_next_image` | `&DImageViewer` | typed signal connection | `connectors::d_image_viewer_request_next_image` |
| `DImageViewer` | `crop_image_changed` | `&DImageViewer` | typed signal connection | `connectors::d_image_viewer_crop_image_changed` |
| `DInputDialog` | `text_value_changed` | `&DInputDialog` | typed signal connection | `connectors::d_input_dialog_text_value_changed` |
| `DInputDialog` | `text_value_selected` | `&DInputDialog` | typed signal connection | `connectors::d_input_dialog_text_value_selected` |
| `DInputDialog` | `int_value_changed` | `&DInputDialog` | typed signal connection | `connectors::d_input_dialog_int_value_changed` |
| `DInputDialog` | `int_value_selected` | `&DInputDialog` | typed signal connection | `connectors::d_input_dialog_int_value_selected` |
| `DInputDialog` | `double_value_changed` | `&DInputDialog` | typed signal connection | `connectors::d_input_dialog_double_value_changed` |
| `DInputDialog` | `double_value_selected` | `&DInputDialog` | typed signal connection | `connectors::d_input_dialog_double_value_selected` |
| `DInputDialog` | `cancel_button_clicked` | `&DInputDialog` | typed signal connection | `connectors::d_input_dialog_cancel_button_clicked` |
| `DInputDialog` | `ok_button_clicked` | `&DInputDialog` | typed signal connection | `connectors::d_input_dialog_ok_button_clicked` |
| `DInputDialog` | `combo_box_current_index_changed` | `&DInputDialog` | typed signal connection | `connectors::d_input_dialog_combo_box_current_index_changed` |
| `DInputDialog` | `text_alert_changed` | `&DInputDialog` | typed signal connection | `connectors::d_input_dialog_text_alert_changed` |
| `DIpv4LineEdit` | `focus_changed` | `&DIpv4LineEdit` | typed signal connection | `connectors::d_ipv4_line_edit_focus_changed` |
| `DKeySequenceEdit` | `editing_finished` | `&DKeySequenceEdit` | typed signal connection | `connectors::d_key_sequence_edit_editing_finished` |
| `DKeySequenceEdit` | `key_sequence_changed` | `&DKeySequenceEdit` | typed signal connection | `connectors::d_key_sequence_edit_key_sequence_changed` |
| `DLineEdit` | `alert_changed` | `&DLineEdit` | typed signal connection | `connectors::d_line_edit_alert_changed` |
| `DLineEdit` | `focus_changed` | `&DLineEdit` | typed signal connection | `connectors::d_line_edit_focus_changed` |
| `DLineEdit` | `text_changed` | `&DLineEdit` | typed signal connection | `connectors::d_line_edit_text_changed` |
| `DLineEdit` | `text_edited` | `&DLineEdit` | typed signal connection | `connectors::d_line_edit_text_edited` |
| `DLineEdit` | `cursor_position_changed` | `&DLineEdit` | typed signal connection | `connectors::d_line_edit_cursor_position_changed` |
| `DLineEdit` | `return_pressed` | `&DLineEdit` | typed signal connection | `connectors::d_line_edit_return_pressed` |
| `DLineEdit` | `editing_finished` | `&DLineEdit` | typed signal connection | `connectors::d_line_edit_editing_finished` |
| `DLineEdit` | `selection_changed` | `&DLineEdit` | typed signal connection | `connectors::d_line_edit_selection_changed` |
| `DListView` | `row_count_changed` | `&DListView` | typed signal connection | `connectors::d_list_view_row_count_changed` |
| `DListView` | `orientation_changed` | `&DListView` | typed signal connection | `connectors::d_list_view_orientation_changed` |
| `DListView` | `trigger_edit` | `&DListView` | typed signal connection | `connectors::d_list_view_trigger_edit` |
| `DLoadingIndicator` | `direction_changed` | `&DLoadingIndicator` | typed signal connection | `connectors::d_loading_indicator_direction_changed` |
| `DLoadingIndicator` | `rotate_changed` | `&DLoadingIndicator` | typed signal connection | `connectors::d_loading_indicator_rotate_changed` |
| `DMPRISControl` | `mpris_acquired` | `&DMPRISControl` | typed signal connection | `connectors::d_m_p_r_i_s_control_mpris_acquired` |
| `DMPRISControl` | `mpris_changed` | `&DMPRISControl` | typed signal connection | `connectors::d_m_p_r_i_s_control_mpris_changed` |
| `DMPRISControl` | `mpris_losted` | `&DMPRISControl` | typed signal connection | `connectors::d_m_p_r_i_s_control_mpris_losted` |
| `DMainWindow` | `window_radius_changed` | `&DMainWindow` | typed signal connection | `connectors::d_main_window_window_radius_changed` |
| `DMainWindow` | `border_width_changed` | `&DMainWindow` | typed signal connection | `connectors::d_main_window_border_width_changed` |
| `DMainWindow` | `border_color_changed` | `&DMainWindow` | typed signal connection | `connectors::d_main_window_border_color_changed` |
| `DMainWindow` | `shadow_radius_changed` | `&DMainWindow` | typed signal connection | `connectors::d_main_window_shadow_radius_changed` |
| `DMainWindow` | `shadow_offset_changed` | `&DMainWindow` | typed signal connection | `connectors::d_main_window_shadow_offset_changed` |
| `DMainWindow` | `shadow_color_changed` | `&DMainWindow` | typed signal connection | `connectors::d_main_window_shadow_color_changed` |
| `DMainWindow` | `clip_path_changed` | `&DMainWindow` | typed signal connection | `connectors::d_main_window_clip_path_changed` |
| `DMainWindow` | `frame_mask_changed` | `&DMainWindow` | typed signal connection | `connectors::d_main_window_frame_mask_changed` |
| `DMainWindow` | `frame_margins_changed` | `&DMainWindow` | typed signal connection | `connectors::d_main_window_frame_margins_changed` |
| `DMainWindow` | `translucent_background_changed` | `&DMainWindow` | typed signal connection | `connectors::d_main_window_translucent_background_changed` |
| `DMainWindow` | `enable_system_resize_changed` | `&DMainWindow` | typed signal connection | `connectors::d_main_window_enable_system_resize_changed` |
| `DMainWindow` | `enable_system_move_changed` | `&DMainWindow` | typed signal connection | `connectors::d_main_window_enable_system_move_changed` |
| `DMainWindow` | `enable_blur_window_changed` | `&DMainWindow` | typed signal connection | `connectors::d_main_window_enable_blur_window_changed` |
| `DMainWindow` | `auto_input_mask_by_clip_path_changed` | `&DMainWindow` | typed signal connection | `connectors::d_main_window_auto_input_mask_by_clip_path_changed` |
| `DMainWindow` | `sidebar_visble_changed` | `&DMainWindow` | typed signal connection | `connectors::d_main_window_sidebar_visble_changed` |
| `DMainWindow` | `sidebar_expaned_changed` | `&DMainWindow` | typed signal connection | `connectors::d_main_window_sidebar_expaned_changed` |
| `DNativeSettings` | `all_keys_changed` | `&DNativeSettings` | typed signal connection | `connectors::d_native_settings_all_keys_changed` |
| `DNativeSettings` | `property_changed` | `&DNativeSettings` | typed signal connection | `connectors::d_native_settings_property_changed` |
| `DPasswordEdit` | `echo_mode_changed` | `&DPasswordEdit` | typed signal connection | `connectors::d_password_edit_echo_mode_changed` |
| `DPictureSequenceView` | `speed_changed` | `&DPictureSequenceView` | typed signal connection | `connectors::d_picture_sequence_view_speed_changed` |
| `DPictureSequenceView` | `play_end` | `&DPictureSequenceView` | typed signal connection | `connectors::d_picture_sequence_view_play_end` |
| `DPlatformHandle` | `frame_margins_changed` | `&DPlatformHandle` | typed signal connection | `connectors::d_platform_handle_frame_margins_changed` |
| `DPlatformHandle` | `window_radius_changed` | `&DPlatformHandle` | typed signal connection | `connectors::d_platform_handle_window_radius_changed` |
| `DPlatformHandle` | `border_width_changed` | `&DPlatformHandle` | typed signal connection | `connectors::d_platform_handle_border_width_changed` |
| `DPlatformHandle` | `border_color_changed` | `&DPlatformHandle` | typed signal connection | `connectors::d_platform_handle_border_color_changed` |
| `DPlatformHandle` | `shadow_radius_changed` | `&DPlatformHandle` | typed signal connection | `connectors::d_platform_handle_shadow_radius_changed` |
| `DPlatformHandle` | `shadow_offset_changed` | `&DPlatformHandle` | typed signal connection | `connectors::d_platform_handle_shadow_offset_changed` |
| `DPlatformHandle` | `shadow_color_changed` | `&DPlatformHandle` | typed signal connection | `connectors::d_platform_handle_shadow_color_changed` |
| `DPlatformHandle` | `window_effect_changed` | `&DPlatformHandle` | typed signal connection | `connectors::d_platform_handle_window_effect_changed` |
| `DPlatformHandle` | `window_start_up_effect_changed` | `&DPlatformHandle` | typed signal connection | `connectors::d_platform_handle_window_start_up_effect_changed` |
| `DPlatformHandle` | `clip_path_changed` | `&DPlatformHandle` | typed signal connection | `connectors::d_platform_handle_clip_path_changed` |
| `DPlatformHandle` | `frame_mask_changed` | `&DPlatformHandle` | typed signal connection | `connectors::d_platform_handle_frame_mask_changed` |
| `DPlatformHandle` | `translucent_background_changed` | `&DPlatformHandle` | typed signal connection | `connectors::d_platform_handle_translucent_background_changed` |
| `DPlatformHandle` | `enable_system_resize_changed` | `&DPlatformHandle` | typed signal connection | `connectors::d_platform_handle_enable_system_resize_changed` |
| `DPlatformHandle` | `enable_system_move_changed` | `&DPlatformHandle` | typed signal connection | `connectors::d_platform_handle_enable_system_move_changed` |
| `DPlatformHandle` | `enable_blur_window_changed` | `&DPlatformHandle` | typed signal connection | `connectors::d_platform_handle_enable_blur_window_changed` |
| `DPlatformHandle` | `auto_input_mask_by_clip_path_changed` | `&DPlatformHandle` | typed signal connection | `connectors::d_platform_handle_auto_input_mask_by_clip_path_changed` |
| `DPlatformTheme` | `cursor_blink_time_changed` | `&DPlatformTheme` | typed signal connection | `connectors::d_platform_theme_cursor_blink_time_changed` |
| `DPlatformTheme` | `cursor_blink_timeout_changed` | `&DPlatformTheme` | typed signal connection | `connectors::d_platform_theme_cursor_blink_timeout_changed` |
| `DPlatformTheme` | `cursor_blink_changed` | `&DPlatformTheme` | typed signal connection | `connectors::d_platform_theme_cursor_blink_changed` |
| `DPlatformTheme` | `double_click_distance_changed` | `&DPlatformTheme` | typed signal connection | `connectors::d_platform_theme_double_click_distance_changed` |
| `DPlatformTheme` | `double_click_time_changed` | `&DPlatformTheme` | typed signal connection | `connectors::d_platform_theme_double_click_time_changed` |
| `DPlatformTheme` | `dnd_drag_threshold_changed` | `&DPlatformTheme` | typed signal connection | `connectors::d_platform_theme_dnd_drag_threshold_changed` |
| `DPlatformTheme` | `theme_name_changed` | `&DPlatformTheme` | typed signal connection | `connectors::d_platform_theme_theme_name_changed` |
| `DPlatformTheme` | `icon_theme_name_changed` | `&DPlatformTheme` | typed signal connection | `connectors::d_platform_theme_icon_theme_name_changed` |
| `DPlatformTheme` | `sound_theme_name_changed` | `&DPlatformTheme` | typed signal connection | `connectors::d_platform_theme_sound_theme_name_changed` |
| `DPlatformTheme` | `font_name_changed` | `&DPlatformTheme` | typed signal connection | `connectors::d_platform_theme_font_name_changed` |
| `DPlatformTheme` | `mono_font_name_changed` | `&DPlatformTheme` | typed signal connection | `connectors::d_platform_theme_mono_font_name_changed` |
| `DPlatformTheme` | `font_point_size_changed` | `&DPlatformTheme` | typed signal connection | `connectors::d_platform_theme_font_point_size_changed` |
| `DPlatformTheme` | `gtk_font_name_changed` | `&DPlatformTheme` | typed signal connection | `connectors::d_platform_theme_gtk_font_name_changed` |
| `DPlatformTheme` | `active_color_changed` | `&DPlatformTheme` | typed signal connection | `connectors::d_platform_theme_active_color_changed` |
| `DPlatformTheme` | `dark_active_color_changed` | `&DPlatformTheme` | typed signal connection | `connectors::d_platform_theme_dark_active_color_changed` |
| `DPlatformTheme` | `palette_changed` | `&DPlatformTheme` | typed signal connection | `connectors::d_platform_theme_palette_changed` |
| `DPlatformTheme` | `dots_per_inch_changed` | `&DPlatformTheme` | typed signal connection | `connectors::d_platform_theme_dots_per_inch_changed` |
| `DPlatformTheme` | `window_radius_changed` | `&DPlatformTheme` | typed signal connection | `connectors::d_platform_theme_window_radius_changed` |
| `DPlatformTheme` | `size_mode_changed` | `&DPlatformTheme` | typed signal connection | `connectors::d_platform_theme_size_mode_changed` |
| `DPlatformTheme` | `scroll_bar_policy_changed` | `&DPlatformTheme` | typed signal connection | `connectors::d_platform_theme_scroll_bar_policy_changed` |
| `DPrintPickColorWidget` | `select_color_button` | `&DPrintPickColorWidget` | typed signal connection | `connectors::d_print_pick_color_widget_select_color_button` |
| `DPrintPickColorWidget` | `signal_color_changed` | `&DPrintPickColorWidget` | typed signal connection | `connectors::d_print_pick_color_widget_signal_color_changed` |
| `DPrintPreviewWidget` | `preview_changed` | `&DPrintPreviewWidget` | typed signal connection | `connectors::d_print_preview_widget_preview_changed` |
| `DPrintPreviewWidget` | `current_page_changed` | `&DPrintPreviewWidget` | typed signal connection | `connectors::d_print_preview_widget_current_page_changed` |
| `DPrintPreviewWidget` | `total_pages` | `&DPrintPreviewWidget` | typed signal connection | `connectors::d_print_preview_widget_total_pages` |
| `DPrintPreviewWidget` | `pages_count_changed` | `&DPrintPreviewWidget` | typed signal connection | `connectors::d_print_preview_widget_pages_count_changed` |
| `DSearchEdit` | `voice_input_finished` | `&DSearchEdit` | typed signal connection | `connectors::d_search_edit_voice_input_finished` |
| `DSearchEdit` | `search_aborted` | `&DSearchEdit` | typed signal connection | `connectors::d_search_edit_search_aborted` |
| `DSearchEdit` | `voice_changed` | `&DSearchEdit` | typed signal connection | `connectors::d_search_edit_voice_changed` |
| `DSettings` | `value_changed` | `&DSettings` | typed signal connection | `connectors::d_settings_value_changed` |
| `DShortcutEditLabel` | `color_setting_change` | `&DShortcutEditLabel` | typed signal connection | `connectors::d_shortcut_edit_label_color_setting_change` |
| `DSimpleListView` | `change_sorting_status` | `&DSimpleListView` | typed signal connection | `connectors::d_simple_list_view_change_sorting_status` |
| `DSlider` | `value_changed` | `&DSlider` | typed signal connection | `connectors::d_slider_value_changed` |
| `DSlider` | `slider_pressed` | `&DSlider` | typed signal connection | `connectors::d_slider_slider_pressed` |
| `DSlider` | `slider_moved` | `&DSlider` | typed signal connection | `connectors::d_slider_slider_moved` |
| `DSlider` | `slider_released` | `&DSlider` | typed signal connection | `connectors::d_slider_slider_released` |
| `DSlider` | `range_changed` | `&DSlider` | typed signal connection | `connectors::d_slider_range_changed` |
| `DSlider` | `action_triggered` | `&DSlider` | typed signal connection | `connectors::d_slider_action_triggered` |
| `DSlider` | `icon_clicked` | `&DSlider` | typed signal connection | `connectors::d_slider_icon_clicked` |
| `DSpinBox` | `alert_changed` | `&DSpinBox` | typed signal connection | `connectors::d_spin_box_alert_changed` |
| `DStackWidget` | `busy_changed` | `&DStackWidget` | typed signal connection | `connectors::d_stack_widget_busy_changed` |
| `DStackWidget` | `depth_changed` | `&DStackWidget` | typed signal connection | `connectors::d_stack_widget_depth_changed` |
| `DStackWidget` | `current_index_changed` | `&DStackWidget` | typed signal connection | `connectors::d_stack_widget_current_index_changed` |
| `DStackWidget` | `switch_widget_finished` | `&DStackWidget` | typed signal connection | `connectors::d_stack_widget_switch_widget_finished` |
| `DSwitchButton` | `checked_changed` | `&DSwitchButton` | typed signal connection | `connectors::d_switch_button_checked_changed` |
| `DSwitchHeaderLine` | `checked_changed` | `&DSwitchHeaderLine` | typed signal connection | `connectors::d_switch_header_line_checked_changed` |
| `DTabBar` | `current_changed` | `&DTabBar` | typed signal connection | `connectors::d_tab_bar_current_changed` |
| `DTabBar` | `tab_close_requested` | `&DTabBar` | typed signal connection | `connectors::d_tab_bar_tab_close_requested` |
| `DTabBar` | `tab_moved` | `&DTabBar` | typed signal connection | `connectors::d_tab_bar_tab_moved` |
| `DTabBar` | `tab_is_inserted` | `&DTabBar` | typed signal connection | `connectors::d_tab_bar_tab_is_inserted` |
| `DTabBar` | `tab_is_removed` | `&DTabBar` | typed signal connection | `connectors::d_tab_bar_tab_is_removed` |
| `DTabBar` | `tab_bar_clicked` | `&DTabBar` | typed signal connection | `connectors::d_tab_bar_tab_bar_clicked` |
| `DTabBar` | `tab_bar_double_clicked` | `&DTabBar` | typed signal connection | `connectors::d_tab_bar_tab_bar_double_clicked` |
| `DTabBar` | `tab_add_requested` | `&DTabBar` | typed signal connection | `connectors::d_tab_bar_tab_add_requested` |
| `DTabBar` | `tab_release_requested` | `&DTabBar` | typed signal connection | `connectors::d_tab_bar_tab_release_requested` |
| `DTabBar` | `drag_action_changed` | `&DTabBar` | typed signal connection | `connectors::d_tab_bar_drag_action_changed` |
| `DTabBar` | `drag_started` | `&DTabBar` | typed signal connection | `connectors::d_tab_bar_drag_started` |
| `DTabBar` | `drag_end` | `&DTabBar` | typed signal connection | `connectors::d_tab_bar_drag_end` |
| `DTickEffect` | `finished` | `&DTickEffect` | typed signal connection | `connectors::d_tick_effect_finished` |
| `DTickEffect` | `state_changed` | `&DTickEffect` | typed signal connection | `connectors::d_tick_effect_state_changed` |
| `DTitleBarToolInterface` | `triggered` | `&DTitleBarToolInterface` | typed signal connection | `connectors::d_title_bar_tool_interface_triggered` |
| `DTitlebar` | `option_clicked` | `&DTitlebar` | typed signal connection | `connectors::d_titlebar_option_clicked` |
| `DTitlebar` | `double_clicked` | `&DTitlebar` | typed signal connection | `connectors::d_titlebar_double_clicked` |
| `DTitlebar` | `mouse_moving` | `&DTitlebar` | typed signal connection | `connectors::d_titlebar_mouse_moving` |
| `DWaterProgress` | `value_changed` | `&DWaterProgress` | typed signal connection | `connectors::d_water_progress_value_changed` |
| `DWindowManagerHelper` | `window_manager_changed` | `&DWindowManagerHelper` | typed signal connection | `connectors::d_window_manager_helper_window_manager_changed` |
| `DWindowManagerHelper` | `has_blur_window_changed` | `&DWindowManagerHelper` | typed signal connection | `connectors::d_window_manager_helper_has_blur_window_changed` |
| `DWindowManagerHelper` | `has_composite_changed` | `&DWindowManagerHelper` | typed signal connection | `connectors::d_window_manager_helper_has_composite_changed` |
| `DWindowManagerHelper` | `has_no_titlebar_changed` | `&DWindowManagerHelper` | typed signal connection | `connectors::d_window_manager_helper_has_no_titlebar_changed` |
| `DWindowManagerHelper` | `has_wallpaper_effect_changed` | `&DWindowManagerHelper` | typed signal connection | `connectors::d_window_manager_helper_has_wallpaper_effect_changed` |
| `DWindowManagerHelper` | `window_list_changed` | `&DWindowManagerHelper` | typed signal connection | `connectors::d_window_manager_helper_window_list_changed` |
| `DWindowManagerHelper` | `window_motif_w_m_hints_changed` | `&DWindowManagerHelper` | typed signal connection | `connectors::d_window_manager_helper_window_motif_w_m_hints_changed` |
| `DWindowMaxButton` | `maximized_changed` | `&DWindowMaxButton` | typed signal connection | `connectors::d_window_max_button_maximized_changed` |
| `QAbstractButton` | `pressed` | `&QAbstractButton` | typed signal connection | `connectors::q_abstract_button_pressed` |
| `QAbstractButton` | `released` | `&QAbstractButton` | typed signal connection | `connectors::q_abstract_button_released` |
| `QAbstractButton` | `clicked` | `&QAbstractButton` | typed signal connection | `connectors::q_abstract_button_clicked` |
| `QAbstractButton` | `toggled` | `&QAbstractButton` | typed signal connection | `connectors::q_abstract_button_toggled` |
| `QAbstractButton` | `is_checked` | `&QAbstractButton` | const method | `connectors::q_abstract_button_is_checked` |
| `QAbstractItemDelegate` | `size_hint_changed` | `&QAbstractItemDelegate` | typed signal connection | `connectors::q_abstract_item_delegate_size_hint_changed` |
| `QAbstractItemModel` | `data_changed` | `&QAbstractItemModel` | typed signal connection | `connectors::q_abstract_item_model_data_changed` |
| `QAbstractItemModel` | `header_data_changed` | `&QAbstractItemModel` | typed signal connection | `connectors::q_abstract_item_model_header_data_changed` |
| `QAbstractItemModel` | `rows_about_to_be_inserted` | `&QAbstractItemModel` | typed signal connection | `connectors::q_abstract_item_model_rows_about_to_be_inserted` |
| `QAbstractItemModel` | `rows_inserted` | `&QAbstractItemModel` | typed signal connection | `connectors::q_abstract_item_model_rows_inserted` |
| `QAbstractItemModel` | `rows_about_to_be_removed` | `&QAbstractItemModel` | typed signal connection | `connectors::q_abstract_item_model_rows_about_to_be_removed` |
| `QAbstractItemModel` | `rows_removed` | `&QAbstractItemModel` | typed signal connection | `connectors::q_abstract_item_model_rows_removed` |
| `QAbstractItemModel` | `columns_about_to_be_inserted` | `&QAbstractItemModel` | typed signal connection | `connectors::q_abstract_item_model_columns_about_to_be_inserted` |
| `QAbstractItemModel` | `columns_inserted` | `&QAbstractItemModel` | typed signal connection | `connectors::q_abstract_item_model_columns_inserted` |
| `QAbstractItemModel` | `columns_about_to_be_removed` | `&QAbstractItemModel` | typed signal connection | `connectors::q_abstract_item_model_columns_about_to_be_removed` |
| `QAbstractItemModel` | `columns_removed` | `&QAbstractItemModel` | typed signal connection | `connectors::q_abstract_item_model_columns_removed` |
| `QAbstractItemModel` | `model_about_to_be_reset` | `&QAbstractItemModel` | typed signal connection | `connectors::q_abstract_item_model_model_about_to_be_reset` |
| `QAbstractItemModel` | `model_reset` | `&QAbstractItemModel` | typed signal connection | `connectors::q_abstract_item_model_model_reset` |
| `QAbstractItemModel` | `rows_about_to_be_moved` | `&QAbstractItemModel` | typed signal connection | `connectors::q_abstract_item_model_rows_about_to_be_moved` |
| `QAbstractItemModel` | `rows_moved` | `&QAbstractItemModel` | typed signal connection | `connectors::q_abstract_item_model_rows_moved` |
| `QAbstractItemModel` | `columns_about_to_be_moved` | `&QAbstractItemModel` | typed signal connection | `connectors::q_abstract_item_model_columns_about_to_be_moved` |
| `QAbstractItemModel` | `columns_moved` | `&QAbstractItemModel` | typed signal connection | `connectors::q_abstract_item_model_columns_moved` |
| `QAbstractItemView` | `pressed` | `&QAbstractItemView` | typed signal connection | `connectors::q_abstract_item_view_pressed` |
| `QAbstractItemView` | `clicked` | `&QAbstractItemView` | typed signal connection | `connectors::q_abstract_item_view_clicked` |
| `QAbstractItemView` | `double_clicked` | `&QAbstractItemView` | typed signal connection | `connectors::q_abstract_item_view_double_clicked` |
| `QAbstractItemView` | `activated` | `&QAbstractItemView` | typed signal connection | `connectors::q_abstract_item_view_activated` |
| `QAbstractItemView` | `entered` | `&QAbstractItemView` | typed signal connection | `connectors::q_abstract_item_view_entered` |
| `QAbstractItemView` | `viewport_entered` | `&QAbstractItemView` | typed signal connection | `connectors::q_abstract_item_view_viewport_entered` |
| `QAbstractItemView` | `icon_size_changed` | `&QAbstractItemView` | typed signal connection | `connectors::q_abstract_item_view_icon_size_changed` |
| `QAbstractSlider` | `value_changed` | `&QAbstractSlider` | typed signal connection | `connectors::q_abstract_slider_value_changed` |
| `QAbstractSlider` | `slider_pressed` | `&QAbstractSlider` | typed signal connection | `connectors::q_abstract_slider_slider_pressed` |
| `QAbstractSlider` | `slider_moved` | `&QAbstractSlider` | typed signal connection | `connectors::q_abstract_slider_slider_moved` |
| `QAbstractSlider` | `slider_released` | `&QAbstractSlider` | typed signal connection | `connectors::q_abstract_slider_slider_released` |
| `QAbstractSlider` | `range_changed` | `&QAbstractSlider` | typed signal connection | `connectors::q_abstract_slider_range_changed` |
| `QAbstractSlider` | `action_triggered` | `&QAbstractSlider` | typed signal connection | `connectors::q_abstract_slider_action_triggered` |
| `QAbstractSpinBox` | `editing_finished` | `&QAbstractSpinBox` | typed signal connection | `connectors::q_abstract_spin_box_editing_finished` |
| `QAction` | `changed` | `&QAction` | typed signal connection | `connectors::q_action_changed` |
| `QAction` | `enabled_changed` | `&QAction` | typed signal connection | `connectors::q_action_enabled_changed` |
| `QAction` | `checkable_changed` | `&QAction` | typed signal connection | `connectors::q_action_checkable_changed` |
| `QAction` | `visible_changed` | `&QAction` | typed signal connection | `connectors::q_action_visible_changed` |
| `QAction` | `triggered` | `&QAction` | typed signal connection | `connectors::q_action_triggered` |
| `QAction` | `hovered` | `&QAction` | typed signal connection | `connectors::q_action_hovered` |
| `QAction` | `toggled` | `&QAction` | typed signal connection | `connectors::q_action_toggled` |
| `QComboBox` | `edit_text_changed` | `&QComboBox` | typed signal connection | `connectors::q_combo_box_edit_text_changed` |
| `QComboBox` | `activated` | `&QComboBox` | typed signal connection | `connectors::q_combo_box_activated` |
| `QComboBox` | `text_activated` | `&QComboBox` | typed signal connection | `connectors::q_combo_box_text_activated` |
| `QComboBox` | `highlighted` | `&QComboBox` | typed signal connection | `connectors::q_combo_box_highlighted` |
| `QComboBox` | `text_highlighted` | `&QComboBox` | typed signal connection | `connectors::q_combo_box_text_highlighted` |
| `QComboBox` | `current_index_changed` | `&QComboBox` | typed signal connection | `connectors::q_combo_box_current_index_changed` |
| `QComboBox` | `current_text_changed` | `&QComboBox` | typed signal connection | `connectors::q_combo_box_current_text_changed` |
| `QDialog` | `finished` | `&QDialog` | typed signal connection | `connectors::q_dialog_finished` |
| `QDialog` | `accepted` | `&QDialog` | typed signal connection | `connectors::q_dialog_accepted` |
| `QDialog` | `rejected` | `&QDialog` | typed signal connection | `connectors::q_dialog_rejected` |
| `QDoubleSpinBox` | `value_changed` | `&QDoubleSpinBox` | typed signal connection | `connectors::q_double_spin_box_value_changed` |
| `QDoubleSpinBox` | `text_changed` | `&QDoubleSpinBox` | typed signal connection | `connectors::q_double_spin_box_text_changed` |
| `QFileDialog` | `file_selected` | `&QFileDialog` | typed signal connection | `connectors::q_file_dialog_file_selected` |
| `QFileDialog` | `files_selected` | `&QFileDialog` | typed signal connection | `connectors::q_file_dialog_files_selected` |
| `QFileDialog` | `current_changed` | `&QFileDialog` | typed signal connection | `connectors::q_file_dialog_current_changed` |
| `QFileDialog` | `directory_entered` | `&QFileDialog` | typed signal connection | `connectors::q_file_dialog_directory_entered` |
| `QFileDialog` | `url_selected` | `&QFileDialog` | typed signal connection | `connectors::q_file_dialog_url_selected` |
| `QFileDialog` | `urls_selected` | `&QFileDialog` | typed signal connection | `connectors::q_file_dialog_urls_selected` |
| `QFileDialog` | `current_url_changed` | `&QFileDialog` | typed signal connection | `connectors::q_file_dialog_current_url_changed` |
| `QFileDialog` | `directory_url_entered` | `&QFileDialog` | typed signal connection | `connectors::q_file_dialog_directory_url_entered` |
| `QFileDialog` | `filter_selected` | `&QFileDialog` | typed signal connection | `connectors::q_file_dialog_filter_selected` |
| `QGraphicsEffect` | `enabled_changed` | `&QGraphicsEffect` | typed signal connection | `connectors::q_graphics_effect_enabled_changed` |
| `QGraphicsView` | `rubber_band_changed` | `&QGraphicsView` | typed signal connection | `connectors::q_graphics_view_rubber_band_changed` |
| `QLabel` | `link_activated` | `&QLabel` | typed signal connection | `connectors::q_label_link_activated` |
| `QLabel` | `link_hovered` | `&QLabel` | typed signal connection | `connectors::q_label_link_hovered` |
| `QLineEdit` | `text_changed` | `&QLineEdit` | typed signal connection | `connectors::q_line_edit_text_changed` |
| `QLineEdit` | `text_edited` | `&QLineEdit` | typed signal connection | `connectors::q_line_edit_text_edited` |
| `QLineEdit` | `cursor_position_changed` | `&QLineEdit` | typed signal connection | `connectors::q_line_edit_cursor_position_changed` |
| `QLineEdit` | `return_pressed` | `&QLineEdit` | typed signal connection | `connectors::q_line_edit_return_pressed` |
| `QLineEdit` | `editing_finished` | `&QLineEdit` | typed signal connection | `connectors::q_line_edit_editing_finished` |
| `QLineEdit` | `selection_changed` | `&QLineEdit` | typed signal connection | `connectors::q_line_edit_selection_changed` |
| `QLineEdit` | `input_rejected` | `&QLineEdit` | typed signal connection | `connectors::q_line_edit_input_rejected` |
| `QMainWindow` | `icon_size_changed` | `&QMainWindow` | typed signal connection | `connectors::q_main_window_icon_size_changed` |
| `QMainWindow` | `tool_button_style_changed` | `&QMainWindow` | typed signal connection | `connectors::q_main_window_tool_button_style_changed` |
| `QMenu` | `about_to_show` | `&QMenu` | typed signal connection | `connectors::q_menu_about_to_show` |
| `QMenu` | `about_to_hide` | `&QMenu` | typed signal connection | `connectors::q_menu_about_to_hide` |
| `QProgressBar` | `value_changed` | `&QProgressBar` | typed signal connection | `connectors::q_progress_bar_value_changed` |
| `QSpinBox` | `value_changed` | `&QSpinBox` | typed signal connection | `connectors::q_spin_box_value_changed` |
| `QSpinBox` | `text_changed` | `&QSpinBox` | typed signal connection | `connectors::q_spin_box_text_changed` |
| `QTableWidget` | `item_selection_changed` | `&QTableWidget` | typed signal connection | `connectors::q_table_widget_item_selection_changed` |
| `QTableWidget` | `cell_pressed` | `&QTableWidget` | typed signal connection | `connectors::q_table_widget_cell_pressed` |
| `QTableWidget` | `cell_clicked` | `&QTableWidget` | typed signal connection | `connectors::q_table_widget_cell_clicked` |
| `QTableWidget` | `cell_double_clicked` | `&QTableWidget` | typed signal connection | `connectors::q_table_widget_cell_double_clicked` |
| `QTableWidget` | `cell_activated` | `&QTableWidget` | typed signal connection | `connectors::q_table_widget_cell_activated` |
| `QTableWidget` | `cell_entered` | `&QTableWidget` | typed signal connection | `connectors::q_table_widget_cell_entered` |
| `QTableWidget` | `cell_changed` | `&QTableWidget` | typed signal connection | `connectors::q_table_widget_cell_changed` |
| `QTableWidget` | `current_cell_changed` | `&QTableWidget` | typed signal connection | `connectors::q_table_widget_current_cell_changed` |
| `QTextEdit` | `text_changed` | `&QTextEdit` | typed signal connection | `connectors::q_text_edit_text_changed` |
| `QTextEdit` | `undo_available` | `&QTextEdit` | typed signal connection | `connectors::q_text_edit_undo_available` |
| `QTextEdit` | `redo_available` | `&QTextEdit` | typed signal connection | `connectors::q_text_edit_redo_available` |
| `QTextEdit` | `copy_available` | `&QTextEdit` | typed signal connection | `connectors::q_text_edit_copy_available` |
| `QTextEdit` | `selection_changed` | `&QTextEdit` | typed signal connection | `connectors::q_text_edit_selection_changed` |
| `QTextEdit` | `cursor_position_changed` | `&QTextEdit` | typed signal connection | `connectors::q_text_edit_cursor_position_changed` |
| `QTimer` | `timeout` | `&QTimer` | typed signal connection | `connectors::q_timer_timeout` |
| `QWidget` | `window_title_changed` | `&QWidget` | typed signal connection | `connectors::q_widget_window_title_changed` |
| `QWidget` | `window_icon_changed` | `&QWidget` | typed signal connection | `connectors::q_widget_window_icon_changed` |
| `QWidget` | `window_icon_text_changed` | `&QWidget` | typed signal connection | `connectors::q_widget_window_icon_text_changed` |
| `QWidget` | `custom_context_menu_requested` | `&QWidget` | typed signal connection | `connectors::q_widget_custom_context_menu_requested` |

CXX-Qt imported items audited: **349**.
