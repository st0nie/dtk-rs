//! Headless QtCore scope cleanup: no DISPLAY, Qt platform plugin or DApplication.
//! env -u DISPLAY -u WAYLAND_DISPLAY -u QT_QPA_PLATFORM cargo run --example core_application_selfcheck
use dtk::core::DConfig;
use dtk::gui::DGuiApplicationHelper;
use dtk::{
    DApplication, GuiSender, QCoreApplication, QPixmap, QTimer, QVariant, QWidget, QtLifecycle,
};
#[path = "support/expected_panic.rs"]
mod expected_panic;
use expected_panic::expect_panic;
use std::cell::Cell;
use std::rc::Rc;

struct ReenterOnDrop {
    rejected: Rc<Cell<bool>>,
    timer: QTimer,
}

impl Drop for ReenterOnDrop {
    fn drop(&mut self) {
        // Teardown has to reject reentry before destroying any native objects,
        // including still-live timer handles, not only once Qt deletes the app.
        expect_panic("owning Qt thread", || QTimer::single_shot(0, || {}));
        expect_panic("only one Qt application", || {
            QCoreApplication::with("reenter", |_| {})
        });
        expect_panic("QTimer was destroyed", || self.timer.stop());
        self.rejected.set(true);
    }
}

fn missing_config() -> dtk::Owned<DConfig> {
    DConfig::create_generic("dtk-rs-nonexistent-core-meta", "")
        .expect("native DConfig factory allocates an object even for absent metadata")
}

fn main() {
    std::thread::spawn(|| {
        expect_panic("process main thread", || {
            QCoreApplication::with("wrong-thread", |_| {})
        });
        expect_panic("process main thread", || {
            DApplication::with("wrong-thread", |_| {})
        });
    })
    .join()
    .expect("wrong-thread application creation is rejected");
    expect_panic("create QCoreApplication", missing_config);

    let retained = Rc::new(());
    let retained_weak = Rc::downgrade(&retained);
    let cleanup_rejected = Rc::new(Cell::new(false));
    let teardown_observed = cleanup_rejected.clone();
    let (config, timer, specific, value, callback_id) =
        QCoreApplication::with("headless-config", |app| {
            expect_panic("only one Qt application", || {
                QCoreApplication::with("nested", |_| {})
            });
            expect_panic("only one Qt application", || {
                DApplication::with("nested-gui", |_| {})
            });
            expect_panic("create DApplication", || QWidget::new(None));
            expect_panic("create DApplication", || QPixmap::new(""));
            expect_panic("create DApplication", DGuiApplicationHelper::instance);
            std::thread::spawn(|| {
                expect_panic("owning Qt thread", missing_config);
                expect_panic("owning Qt thread", || QTimer::single_shot(0, || {}));
            })
            .join()
            .unwrap();

            let config = missing_config();
            let specific = DConfig::create_for_app(
                "org.dtk.rs.headless-selfcheck",
                "dtk-rs-nonexistent-core-meta",
                "",
            )
            .expect("native app-specific DConfig factory");
            assert!(!config.is_valid() && !specific.is_valid());
            assert_eq!(
                config
                    .value("missing", &QVariant::from_string("fallback"))
                    .to_string(),
                "fallback"
            );
            assert!(
                !config.was_parented(),
                "scope registry must not alter native QObject parents"
            );
            let timer = QTimer::new(app);
            assert!(
                timer.was_parented(),
                "QTimer retains its explicitly requested parent"
            );
            let reenter = ReenterOnDrop {
                rejected: teardown_observed,
                timer: timer.clone(),
            };
            let callback_id = timer.on_timeout(move || {
                let _ = (&retained, &reenter);
            });
            assert_ne!(callback_id, 0);
            timer.start(60_000); // never fires; must release its Rust captures at scope end
            QTimer::single_shot(0, QCoreApplication::quit);
            assert_eq!(app.exec(), 0);
            assert!(config.is_alive() && timer.is_alive()); // exec exit alone is not scope exit
            (
                config,
                timer,
                specific,
                QVariant::from_string("value survives runtime"),
                callback_id,
            )
        });
    assert!(
        retained_weak.upgrade().is_none(),
        "pending timer callback must be freed"
    );
    assert!(
        cleanup_rejected.get(),
        "reentry from capture Drop must be safely rejected"
    );
    assert!(
        !config.is_alive() && !timer.is_alive() && !specific.is_alive(),
        "exported handles must be invalidated"
    );
    expect_panic("DConfig was destroyed", || config.name());
    expect_panic("QTimer was destroyed", || timer.stop());
    expect_panic("create QCoreApplication", QCoreApplication::quit);
    assert!(
        !dtk::unregister_callback(callback_id),
        "teardown must remove relay/callback ids"
    );
    assert_eq!(value.to_string(), "value survives runtime");
    drop((config, timer, specific)); // old owners must not touch freed native memory

    // QtCore socket notifications also deliver worker messages without GUI
    // initialization, and scope exit drops the receiver even with a live sender.
    let message = Rc::new(Cell::new(0));
    let observed = message.clone();
    let sender = QCoreApplication::with("headless-worker", |app| {
        let sender = GuiSender::new(app, move |value: i32| {
            observed.set(value);
            QCoreApplication::quit();
        })
        .expect("create QtCore socket notifier");
        let worker = sender.clone();
        std::thread::spawn(move || worker.send(42).unwrap())
            .join()
            .unwrap();
        assert_eq!(app.exec(), 0);
        sender
    });
    assert_eq!(message.get(), 42);
    assert_eq!(
        sender.send(43).unwrap_err().kind(),
        std::io::ErrorKind::BrokenPipe
    );

    // Cleanup must work without running exec and without a Qt event-loop turn.
    let pending = Rc::new(());
    let weak = Rc::downgrade(&pending);
    let old = QCoreApplication::with("never-executed", |_| {
        QTimer::single_shot(60_000, move || {
            let _ = &pending;
        });
        let object = missing_config();
        let view = object.clone();
        drop(object); // queues deleteLater, but no event loop will run
        view
    });
    assert!(!old.is_alive());
    assert!(weak.upgrade().is_none());

    let pending = Rc::new(());
    let weak = Rc::downgrade(&pending);
    expect_panic("scope panic", || {
        QCoreApplication::with("unwind", |_| {
            let _config = missing_config();
            QTimer::single_shot(60_000, move || {
                let _ = &pending;
            });
            panic!("scope panic");
        });
    });
    assert!(
        weak.upgrade().is_none(),
        "unwinding must free unfired captures"
    );

    // Teardown freed the singleton; repeated scopes are independent, and old
    // QPointers stay dead even if native allocation addresses are reused.
    QCoreApplication::with("fresh-scope", |app| {
        assert!(!old.is_alive());
        for _ in 0..2 {
            QTimer::single_shot(0, QCoreApplication::quit);
            assert_eq!(app.exec(), 0);
        }
    });
    println!("core application selfcheck ok");
}
