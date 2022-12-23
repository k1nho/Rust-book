use std::sync::{Arc, Mutex};
use std::thread;

pub fn mutex_pattern() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("number is {:?}", m);
}

pub fn sharing_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut thread_vector = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let t = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        thread_vector.push(t);
    }

    for t in thread_vector {
        t.join().unwrap();
    }

    println!(
        "the final result after all threads finish is {}",
        *counter.lock().unwrap()
    );
}
