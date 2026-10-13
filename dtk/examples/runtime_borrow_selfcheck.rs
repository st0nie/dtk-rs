//! Real Qt/DTK regression for native call, callback and alias lifetime boundaries.
//! QT_QPA_PLATFORM=offscreen cargo run --example runtime_borrow_selfcheck
use dtk::widgets::{DStackWidget, DStyleOptionButton};
use dtk::{
    DApplication, DMainWindow, DMessageBox, DPushButton, PaintDelegate, PaintWidget,
    PaintWidgetEvent, QTableWidget, QTimer, QWidget, SignalI32,
};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[path = "support/expected_panic.rs"]
mod expected_panic;
use expected_panic::expect_panic;

fn argument_conversion_cannot_open_an_aliasing_transaction() {
    // Implementing this trait is unsafe; the implementation below upholds the
    // live QObject contract. Its harmless reentry probes the enclosing safe
    // operation's argument-conversion boundary before FFI has even started.
    struct Argument {
        view: QWidget,
        observed: Rc<Cell<bool>>,
    }
    unsafe impl dtk::Signal0 for Argument {
        fn qobject_ptr(&self) -> *mut dtk_sys::ffi::QObject {
            expect_panic("native state is borrowed", || self.view.resize(1, 1));
            self.observed.set(true);
            dtk::Signal0::qobject_ptr(&self.view)
        }
    }
    DApplication::with("runtime-argument-borrow", |_| {
        let parent = QWidget::new(None);
        let checked = Rc::new(Cell::new(false));
        let argument = Argument {
            view: parent.clone(),
            observed: checked.clone(),
        };
        let timer = QTimer::new(&argument);
        assert!(checked.get() && timer.is_alive());
    });
}

fn fd_readiness_is_coalesced_until_the_callback_returns(typed: bool) {
    use std::io::{Read, Write};
    use std::os::fd::AsRawFd;
    use std::os::unix::net::UnixStream;
    use std::time::Duration;
    DApplication::with("runtime-fd-readiness", |app| {
        DApplication::set_quit_on_last_window_closed(false);
        let (mut receiver, mut producer) = UnixStream::pair().unwrap();
        // A duplicate callback must fail deterministically rather than hang.
        receiver
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();
        let descriptor = receiver.as_raw_fd();
        let notifier = dtk::QSocketNotifier::new(descriptor, app);
        let modal = DMessageBox::new();
        let calls = Rc::new(Cell::new(0));
        let drained = Rc::new(Cell::new(false));
        let completed = Rc::new(Cell::new(false));
        producer.write_all(b"x").unwrap();
        let mut producer = Some(producer);
        let (observed, read_done, paused, seen) = (
            calls.clone(),
            drained.clone(),
            notifier.clone(),
            completed.clone(),
        );
        let handler = move || {
            observed.set(observed.get() + 1);
            assert_eq!(
                observed.get(),
                1,
                "same level-triggered readiness was replayed"
            );
            let close = modal.clone();
            QTimer::single_shot(0, move || close.close());
            // The fd is deliberately NOT drained until this nested loop ends.
            // It remains readable, but must not queue a second callback/read.
            modal.exec();
            let mut byte = [0];
            receiver.read_exact(&mut byte).unwrap();
            assert_eq!(byte, [b'x']);
            read_done.set(true);
            paused.set_enabled(false); // completion must honor explicit disable
            let mut producer = producer.take().unwrap();
            let (observed, read_done, seen) = (observed.clone(), read_done.clone(), seen.clone());
            QTimer::single_shot(0, move || {
                producer.write_all(b"y").unwrap(); // disabled notifier must stay quiet
                QTimer::single_shot(0, move || {
                    seen.set(observed.get() == 1 && read_done.get());
                    DApplication::quit();
                });
            });
        };
        let id = if typed {
            let mut handler = handler;
            notifier.connect_signal_i32("activated(int)", move |fd| {
                assert_eq!(fd, descriptor);
                handler();
            })
        } else {
            notifier.on_activated(handler)
        };
        assert_ne!(id, 0);
        assert_eq!(app.exec(), 0);
        assert!(completed.get());
    });
}

fn copied_signal_and_native_child_deletion() {
    DApplication::with("runtime-signal-borrow", |_| {
        let stack = DStackWidget::new();
        let child = QWidget::new(None);
        let calls = Rc::new(Cell::new(0));
        let (alias, observed) = (stack.clone(), calls.clone());
        stack.connect_signal_i32("currentIndexChanged(int)", move |index| {
            if index == 0 {
                observed.set(observed.get() + 1);
                // Formerly ran inside setCurrentIndex before pushWidget's
                // trailing depth() read. The native call now returns first.
                alias.clear(); // real native qDeleteAll(widgetList)
            }
        });
        assert_eq!(stack.push_widget(&child, false), 0);
        assert_eq!(calls.get(), 1);
        assert!(!child.is_alive());
        assert_eq!(stack.depth(), 0);
        expect_panic("QWidget was destroyed", || child.width());
    });
}

fn native_animation_keeps_no_deleted_widget_borrows() {
    DApplication::with("runtime-animation-borrow", |app| {
        DApplication::set_quit_on_last_window_closed(false);
        let stack = DStackWidget::new();
        stack.set_animation_duration(10_000);
        let first = QWidget::new(None);
        let second = QWidget::new(None);
        assert_eq!(stack.push_widget(&first, false), 0);
        assert_eq!(stack.push_widget(&second, true), 1);
        assert!(stack.busy());
        stack.clear(); // native transition info had raw oldWidget/newWidget
        assert!(!stack.busy() && !first.is_alive() && !second.is_alive());
        let checked = Rc::new(Cell::new(false));
        let seen = checked.clone();
        QTimer::single_shot(25, move || {
            assert_eq!(stack.depth(), 0);
            assert!(!stack.busy());
            seen.set(true);
            DApplication::quit();
        });
        assert_eq!(app.exec(), 0);
        assert!(checked.get());
    });
}

fn show_callback_with_parent_deletion_and_nested_loop() {
    DApplication::with("runtime-show-borrow", |app| {
        DApplication::set_quit_on_last_window_closed(false);
        let modal = DMessageBox::new();
        let slot: Rc<RefCell<Option<DMainWindow>>> = Rc::new(RefCell::new(None));
        let child_slot: Rc<RefCell<Option<QWidget>>> = Rc::new(RefCell::new(None));
        let completed = Rc::new(Cell::new(false));
        let (views, child_views, observed, nested) = (
            slot.clone(),
            child_slot.clone(),
            completed.clone(),
            modal.clone(),
        );
        let window = DMainWindow::new_with_events(
            move || {
                let parent = views.borrow().as_ref().unwrap().clone();
                let child = child_views.borrow().as_ref().unwrap().clone();
                assert!(parent.is_alive() && child.is_alive());
                parent.delete_later();
                let (close, nested_child, nested_parent) =
                    (nested.clone(), child.clone(), parent.clone());
                QTimer::single_shot(0, move || {
                    assert!(
                        nested_child.is_alive(),
                        "active native arguments must not be freed"
                    );
                    // A modal exec leaves a suspended native frame. Arbitrary
                    // access through an alias is rejected, not a second borrow.
                    expect_panic("native state is borrowed", || nested_child.width());
                    expect_panic("only a modal dialog", || nested_parent.close());
                    close.close();
                });
                nested.exec();
                observed.set(true);
            },
            || true,
        );
        let child = QWidget::new(None);
        window.set_central_widget(&child);
        *slot.borrow_mut() = Some(window.clone());
        *child_slot.borrow_mut() = Some(child.clone());
        window.show(); // exact baseline showEvent -> deleteLater -> modal crash
        assert!(
            completed.get(),
            "show callback did not complete after native show"
        );
        let view = window.clone();
        let checked = Rc::new(Cell::new(false));
        let seen = checked.clone();
        QTimer::single_shot(0, move || {
            assert!(!view.is_alive() && !child.is_alive());
            seen.set(true);
            DApplication::quit();
        });
        assert_eq!(app.exec(), 0);
        assert!(checked.get());
    });
}

fn receiver_deletion_during_queued_callback() {
    DApplication::with("runtime-receiver-deletion", |app| {
        DApplication::set_quit_on_last_window_closed(false);
        let button = DPushButton::new("delete sender");
        let dialog = DMessageBox::new();
        let calls = Rc::new(Cell::new(0));
        let (view, nested, observed) = (button.clone(), dialog.clone(), calls.clone());
        button.on_clicked(move || {
            observed.set(observed.get() + 1);
            view.delete_later();
            let close = nested.clone();
            QTimer::single_shot(0, move || close.close());
            nested.exec();
        });
        button.click();
        assert_eq!(calls.get(), 1);
        QTimer::single_shot(0, move || {
            assert!(!button.is_alive());
            DApplication::quit();
        });
        assert_eq!(app.exec(), 0);
    });
}

struct ReenterOnDrop {
    button: DPushButton,
    observed: Rc<Cell<bool>>,
}
impl Drop for ReenterOnDrop {
    fn drop(&mut self) {
        // A capture destructor must not run in QObject's native destructor.
        // It can safely access an independent, still-live QObject at retirement.
        self.button.set_text("retired outside destructor");
        self.observed.set(true);
    }
}
fn native_destruction_retires_captures() {
    DApplication::with("runtime-capture-retirement", |app| {
        DApplication::set_quit_on_last_window_closed(false);
        let parent = QWidget::new(None);
        let child = DPushButton::new("child sender");
        child.as_widget().set_parent(&parent);
        let independent = DPushButton::new("still live");
        let retired = Rc::new(Cell::new(false));
        let capture = ReenterOnDrop {
            button: independent.clone(),
            observed: retired.clone(),
        };
        child.on_clicked(move || {
            let _ = &capture;
        });
        parent.delete_later();
        QTimer::single_shot(0, move || {
            assert!(!child.is_alive());
            assert!(
                retired.get(),
                "capture Drop was not retired to a safe boundary"
            );
            assert!(independent.is_alive());
            DApplication::quit();
        });
        assert_eq!(app.exec(), 0);
    });
}

fn paint_and_close_return_borrows() {
    DApplication::with("runtime-synchronous-borrows", |app| {
        DApplication::set_quit_on_last_window_closed(false);
        let closed = Rc::new(Cell::new(false));
        let painted = Rc::new(Cell::new(false));
        let delegated = Rc::new(Cell::new(false));
        let slot: Rc<RefCell<Option<QWidget>>> = Rc::new(RefCell::new(None));
        let (views, seen) = (slot.clone(), closed.clone());
        let window = DMainWindow::new_with_events(
            || {},
            move || {
                let view = views.borrow().as_ref().unwrap().clone();
                expect_panic("native state is borrowed", || view.resize(1, 1));
                expect_panic("native state is borrowed", DMessageBox::new);
                seen.set(true);
                false // actual native return-value veto, not a queued notification
            },
        );
        *slot.borrow_mut() = Some(window.as_widget());
        window.show();
        window.close();
        assert!(closed.get() && window.is_visible());

        let (parent, paint_seen) = (window.clone(), painted.clone());
        let mut value = Some(DStyleOptionButton::new());
        let paint = PaintWidget::new(None, move |event| {
            if let PaintWidgetEvent::Paint(painter, width, height) = event {
                // Parent/related QObject access is forbidden even via another
                // checked clone while the stack-scoped QPainter is borrowed.
                expect_panic("native state is borrowed", || parent.resize(2, 2));
                expect_panic("native state is borrowed", || QTimer::single_shot(0, || {}));
                parent.delete_later(); // request only; never flush from paint
                drop(value.take()); // unique opaque deletion is retired, too
                painter.fill_rect(0, 0, width, height, &dtk::QColor::rgb(1, 2, 3));
                paint_seen.set(true);
            }
        });
        window.set_central_widget(&paint.as_widget());
        paint.show();

        let table = QTableWidget::new(1, 1);
        table.set_cell_text(0, 0, "paint index");
        let (table_alias, model_seen) = (table.clone(), delegated.clone());
        let delegate = PaintDelegate::new(&table, move |painter, index, _, _, _, _, _| {
            expect_panic("native state is borrowed", || table_alias.set_row_count(0));
            assert_eq!(
                index.data_string(dtk::qt::item_role::DISPLAY_ROLE),
                "paint index"
            );
            painter.set_pen_color(&dtk::QColor::rgb(3, 2, 1));
            model_seen.set(true);
        });
        table.set_delegate_for_column(0, &delegate);
        table.resize(240, 160);
        table.show();
        // update/paint happen on real Qt event delivery; no raw pointer fixture.
        let result = Rc::new(Cell::new(false));
        let checked = result.clone();
        let check = QTimer::new(app);
        let polls = Rc::new(Cell::new(0));
        check.on_timeout(move || {
            polls.set(polls.get() + 1);
            if painted.get() && delegated.get() && !window.is_alive() {
                checked.set(true);
                DApplication::quit();
            } else {
                assert!(
                    polls.get() < 500,
                    "native paint/retirement checks did not finish"
                );
            }
        });
        check.start(1);
        assert_eq!(app.exec(), 0);
        assert!(result.get());
    });
}

fn main() {
    argument_conversion_cannot_open_an_aliasing_transaction();
    fd_readiness_is_coalesced_until_the_callback_returns(false);
    fd_readiness_is_coalesced_until_the_callback_returns(true);
    copied_signal_and_native_child_deletion();
    native_animation_keeps_no_deleted_widget_borrows();
    show_callback_with_parent_deletion_and_nested_loop();
    receiver_deletion_during_queued_callback();
    native_destruction_retires_captures();
    paint_and_close_return_borrows();
    assert!(dtk_sys::callback_registry_is_empty());
    println!(
        "runtime borrow selfcheck ok: signal/frame ordering, receiver+parent deletion, modal loops, capture retirement, painter/index/close borrows"
    );
}
