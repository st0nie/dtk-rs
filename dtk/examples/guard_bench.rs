//! Compare checked QWidget/DTK getter overhead to the same raw FFI call.
//! QT_QPA_PLATFORM=offscreen cargo run --release --example guard_bench
use dtk::widgets::DComboBox;
use dtk::{DApplication, QTimer, QWidget, Signal0};
use dtk_sys::ffi;
use std::hint::black_box;
use std::time::Instant;

const N: usize = 1_000_000;

fn measure(mut get: impl FnMut() -> i32) -> f64 {
    let start = Instant::now();
    let mut sum = 0i64;
    for _ in 0..N {
        sum += black_box(get() as i64);
    }
    black_box(sum);
    start.elapsed().as_nanos() as f64 / N as f64
}

fn main() {
    DApplication::with("dtk-guard-bench", |app| {
        DApplication::set_quit_on_last_window_closed(false);
        let widget = QWidget::new(None);
        let combo = DComboBox::new();
        // QWidget/QObject are the primary Qt base of these objects. The raw path
        // is benchmarking-only and must never be used after their Qt destruction.
        let raw = widget.qobject_ptr().cast::<ffi::QWidget>();
        let combo_raw = combo.as_widget().qobject_ptr().cast::<ffi::QWidget>();
        let baseline = measure(|| unsafe { ffi::widget_width(black_box(raw)) });
        let checked = measure(|| black_box(&widget).width());
        let combo_checked = measure(|| black_box(&combo).width());
        let combo_raw_ns = measure(|| unsafe { ffi::widget_width(black_box(combo_raw)) });
        println!("raw QWidget width: {baseline:.1} ns/call");
        println!("checked QWidget width: {checked:.1} ns/call");
        println!("raw DTK widget width: {combo_raw_ns:.1} ns/call");
        println!("checked DTK widget width: {combo_checked:.1} ns/call");
        widget.delete_later();
        combo.delete_later();
        QTimer::single_shot(10, DApplication::quit);
        assert_eq!(app.exec(), 0);
    });
}
