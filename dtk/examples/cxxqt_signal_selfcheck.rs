//! CXX-Qt typed signals on imported DTK QObjects, under dtk-rs runtime rules.
//! QT_QPA_PLATFORM=offscreen cargo run --example cxxqt_signal_selfcheck
//!
//! Covers: typed connect / RAII disconnect / release, closure release when Qt
//! destroys the sender, deferred delivery after the NativeCall that emitted,
//! FIFO nested emission without FnMut reentry, owned QString conversion,
//! multi-argument signals, shared guarded clones, foreign-thread ids and
//! application-scope cleanup of released connections.
use dtk::{
    Connection, DApplication, QTimer, unregister_callback,
    widgets::{DDialog, DLineEdit, DSearchEdit, DSlider, DSwitchButton, DTabBar},
};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

fn main() {
    DApplication::with("dtk-cxxqt-signal-selfcheck", |app| {
        DApplication::set_quit_on_last_window_closed(false);
        typed_bool_queue_and_raii();
        release_and_foreign_thread();
        owned_qstring_and_multi_args();
        destroyed_sender_releases_closure(app);
        // Released connections on surviving objects: closed by scope exit.
        let survivor = DSwitchButton::new();
        let id = survivor.on_checked_changed(|_| {}).release();
        assert!(dtk_sys::native_signal::is_connected(id));
        std::mem::forget(survivor); // native object reclaimed by scope teardown
    });
    assert!(
        dtk_sys::callback_registry_is_empty(),
        "typed connections and closures must not survive the application scope"
    );
    println!("cxxqt signal selfcheck ok");
}

fn typed_bool_queue_and_raii() {
    let button = DSwitchButton::new();
    let seen = Rc::new(RefCell::new(Vec::new()));
    let (view, log) = ((*button).clone(), seen.clone());
    let connection = button.on_checked_changed(move |checked| {
        // A guarded call here needs a fresh NativeCall: it would panic if the
        // closure ran inside set_checked's still-open native transaction.
        assert_eq!(view.is_checked(), log.borrow().is_empty());
        log.borrow_mut().push(checked);
        if checked {
            // Nested emission of the same signal: queued FIFO, not reentrant.
            view.set_checked(false);
            assert_eq!(log.borrow().len(), 1, "same FnMut must not be reentered");
        }
    });
    assert!(connection.is_connected());
    button.set_checked(true);
    assert_eq!(
        *seen.borrow(),
        [true, false],
        "nested emission lost/reordered"
    );
    assert!(!button.is_checked());

    drop(connection); // RAII disconnect
    button.set_checked(true);
    assert_eq!(seen.borrow().len(), 2, "dropped Connection still delivered");

    // Disconnecting from inside its own callback cancels later queued values.
    let calls = Rc::new(Cell::new(0));
    let slot: Rc<RefCell<Option<Connection>>> = Rc::default();
    let (view, count, own) = ((*button).clone(), calls.clone(), slot.clone());
    *slot.borrow_mut() = Some(button.on_checked_changed(move |_| {
        count.set(count.get() + 1);
        view.set_checked(!view.is_checked()); // queues one more value
        drop(own.borrow_mut().take()); // ...which must now be discarded
    }));
    button.set_checked(false);
    assert_eq!(calls.get(), 1, "cancelled typed callback was resurrected");
    assert!(slot.borrow().is_none());
}

fn release_and_foreign_thread() {
    let button = DSwitchButton::new();
    let calls = Rc::new(Cell::new(0));
    let count = calls.clone();
    let id = button
        .on_checked_changed(move |_| count.set(count.get() + 1))
        .release();
    button.set_checked(true);
    assert_eq!(calls.get(), 1, "released connection must stay connected");
    assert!(
        !std::thread::spawn(move || unregister_callback(id))
            .join()
            .unwrap(),
        "a worker must not disconnect a GUI-thread typed connection"
    );
    button.set_checked(false);
    assert_eq!(calls.get(), 2);
    assert!(unregister_callback(id));
    button.set_checked(true);
    assert_eq!(calls.get(), 2);
    assert!(!dtk_sys::native_signal::is_connected(id));
}

fn owned_qstring_and_multi_args() {
    let edit = DLineEdit::new();
    let texts = Rc::new(RefCell::new(Vec::new()));
    let log = texts.clone();
    let _text = edit.on_text_changed(move |text| log.borrow_mut().push(text));
    edit.set_text("中文 ✓ dtk-rs");
    edit.set_text("");
    assert_eq!(*texts.borrow(), ["中文 ✓ dtk-rs", ""]);

    let slider = DSlider::new();
    let events = Rc::new(RefCell::new(Vec::new()));
    let log = events.clone();
    let _range = slider.on_range_changed(move |min, max| log.borrow_mut().push((min, max)));
    let log = events.clone();
    let _value = slider.on_value_changed(move |value| log.borrow_mut().push((value, -1)));
    slider.set_maximum(50);
    slider.set_value(42);
    assert!(events.borrow().contains(&(0, 50)), "{:?}", events.borrow());
    assert_eq!(events.borrow().last(), Some(&(42, -1)));

    let tabs = DTabBar::new();
    let moves = Rc::new(RefCell::new(Vec::new()));
    let log = moves.clone();
    let _moved = tabs.on_tab_moved(move |from, to| log.borrow_mut().push((from, to)));
    let log = moves.clone();
    let _current = tabs.on_current_changed(move |index| log.borrow_mut().push((index, -1)));
    tabs.add_tab("a");
    tabs.add_tab("b");
    tabs.set_current_index(1);
    tabs.move_tab(0, 1);
    let moves = moves.borrow();
    assert_eq!(moves.first(), Some(&(0, -1)), "{moves:?}");
    assert!(
        moves.contains(&(1, -1)) && moves.contains(&(0, 1)),
        "{moves:?}"
    );
    drop(moves);

    // Inherited DLineEdit signal through the subclass import.
    let search = DSearchEdit::new();
    let connection = search.on_text_changed(|_| {});
    assert!(connection.is_connected());

    let dialog = DDialog::new();
    let shown = Rc::new(RefCell::new(Vec::new()));
    let log = shown.clone();
    let _visible = dialog.on_visible_changed(move |visible| log.borrow_mut().push(visible));
    dialog.as_widget().show();
    dialog.as_widget().hide();
    assert_eq!(*shown.borrow(), [true, false]);
}

fn destroyed_sender_releases_closure(app: &DApplication) {
    let button = DSwitchButton::new();
    let probe = Rc::new(());
    let weak = Rc::downgrade(&probe);
    let connection = button.on_checked_changed(move |_| {
        let _ = &probe;
    });
    let view = (*button).clone(); // shared guarded handle, not an owner
    view.delete_later();
    QTimer::single_shot(20, DApplication::quit);
    assert_eq!(app.exec(), 0);
    assert!(!view.is_alive(), "Qt deleted the sender");
    assert!(
        weak.upgrade().is_none(),
        "Qt deletion must release the closure"
    );
    assert!(!connection.is_connected());
    drop(connection); // disconnecting after sender destruction is a no-op
    drop(button); // escaped dead owner does nothing
}
