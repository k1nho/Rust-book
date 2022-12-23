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

pub fn multiple_message_sending() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg_queue = vec![
            String::from("hello"),
            String::from("How are you?"),
            String::from("3rd message"),
            String::from("end connection"),
        ];

        msg_queue.iter().for_each(|msg| {
            tx.send(msg.to_string()).unwrap();
        });
    });

    for msg in rx {
        println!("{msg}");
    }
}

pub fn multiple_producers() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let msg_queue = vec![
            String::from("hello t1"),
            String::from("How are you? t1"),
            String::from("3rd message t1"),
            String::from("end connection t1"),
        ];

        msg_queue.iter().for_each(|msg| {
            tx1.send(msg.to_string()).unwrap();
        });
    });

    thread::spawn(move || {
        let msg_queue = vec![
            String::from("hello"),
            String::from("How are you?"),
            String::from("3rd message"),
            String::from("end connection"),
        ];

        msg_queue.iter().for_each(|msg| {
            tx.send(msg.to_string()).unwrap();
        });
    });

    for msg in rx {
        println!("{msg}");
    }
}
