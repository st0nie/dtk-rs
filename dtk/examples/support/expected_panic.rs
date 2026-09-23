//! Assert a synchronous, deliberately rejected Qt operation panics for the
//! expected reason, without printing a misleading panic trace in smoke tests.

// The callers are single-threaded offscreen examples. Restore the global hook
// before checking the result so unexpected failures are never hidden.
pub fn expect_panic<T>(expected: &str, f: impl FnOnce() -> T) {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    std::panic::set_hook(hook);
    let payload = result
        .err()
        .expect("operation must reject invalid Qt state");
    let message = payload
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| payload.downcast_ref::<&str>().copied())
        .expect("panic must contain a message");
    assert!(message.contains(expected), "unexpected panic: {message}");
}
