fn main() {
    // we can use references when we do not want to give ownership of a variable
    let mut s = String::from("owner");
    let size_s = get_len(&s);
    println!("word {}, with len {}", s, size_s); // s is valid here, it was passed by ref (borrowed)
    append_to_end(&mut s);                                                 
    // This concept of passing without relinquising ownership is called borrowing
    // however, we cannot modify the variable in the other function when we borrow it (immutable default)  
    println!("word is now {}, with len {}", s ,get_len(&s));

    // NOTE: If you have a mutable reference to a value, there cannot be more than 1 (we also
    // cannot have mutable reference when we have a normal reference as well)
}

fn get_len(s: &String) -> usize {
    s.len()
}

fn append_to_end(s : &mut String) { 
    s.push_str(", borrower");
}
