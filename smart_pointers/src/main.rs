use std::ops::Deref;
use std::rc::Rc;

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percent_of_max = self.value as f64 / self.max as f64;

        if percent_of_max >= 1.0 {
            self.messenger.send("Over the quota");
        } else if percent_of_max >= 0.9 {
            self.messenger.send("90% of the quota consumed");
        } else if percent_of_max >= 0.75 {
            self.messenger.send("75% of the quota consumed");
        }
    }
}

struct FancyData {
    data: String,
    id: u32,
}

impl FancyData {
    fn get_data(&self) -> &str {
        self.data.as_str()
    }
}

impl Drop for FancyData {
    fn drop(&mut self) {
        println!("out of scoped ...  being dropped {}", self.id);
    }
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Deref trait so that the struct MyBox can be derefenreced
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use crate::List::{Cons, Nil};

fn main() {
    // SMART POINTERS

    // Box<T>: enables storage of data in the heap rather than the stack
    // Commonly used in 3 cases:
    // 1. type size cannot be know at compile time and we want to use that value in a context that
    //    requires exact size
    // 2. large amount of data, we want to transfer ownership but ensure that the data wont be
    //    copied
    // 3. own a value and you care that is a type that implements particular trait rather than an
    //    specific type

    let b = Box::new(5);
    print!("this value is a boxed value {}", b);

    // it is possible with box to know the size of a recursively infinite data as it is the case
    // here with list that contains a Cons pair of i32 and List, by boxing the List we know the
    // fixed size of data i32 + box pointer
    let list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));

    // Two traits are key to the implmentation of Box: Deref, and Drop
    let ref_to_i32 = MyBox::new(5);
    println!("the value of the i32 ref is {}", *ref_to_i32);

    let f_data = FancyData {
        data: String::from("Rustup to install"),
        id: 1,
    };
    let t_data = FancyData {
        data: String::from("More rust"),
        id: 2,
    };

    println!("Getting f_data: {}", f_data.get_data());
    println!("Getting t_data: {}", t_data.get_data());
    // Now handle automatic drop by implmenting the drop trait

    // Rc<T> smart pointer
    // Enable multiple ownership (reference counting pointer)
    // clone() increases the reference count
    let a_list = Rc::new(Cons(1, Rc::new(Cons(3, Rc::new(Nil)))));
    let b_list = Cons(2, Rc::clone(&a_list));
    let c_list = Cons(5, Rc::clone(&a_list));
    println!(
        "the current reference count for list a is {}",
        Rc::strong_count(&a_list)
    );

    // RefCell<T> smart pointer
    // interior mutability (mutate data when there are immutable references to it)
    // this happens in runtime, so in case of an error it panics

    // Weak<T> in order to avoid reference cycles that can produce stack overflow
    // we can use Weak<T> which is a variant of Rc<T> such that the counter does not have to reach
    // 0 in order to be dropped
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_90_percent_message() {
        let mock_messanger = MockMessenger::new();
        mock_messanger.send("my first message");

        assert_eq!(mock_messanger.sent_messages.borrow().len(), 1);
    }
}
