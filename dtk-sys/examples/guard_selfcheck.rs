//! QPointer lifetime check without dereferencing a dangling widget handle.
//! QT_QPA_PLATFORM=offscreen cargo run -p dtk-sys --example guard_selfcheck
use dtk_sys::ffi;
use std::cell::Cell;
use std::rc::Rc;

fn main() {
    let name = b"dtk-guard-selfcheck";
    let owner = unsafe {
        ffi::application_runtime_new(
            "dtk-guard-selfcheck",
            name.to_vec(),
            vec![name.len()],
            true,
            0,
        )
    };
    let app = owner.as_ref().expect("scoped native GUI application") as *const _ as *mut _;
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
        // The low-level bridge also rejects recursive exec while a native
        // event/FFI frame is active. Keep the owner outside this callback.
        assert!(!unsafe { ffi::application_runtime_can_exec(&*app) });
        assert_eq!(unsafe { ffi::application_runtime_exec(app) }, -1);
        checked.set(dead);
        unsafe { ffi::application_quit() };
    });
    unsafe { ffi::timer_single_shot(20, id) };
    assert_eq!(unsafe { ffi::application_runtime_exec(app) }, 0);
    assert!(
        saw_deletion.get(),
        "QPointer must clear on Qt parent destruction"
    );
    drop(owner); // outside all native event/exec frames
    assert!(unsafe { ffi::application_can_construct() });
    assert!(dtk_sys::callback_registry_is_empty());
    // A fresh runtime must own fresh writable argv, not reuse Qt-mutated
    // static storage from the preceding application.
    let argument = "second|scope\u{1f}中文";
    let mut bytes = b"second-app".to_vec();
    bytes.extend_from_slice(argument.as_bytes());
    let next =
        unsafe { ffi::application_runtime_new("second", bytes, vec![10, argument.len()], true, 0) };
    assert!(!next.is_null());
    assert!(unsafe { ffi::application_has_arg("second-app") });
    assert!(unsafe { ffi::application_has_arg(argument) });
    assert!(!unsafe { ffi::application_has_arg("dtk-guard-selfcheck") });
    assert!(unsafe { ffi::application_runtime_worker_roundtrip() });
    let queued = unsafe { ffi::config_worker_probe_new() };
    drop(next); // no exec: A is latched until teardown's FIFO marker is queued
    assert!(unsafe { ffi::config_worker_probe_completed(queued.as_ref().unwrap()) });
    drop(queued);
    assert!(unsafe { ffi::application_can_construct() });
    assert!(dtk_sys::callback_registry_is_empty());
    assert!(unsafe { ffi::application_runtime_workers_stopped() });
    assert_eq!(unsafe { ffi::relay_live_count() }, 0);
    println!(
        "guard selfcheck ok: QPointer, recursive exec rejection, fresh argv, accepted DConfig queue completion"
    );
}
