//! Nested Qt signal emissions must be queued, not lost or alias FnMut.
//! QT_QPA_PLATFORM=offscreen cargo run --example signal_reentrancy_selfcheck
use dtk::{DApplication, DPushButton, QTimer, unregister_callback};
use std::cell::Cell;
use std::rc::Rc;

fn main() {
    let app = DApplication::new("dtk-signal-reentrancy-selfcheck");
    DApplication::set_quit_on_last_window_closed(false);
    let button = DPushButton::new("recursive click");
    let count = Rc::new(Cell::new(0));
    let (view, calls) = (button.clone(), count.clone());
    let id = button.on_clicked(move || {
        calls.set(calls.get() + 1);
        if calls.get() == 1 {
            view.click(); // Qt delivers clicked(bool) reentrantly to the same relay
        }
    });
    button.click();
    assert_eq!(count.get(), 2, "nested clicked signal was dropped");
    assert!(unregister_callback(id));

    let cancelled = Rc::new(Cell::new(0));
    let (view, calls) = (button.clone(), cancelled.clone());
    let callback_id = Rc::new(Cell::new(0));
    let slot = callback_id.clone();
    let id = button.on_clicked(move || {
        calls.set(calls.get() + 1);
        if calls.get() == 1 {
            view.click();
            assert!(unregister_callback(slot.get()));
        }
    });
    callback_id.set(id);
    button.click();
    assert_eq!(
        cancelled.get(),
        1,
        "cancelled signal callback was resurrected"
    );
    assert!(!unregister_callback(id));

    let foreign_calls = Rc::new(Cell::new(0));
    let observed = foreign_calls.clone();
    let id = button.on_clicked(move || observed.set(observed.get() + 1));
    assert!(id != 0);
    assert!(
        !std::thread::spawn(move || unregister_callback(id))
            .join()
            .unwrap(),
        "worker must not access the GUI relay or remove its callback"
    );
    button.click();
    assert_eq!(foreign_calls.get(), 1);
    assert!(unregister_callback(id));
    button.click();
    assert_eq!(foreign_calls.get(), 1);

    button.delete_later();
    QTimer::single_shot(10, DApplication::quit);
    assert_eq!(app.exec(), 0);
    println!("signal reentrancy selfcheck ok");
}
