//! QPointer lifetime check without dereferencing a dangling widget handle.
//! QT_QPA_PLATFORM=offscreen cargo run -p dtk-sys --example guard_selfcheck
use dtk_sys::ffi;
use std::cell::Cell;
use std::rc::Rc;

fn main() {
    let name = b"dtk-guard-selfcheck";
    let app =
        unsafe { ffi::application_new("dtk-guard-selfcheck", name.to_vec(), vec![name.len()]) };
    assert!(!app.is_null());
    unsafe { ffi::application_set_quit_on_last_window_closed(false) };
    let parent = unsafe { ffi::widget_new(std::ptr::null_mut()) };
    let child = unsafe { ffi::widget_new(parent) };
    let guard = unsafe { ffi::guard_widget(child) };
    assert!(unsafe { ffi::guard_alive(guard.as_ref().expect("QPointer created")) });

    // Qt deletes the child with its parent. Retain the child's guard and
    // inspect it without dereferencing the now-dangling child pointer.
    unsafe { ffi::object_delete_later(parent.cast()) };
    let saw_deletion = Rc::new(Cell::new(false));
    let checked = saw_deletion.clone();
    let id = dtk_sys::register_cb_once(move || {
        let dead = !unsafe { ffi::guard_alive(guard.as_ref().expect("guard still owned")) };
        checked.set(dead);
        unsafe { ffi::application_quit() };
    });
    unsafe { ffi::timer_single_shot(20, id) };
    assert_eq!(unsafe { ffi::application_exec(app) }, 0);
    assert!(
        saw_deletion.get(),
        "QPointer must clear on Qt parent destruction"
    );
    println!("guard selfcheck ok");
}
