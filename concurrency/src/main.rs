use std::thread;
use std::time::Duration;

fn main() {
    // spawning a thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {}, spawned thread", i);
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
}
