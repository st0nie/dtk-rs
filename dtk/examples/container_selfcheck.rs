//! Generated container and Qt-class enum mappings: `QList<int>` returns as an
//! owned `Vec<i32>`, `QList<int>`/`QVector<int>` parameters as `&[i32]`,
//! `QList<QString>` as `Vec<String>`, and `QEasingCurve::Type` as `i32`.
//! QT_QPA_PLATFORM=offscreen cargo run --example container_selfcheck
use dtk::widgets::{DColoredProgressBar, DSlider, DStackWidget};
use dtk::{DApplication, QBrush, QColor};

fn main() {
    DApplication::with("dtk-container-selfcheck", |_app| {
        let bar = DColoredProgressBar::new();
        assert!(bar.thresholds().is_empty());
        let brush = QBrush::from_color(&QColor::from_rgb(1, 2, 3));
        for threshold in [70, 30, 90] {
            bar.add_threshold(threshold, &brush);
        }
        bar.remove_threshold(90);
        let mut thresholds = bar.thresholds();
        thresholds.sort();
        assert_eq!(thresholds, [30, 70], "owned copy of QList<int>");

        // QEasingCurve::InOutQuad == 3; the getter returns the native value.
        let stack = DStackWidget::new();
        stack.set_animation_type(3);
        assert_eq!(stack.animation_type(), 3);

        let slider = DSlider::new();
        slider.set_mark_positions(&[0, 50, 100]);
        slider.set_mark_positions(&[]);
        assert!(slider.tick_position() >= 0);
    });
    println!("container selfcheck ok");
}
