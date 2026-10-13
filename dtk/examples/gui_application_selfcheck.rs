//! Deterministic GUI shutdown. QT_QPA_PLATFORM=offscreen cargo run --example gui_application_selfcheck
//! Native QPointer witnesses distinguish actual deletion from scope invalidation.
use dtk::core::DConfig;
use dtk::gui::{DGuiApplicationHelper, FontSizeType};
use dtk::widgets::{
    DFileIconProvider, DFontSizeManager, DMessageManager, DPaletteHelper, DStyleOptionButton,
    DWaterMarkHelper, WaterMarkData,
};
use dtk::*;
use dtk_sys::{ObjectGuard, ffi};
use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
#[path = "support/expected_panic.rs"]
mod expected_panic;
use expected_panic::expect_panic;

type Captures = Rc<RefCell<Vec<Weak<()>>>>;

fn capture(captures: &Captures) -> Rc<()> {
    let value = Rc::new(());
    captures.borrow_mut().push(Rc::downgrade(&value));
    value
}

fn witness(object: &impl Signal0) -> ObjectGuard {
    // The safe view verifies its owning thread, scope and native base pointer.
    unsafe { ObjectGuard::from_object(object.qobject_ptr()) }
}

fn assert_clean() {
    assert!(
        unsafe { ffi::application_runtime_workers_stopped() },
        "native DTK event/notify frames must be joined before app deletion"
    );
    assert!(
        unsafe { ffi::application_can_construct() },
        "native singleton reservation released"
    );
    assert!(
        dtk_sys::callback_registry_is_empty(),
        "all Rust callback/dispatch/relay records released"
    );
    assert_eq!(
        unsafe { ffi::relay_live_count() },
        0,
        "no native relays, including deferred/disconnected ones"
    );
}

struct ReenterOnDrop {
    checked: Rc<Cell<bool>>,
    timer: QTimer,
    native_timer: ObjectGuard,
    font: dtk::gui::DFontManager,
}

impl Drop for ReenterOnDrop {
    fn drop(&mut self) {
        assert!(
            self.native_timer.native_is_alive(),
            "closing must precede native deletion"
        );
        assert!(!self.timer.is_alive() && !self.font.is_alive());
        // Independent values do not enter the application/connection runtime.
        assert_eq!(
            QVariant::from_string("cleanup value").to_string(),
            "cleanup value"
        );
        assert_eq!(QColor::rgb(1, 2, 3).rgba_u32(), 0xff010203);
        let image = QImage::new(1, 1);
        assert_eq!(image.width(), 1);
        drop(QFont::new());
        expect_panic("create DApplication", || QWidget::new(None));
        expect_panic("create DApplication", DMessageManager::instance);
        expect_panic("create DApplication", DGuiApplicationHelper::instance);
        expect_panic("create DApplication", DFontSizeManager::instance);
        expect_panic("Qt application is closing", DStyleOptionButton::new);
        expect_panic("create DApplication", || {
            DApplication::set_application_display_name("closing")
        });
        expect_panic("create DApplication", || DApplication::has_arg("--smoke"));
        expect_panic("create DApplication", DApplication::quit);
        expect_panic("owning Qt thread", || QTimer::single_shot(0, || {}));
        expect_panic("create QCoreApplication", || {
            DConfig::create_generic("missing", "")
        });
        expect_panic("only one Qt application", || {
            DApplication::with("reenter-gui", |_| {})
        });
        expect_panic("only one Qt application", || {
            DApplication::with_quit_guard("reenter-guard", || true, |_| {})
        });
        expect_panic("only one Qt application", || {
            QCoreApplication::with("reenter-core", |_| {})
        });
        expect_panic("Qt application is closing", || self.timer.stop());
        expect_panic("Qt application is closing", || {
            self.font.font_pixel_size(FontSizeType::T6)
        });
        assert!(!unregister_callback(usize::MAX)); // unregister reentry is harmless
        self.checked.set(true);
    }
}

struct PanicOnDrop(Rc<Cell<bool>>);
impl Drop for PanicOnDrop {
    fn drop(&mut self) {
        self.0.set(true);
        panic!("expected capture Drop panic");
    }
}

fn main() {
    assert_clean();
    std::thread::spawn(|| {
        expect_panic("process main thread", || {
            DApplication::with("worker", |_| {})
        });
        let marker = Rc::new(());
        let weak = Rc::downgrade(&marker);
        expect_panic("process main thread", || {
            DApplication::with_quit_guard(
                "worker-guard",
                move || {
                    let _ = &marker;
                    true
                },
                |_| {},
            )
        });
        assert!(weak.upgrade().is_none());
        assert!(dtk_sys::callback_registry_is_empty());
    })
    .join()
    .unwrap();

    let captures: Captures = Rc::new(RefCell::new(Vec::new()));
    let teardown_checked = Rc::new(Cell::new(false));
    let in_callback = Rc::new(Cell::new(false));
    let callback_returned = Rc::new(Cell::new(false));
    let quit_calls = Rc::new(Cell::new(0));
    let destroyed_dispatches = Rc::new(Cell::new(0));
    let witnesses = Rc::new(RefCell::new(Vec::new()));
    let singleton_witnesses = RefCell::new(Vec::new());
    let guard_marker = capture(&captures);
    let guard_calls = quit_calls.clone();
    let (window, taken, timer, helper, font, message, palette, watermark, sender, ids) =
        DApplication::with_quit_guard(
            "scoped-gui",
            move || {
                let _ = &guard_marker;
                guard_calls.set(guard_calls.get() + 1);
                guard_calls.get() > 1 // first quit is vetoed, the second succeeds
            },
            |app| {
                DApplication::set_quit_on_last_window_closed(false);
                assert!(!unsafe { ffi::application_runtime_workers_stopped() });
                expect_panic("only one Qt application", || {
                    DApplication::with("nested", |_| {})
                });
                expect_panic("only one Qt application", || {
                    QCoreApplication::with("nested-core", |_| {})
                });
                let marker = capture(&captures);
                expect_panic("only one Qt application", || {
                    DApplication::with_quit_guard(
                        "nested-guard",
                        move || {
                            let _ = &marker;
                            true
                        },
                        |_| {},
                    )
                });
                witnesses.borrow_mut().push(witness(app));
                let show_marker = capture(&captures);
                let close_marker = capture(&captures);
                let allow_close = Rc::new(Cell::new(true));
                let close_allowed = allow_close.clone();
                let window = DMainWindow::new_with_events(
                    move || {
                        let _ = &show_marker;
                    },
                    move || {
                        let _ = &close_marker;
                        close_allowed.get()
                    },
                );
                let central = QWidget::new(None);
                let child = DPushButton::new("child");
                child.as_widget().set_parent(&central);
                window.set_central_widget(&central);
                let taken = window.take_central_widget().unwrap();
                assert!(central.was_parented() && !taken.was_parented());
                drop(central); // original owner cannot delete the transferred allocation
                assert!(taken.is_alive() && child.is_alive());
                for object in [&window.as_widget(), &taken, &child.as_widget()] {
                    witnesses.borrow_mut().push(witness(object));
                }
                witnesses.borrow_mut().push(witness(&window.titlebar()));
                let helper = DGuiApplicationHelper::instance().unwrap();
                let font = helper.font_manager().unwrap();
                let message = DMessageManager::instance().unwrap();
                let palette = DPaletteHelper::instance().unwrap();
                let watermark = DWaterMarkHelper::instance().unwrap();
                singleton_witnesses.borrow_mut().extend([
                    witness(&helper),
                    witness(&font),
                    witness(&message),
                    witness(&palette),
                    witness(&watermark),
                ]);
                let mut ids = Vec::new();
                // All five are borrowed getters, not constructor-owned objects.
                macro_rules! singleton_connection {
                    ($object:expr) => {{
                        let marker = capture(&captures);
                        let dispatches = destroyed_dispatches.clone();
                        let id = $object.connect_signal("destroyed(QObject*)", move || {
                            let _ = &marker;
                            dispatches.set(dispatches.get() + 1);
                        });
                        assert_ne!(id, 0);
                        ids.push(id);
                    }};
                }
                singleton_connection!(helper);
                singleton_connection!(font);
                singleton_connection!(message);
                singleton_connection!(palette);
                singleton_connection!(watermark);
                let timer = QTimer::new(app);
                assert!(timer.was_parented());
                let reenter = ReenterOnDrop {
                    checked: teardown_checked.clone(),
                    timer: timer.clone(),
                    native_timer: witness(&timer),
                    font: font.clone(),
                };
                let marker = capture(&captures);
                ids.push(timer.on_timeout(move || {
                    let _ = (&marker, &reenter);
                }));
                timer.start(60_000);
                witnesses.borrow_mut().push(witness(&timer));
                let config = DConfig::create_generic("dtk-rs-gui-scope-missing-meta", "").unwrap();
                assert!(
                    !config.was_parented(),
                    "tracking never reparents native objects"
                );
                witnesses.borrow_mut().push(witness(&config));
                let marker = capture(&captures);
                let paint = PaintWidget::new(Some(&window.as_widget()), move |_| {
                    let _ = &marker;
                });
                witnesses.borrow_mut().push(witness(&paint));
                let marker = capture(&captures);
                let delegate = PaintDelegate::new(&window, move |_, _, _, _, _, _, _| {
                    let _ = &marker;
                });
                witnesses.borrow_mut().push(witness(&delegate));
                let marker = capture(&captures);
                ids.push(window.as_widget().add_shortcut("Ctrl+L", move || {
                    let _ = &marker;
                }));
                let menu = DMenu::new(&window.as_widget());
                let marker = capture(&captures);
                ids.push(menu.add_action("item", move || {
                    let _ = &marker;
                }));
                witnesses.borrow_mut().push(witness(&menu));
                let marker = capture(&captures);
                QTimer::single_shot(60_000, move || {
                    let _ = &marker;
                });
                let marker = capture(&captures);
                let sender = GuiSender::new(app, move |_: i32| {
                    let _ = &marker;
                })
                .unwrap();
                // An untracked native-created top level must also be destroyed.
                let native_popup = unsafe { ffi::widget_new(std::ptr::null_mut()) };
                witnesses
                    .borrow_mut()
                    .push(unsafe { ObjectGuard::from_widget(native_popup) });
                window.show();
                let active = in_callback.clone();
                let returned = callback_returned.clone();
                QTimer::single_shot(0, move || {
                    active.set(true);
                    DApplication::quit();
                    active.set(false);
                    returned.set(true);
                });
                QTimer::single_shot(20, DApplication::quit);
                assert_eq!(app.exec(), 0);
                assert!(callback_returned.get() && !in_callback.get());
                assert!(window.is_alive() && timer.is_alive()); // exec is not scope exit
                // The borrowed token stays usable; a second exec is allowed after return.
                QTimer::single_shot(0, DApplication::quit);
                assert_eq!(app.exec(), 0);
                // Ordinary Qt Quit respects a close veto. Restore it only now:
                // deterministic outer-scope deletion must not be vetoed.
                allow_close.set(false);
                window.show();
                (
                    window, taken, timer, helper, font, message, palette, watermark, sender, ids,
                )
            },
        );
    assert!(quit_calls.get() >= 3 && teardown_checked.get());
    assert!(
        !singleton_witnesses.borrow()[0].native_is_alive(),
        "DTK helper native post-routine must delete/reset it"
    );
    assert!(
        singleton_witnesses.borrow()[1..]
            .iter()
            .all(ObjectGuard::native_is_alive),
        "font/message/palette/watermark borrows must not be deleted by Rust"
    );
    assert!(
        singleton_witnesses
            .borrow()
            .iter()
            .all(|guard| !guard.is_alive())
    );
    assert_eq!(
        destroyed_dispatches.get(),
        0,
        "shutdown never invokes user destruction callbacks"
    );
    assert!(
        captures
            .borrow()
            .iter()
            .all(|weak| weak.upgrade().is_none())
    );
    assert!(
        witnesses
            .borrow()
            .iter()
            .all(|guard| !guard.native_is_alive()),
        "native objects must actually be deleted, not only guarded as dead"
    );
    assert!(!window.is_alive() && !taken.is_alive() && !timer.is_alive());
    assert!(
        !helper.is_alive()
            && !font.is_alive()
            && !message.is_alive()
            && !palette.is_alive()
            && !watermark.is_alive()
    );
    assert!(ids.into_iter().all(|id| !unregister_callback(id)));
    assert_eq!(
        sender.send(1).unwrap_err().kind(),
        std::io::ErrorKind::BrokenPipe
    );
    expect_panic("DMainWindow was destroyed", || window.width());
    expect_panic("DFontManager was destroyed", || {
        font.font_pixel_size(FontSizeType::T6)
    });
    expect_panic("DGuiApplicationHelper was destroyed", || {
        helper.theme_type()
    });
    expect_panic("DMessageManager was destroyed", || {
        message.connect_signal("destroyed(QObject*)", || {})
    });
    expect_panic("DPaletteHelper was destroyed", || {
        palette.connect_signal("destroyed(QObject*)", || {})
    });
    expect_panic("DWaterMarkHelper was destroyed", || {
        watermark.connect_signal("destroyed(QObject*)", || {})
    });
    expect_panic("create DApplication", || QWidget::new(None));
    assert_clean();

    // No exec: dispose dropped owners, forgotten owners, transfers and relays
    // on process-global singleton senders without any DeferredDelete delivery.
    let captures: Captures = Rc::new(RefCell::new(Vec::new()));
    let mut native_singletons = Vec::new();
    let pending = DApplication::with("no-exec", |app| {
        let owner = QWidget::new(None);
        let old = owner.clone();
        let physical = witness(&owner);
        drop(owner);
        let forgotten = QWidget::new(None);
        let forgotten_physical = witness(&forgotten);
        std::mem::forget(forgotten);
        let message = DMessageManager::instance().unwrap();
        let fresh_font = DGuiApplicationHelper::instance()
            .unwrap()
            .font_manager()
            .unwrap();
        assert!(fresh_font.is_alive() && !font.is_alive() && message.is_alive());
        assert!(!window.is_alive() && !timer.is_alive());
        native_singletons.push(witness(&message));
        native_singletons.push(witness(&fresh_font));
        let marker = capture(&captures);
        let id = message.connect_signal("destroyed(QObject*)", move || {
            let _ = &marker;
        });
        assert!(unregister_callback(id)); // disconnected relay still awaits native deletion
        assert!(unsafe { ffi::relay_live_count() } > 0);
        let marker = capture(&captures);
        QTimer::single_shot(60_000, move || {
            let _ = &marker;
        });
        let timer = QTimer::new(app);
        let marker = capture(&captures);
        timer.on_timeout(move || {
            let _ = &marker;
        });
        // These APIs return borrowed process singletons: never delete them.
        let font_size = DFontSizeManager::instance().unwrap();
        let provider = DFileIconProvider::global_provider().unwrap();
        assert!(font_size.is_alive() && provider.is_alive());
        let independent = WaterMarkData::new();
        independent.set_text("owned value survives app");
        (
            old,
            physical,
            forgotten_physical,
            font_size,
            provider,
            independent,
        )
    });
    assert!(!pending.0.is_alive() && !pending.1.native_is_alive() && !pending.2.native_is_alive());
    assert!(!pending.3.is_alive() && !pending.4.is_alive());
    expect_panic("DFontSizeManager application scope has closed", || {
        pending.3.font_pixel_size(0)
    });
    assert_eq!(pending.5.text(), "owned value survives app");
    assert!(
        native_singletons.iter().all(ObjectGuard::native_is_alive),
        "borrowed native singletons must not be freed"
    );
    assert!(
        captures
            .borrow()
            .iter()
            .all(|weak| weak.upgrade().is_none())
    );
    assert_clean();

    // Capture destructor panics are contained; cleanup still completes.
    let dropped = Rc::new(Cell::new(false));
    let probe = PanicOnDrop(dropped.clone());
    DApplication::with("drop-panic", |_| {
        QTimer::single_shot(60_000, move || {
            let _ = &probe;
        });
    });
    assert!(dropped.get());
    assert_clean();

    // An ordinary panic in the outer closure propagates only after native cleanup.
    let escaping = RefCell::new(None);
    let marker = Rc::new(());
    let weak = Rc::downgrade(&marker);
    expect_panic("scope panic", || {
        DApplication::with("panic", |_| {
            let owner = DMainWindow::new();
            escaping.replace(Some((owner.clone(), witness(&owner))));
            QTimer::single_shot(60_000, move || {
                let _ = &marker;
            });
            panic!("scope panic");
        })
    });
    let (old, physical) = escaping.into_inner().unwrap();
    assert!(!old.is_alive() && !physical.native_is_alive() && weak.upgrade().is_none());
    assert_clean();

    // Both panics together: capture Drop is contained while already unwinding
    // the outer closure; its original panic must still be the propagated one.
    let escaping = RefCell::new(None);
    let dropped = Rc::new(Cell::new(false));
    let probe = PanicOnDrop(dropped.clone());
    expect_panic("original outer panic", || {
        DApplication::with("combined-panic", |_| {
            let widget = QWidget::new(None);
            escaping.replace(Some(witness(&widget)));
            QTimer::single_shot(60_000, move || {
                let _ = &probe;
            });
            panic!("original outer panic");
        });
    });
    assert!(dropped.get() && !escaping.into_inner().unwrap().native_is_alive());
    assert_clean();
    DApplication::with("after-combined-panic", |_| {});
    assert_clean();

    // Dropping an escaped old owner inside a fresh scope must not schedule
    // deletion of a fresh allocation, even when native addresses are reused.
    let old_owner = DApplication::with("old-owner", |_| QWidget::new(None));
    DApplication::with("fresh-owner", |_| {
        let fresh = QWidget::new(None);
        drop(old_owner);
        assert!(fresh.is_alive());
        fresh.resize(10, 10);
    });
    assert_clean();

    // A panic in a timer is contained at FFI: its captures are released while
    // the app is still live, then normal outer-scope cleanup reclaims Qt objects.
    struct CallbackPanicDrop {
        checked: Rc<Cell<bool>>,
        view: QWidget,
    }
    impl Drop for CallbackPanicDrop {
        fn drop(&mut self) {
            assert!(self.view.is_alive());
            self.view.width();
            self.checked.set(true);
        }
    }
    let panic_callback_dropped = Rc::new(Cell::new(false));
    let physical = DApplication::with("callback-panic", |app| {
        let widget = QWidget::new(None);
        let probe = CallbackPanicDrop {
            checked: panic_callback_dropped.clone(),
            view: widget.clone(),
        };
        QTimer::single_shot(0, move || {
            let _ = &probe;
            panic!("expected timer callback panic");
        });
        QTimer::single_shot(20, DApplication::quit);
        assert_eq!(app.exec(), 0);
        assert!(widget.is_alive() && panic_callback_dropped.get());
        witness(&widget)
    });
    assert!(!physical.native_is_alive());
    assert_clean();

    // Repeat startup/shutdown, alternate with headless scopes, and ensure stale
    // owners/views cannot revive or delete fresh native allocations/singletons.
    for index in 0..12 {
        DApplication::with(&format!("repeat-{index}"), |app| {
            assert!(!old.is_alive() && !helper.is_alive() && !font.is_alive());
            assert!(!message.is_alive() && !palette.is_alive() && !watermark.is_alive());
            assert!(
                singleton_witnesses
                    .borrow()
                    .iter()
                    .all(|guard| !guard.is_alive())
            );
            assert!(
                !unsafe { ffi::application_runtime_workers_stopped() },
                "native DTK worker restarts only inside a fresh app"
            );
            assert!(
                unsafe { ffi::application_runtime_worker_roundtrip() },
                "restarted GUI worker executes real tasks"
            );
            let fresh = DGuiApplicationHelper::instance().unwrap();
            assert!(fresh.is_alive() && fresh.font_manager().unwrap().is_alive());
            let fresh_font_size = DFontSizeManager::instance().unwrap();
            assert!(fresh_font_size.is_alive() && fresh_font_size.font_pixel_size(0) > 0);
            assert!(!pending.3.is_alive() && !pending.4.is_alive());
            expect_panic("DFontSizeManager application scope has closed", || {
                pending.3.font_pixel_size(0)
            });
            let root = QWidget::new(None);
            let child = QWidget::new(Some(&root));
            assert!(child.was_parented());
            let physical = witness(&child);
            let marker = capture(&captures);
            child.connect_signal("destroyed(QObject*)", move || {
                let _ = &marker;
            });
            if index % 2 == 0 {
                QTimer::single_shot(0, DApplication::quit);
                assert_eq!(app.exec(), 0);
            }
            // Physical child destruction is checked after this scope, below.
            witnesses.borrow_mut().push(physical);
        });
        assert!(
            witnesses
                .borrow()
                .iter()
                .all(|guard| !guard.native_is_alive())
        );
        assert!(
            captures
                .borrow()
                .iter()
                .all(|weak| weak.upgrade().is_none())
        );
        assert_clean();
        QCoreApplication::with("core-between-gui", |app| {
            assert!(!unsafe { ffi::application_runtime_workers_stopped() });
            assert!(
                unsafe { ffi::application_runtime_worker_roundtrip() },
                "GUI-to-Core worker executes queued work"
            );
            expect_panic("create DApplication", DGuiApplicationHelper::instance);
            QTimer::single_shot(0, QCoreApplication::quit);
            assert_eq!(app.exec(), 0);
        });
        assert_clean();
    }
    // Deterministic native queue proof: a latch holds event A, an actual
    // DConfig update B is accepted, then the scope starts shutdown. A producer
    // releases A only after the FIFO stop marker was accepted, so B cannot be
    // mistaken for work that happens to finish before cleanup. No GUI exec or
    // scheduler-delay assumptions; inspect completion before any next scope.
    for index in 0..6 {
        let probe = if index % 2 == 0 {
            DApplication::with("queued-gui-config", |_| unsafe {
                ffi::config_worker_probe_new()
            })
        } else {
            QCoreApplication::with("queued-core-config", |_| unsafe {
                ffi::config_worker_probe_new()
            })
        };
        assert!(
            unsafe { ffi::config_worker_probe_completed(probe.as_ref().unwrap()) },
            "accepted native DConfig update completed before with returned"
        );
        assert_clean();
        drop(probe);
    }
    let queued_panic = RefCell::new(None);
    expect_panic("queued scope panic", || {
        DApplication::with("queued-panic", |_| {
            queued_panic.replace(Some(unsafe { ffi::config_worker_probe_new() }));
            panic!("queued scope panic");
        });
    });
    let probe = queued_panic.into_inner().unwrap();
    assert!(unsafe { ffi::config_worker_probe_completed(probe.as_ref().unwrap()) });
    assert_clean();
    drop(probe);
    drop((window, taken, timer)); // escaped owners never access freed Qt objects
    assert_clean();
    println!(
        "gui application selfcheck ok: normal/no-exec/panic, singleton/parent/transfer, callbacks/reentry, 12 repeated GUI/Core scopes"
    );
}
