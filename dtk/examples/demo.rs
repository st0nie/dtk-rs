use dtk::*;
use std::cell::Cell;
use std::rc::Rc;

fn main() {
    let app = DApplication::new("dtk-rs-demo");

    let win = DMainWindow::new();
    win.set_window_title("dtk-rs demo");
    let tb = win.titlebar();
    tb.set_title("dtk-rs demo");
    tb.set_icon(&QIcon::from_theme("deepin-liferaft"));

    let central = QWidget::new(None);
    let vbox = QVBoxLayout::new(Some(&central));
    vbox.set_contents_margins(20, 20, 20, 20);
    vbox.set_spacing(12);

    let label = DLabel::new("DTK6 Rust binding is up.");
    vbox.add_widget(&label.as_widget());

    let btn = DSuggestButton::new("Click me");
    let clicked = Rc::new(Cell::new(false));
    {
        let clicked = clicked.clone();
        btn.on_clicked(move || {
            label.set_text("Button was clicked ✓");
            clicked.set(true);
            println!("clicked");
        });
    }
    vbox.add_widget(&btn.as_widget());

    // new binding checks: real argv, palette getter, window icon, box-layout stretch/alignment
    if std::env::args().any(|a| a == "--smoke") {
        assert!(
            DApplication::has_arg("--smoke"),
            "real argv not passed to QApplication"
        );
        assert!(!DApplication::has_arg("--hidden"));
        // Qt must receive an argv entry verbatim, even with the old join delimiter.
        let control_arg = "--unit\u{1f}separator";
        if std::env::args().any(|arg| arg == control_arg) {
            assert!(DApplication::has_arg(control_arg));
        }
        let pal = win.palette();
        let hl = pal.color(qt::palette_group::ACTIVE, qt::palette_role::HIGHLIGHT);
        pal.set_color(
            qt::palette_group::INACTIVE,
            qt::palette_role::HIGHLIGHT,
            &hl,
        );
        let got = pal.color(qt::palette_group::INACTIVE, qt::palette_role::HIGHLIGHT);
        assert_eq!(
            got.rgba_u32(),
            hl.rgba_u32(),
            "palette color round-trip broken"
        );
        win.set_window_icon(&QIcon::from_theme("deepin-liferaft"));

        let hbox = QHBoxLayout::new(None);
        hbox.add_stretch(1); // pushes the button right
        let b2 = DPushButton::new("right");
        hbox.add_widget_ex(&b2.as_widget(), 0, qt::alignment::TOP);
        vbox.add_layout(&hbox);
    }

    // --smoke only: verify PaintDelegate + QSocketNotifier chains
    let smoke = std::env::args().any(|a| a == "--smoke");
    let painted = Rc::new(Cell::new(false));
    let notified = Rc::new(Cell::new(false));
    let mut pipe_w = 0i32;
    if smoke {
        let table = QTableWidget::new(1, 2);
        table.set_cell_text(0, 0, "icon cell");
        table.set_cell_text(0, 1, "memory cell");
        {
            let painted = painted.clone();
            let delegate = PaintDelegate::new(&table, move |p, idx, x, y, w, h, _state| {
                assert_eq!(idx.data_string(0), "icon cell"); // DisplayRole
                p.set_clip_rect(x, y, w, h);
                p.fill_rect(x, y, w, h, &QColor::rgb(200, 220, 255));
                let elided = p.elided_text("a-very-long-application-name", qt::elide::RIGHT, w);
                assert!(
                    elided.ends_with('\u{2026}') || elided.len() < 26,
                    "elide failed: {elided}"
                );
                p.draw_text(x, y, w, h, qt::alignment::CENTER, &elided);
                painted.set(true);
            });
            table.set_delegate_for_column(0, &delegate);
        }
        vbox.add_widget(&table.as_widget());

        // self-pipe to test QSocketNotifier
        let mut fds = [0i32; 2];
        assert_eq!(unsafe { libc_pipe(fds.as_mut_ptr()) }, 0);
        pipe_w = fds[1];
        let notifier = QSocketNotifier::new(fds[0], &win);
        {
            let notified = notified.clone();
            let fd_r = fds[0];
            notifier.on_activated(move || {
                let mut buf = [0u8; 8];
                unsafe { libc_read(fd_r, buf.as_mut_ptr(), 8) }; // drain, or level-trigger refires forever
                notified.set(true);
                println!("notified");
            });
        }
    }

    win.set_central_widget(&central);
    win.resize(400, 200);
    win.show();

    if smoke {
        QTimer::single_shot(100, move || btn.click());
        QTimer::single_shot(150, move || unsafe {
            libc_write(pipe_w, b"x".as_ptr() as _, 1);
        });
        QTimer::single_shot(500, move || {
            assert!(
                clicked.get(),
                "clicked callback never fired (signal chain broken)"
            );
            assert!(painted.get(), "PaintDelegate paint callback never fired");
            assert!(notified.get(), "QSocketNotifier activated never fired");
            println!("smoke ok");
            DApplication::quit();
        });
    }

    std::process::exit(app.exec());
}

// no libc crate; declare the syscalls directly
unsafe extern "C" {
    fn pipe(fds: *mut i32) -> i32;
    fn write(fd: i32, buf: *const u8, n: usize) -> isize;
    fn read(fd: i32, buf: *mut u8, n: usize) -> isize;
}
unsafe fn libc_pipe(fds: *mut i32) -> i32 {
    unsafe { pipe(fds) }
}
unsafe fn libc_write(fd: i32, buf: *const u8, n: usize) -> isize {
    unsafe { write(fd, buf, n) }
}
unsafe fn libc_read(fd: i32, buf: *mut u8, n: usize) -> isize {
    unsafe { read(fd, buf, n) }
}
