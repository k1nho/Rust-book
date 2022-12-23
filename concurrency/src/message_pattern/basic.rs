use std::sync::mpsc;
use std::thread;

pub fn message_pattern() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("thread working");
        tx.send(val).unwrap();
    });

    // recv blocks the thread, so if we want to have the main thread working
    // we need to use try_recv
    let message = rx
        .recv()
        .unwrap_or_else(|err| String::from("no data from channel"));

    println!("Message from thread: {}", message);
}
