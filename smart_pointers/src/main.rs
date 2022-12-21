use std::ops::Deref;
use std::rc::Rc;

struct FancyData {
    data: String,
    id: u32,
}

impl FancyData {
    fn getData(&self) -> &str {
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

    println!("Getting f_data: {}", f_data.getData());
    println!("Getting t_data: {}", t_data.getData());
    // Now handle automatic drop by implmenting the drop trait

    // Rc<T> smart pointer
    // Enable multiple ownership (reference counting pointer)
    // clone() increases the reference count
    let a_list = Rc::new(Cons(1, Rc::new(Cons(3, Rc::new(Nil)))));
    let b_list = Cons(2, Rc::clone(&a_list));
    let c_list = Cons(5, Rc::clone(&a_list));

    // RefCell<T> smart pointer
}
