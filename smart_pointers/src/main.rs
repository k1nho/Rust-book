enum List {
    Cons(i32, Box<List>),
    Nil,
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
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Two traits are key to the implmentation of Box: Deref, and Drop
}
