use std::thread;
use std::time::Duration;

use concurrency::{message_pattern, multiple_message_sending};

fn main() {
    let v = vec![1, 2, 3, 4];
    // spawning a thread (using move to take ownership)
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("spawned thread {}: vector {:?}", i, v);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // if we call join this space then the secondary thread will run completely
    // and only then the main thread will run

    for i in 1..5 {
        println!("hi number {}, spawned form main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // wait for the main thread to finish the work
    handle.join().unwrap();

    multiple_message_sending();
}
