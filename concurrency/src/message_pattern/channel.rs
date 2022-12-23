use std::sync::mpsc;
use std::thread;

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
