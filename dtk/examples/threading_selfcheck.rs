//! Headless regression for the typed worker -> GUI channel.
//! Run with QT_QPA_PLATFORM=offscreen cargo run --example threading_selfcheck.
use dtk::{DApplication, GuiSender, QTimer, QWidget};
use std::cell::Cell;
use std::io::ErrorKind;
use std::rc::Rc;

fn assert_send_sync<T: Send + Sync>() {}

fn main() {
    assert_send_sync::<GuiSender<usize>>();
    let app = DApplication::new("dtk-threading-selfcheck");
    DApplication::set_quit_on_last_window_closed(false);
    let owner = QWidget::new(None);
    let received = Rc::new(Cell::new(0usize));
    let count = received.clone();
    const N: usize = 1024; // several bounded GUI delivery batches
    let (check_tx, check_rx) = std::sync::mpsc::channel();
    let gui_owner = owner.clone();
    let sender = GuiSender::new(&owner, move |msg| {
        assert_eq!(msg, count.get(), "worker messages must stay ordered");
        count.set(count.get() + 1);
        if msg + 1 == N {
            gui_owner.delete_later();
            let count = count.clone();
            let tx = check_tx.clone();
            QTimer::single_shot(50, move || {
                assert_eq!(count.get(), N);
                tx.send(()).unwrap();
                DApplication::quit();
            });
        }
    })
    .expect("create GUI sender");

    let later = sender.clone();
    let worker = std::thread::spawn(move || {
        for n in 0..N {
            sender.send(n).expect("queue worker message");
        }
    });
    assert_eq!(app.exec(), 0);
    worker.join().unwrap();
    check_rx.recv().unwrap();
    let error = later.send(N).expect_err("Qt owner destroyed");
    assert_eq!(error.kind(), ErrorKind::BrokenPipe);
    println!("threading selfcheck ok");
}
