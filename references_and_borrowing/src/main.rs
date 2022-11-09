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
    
    let borrow = dangle();
    println!("borrow obtained string is  {}", borrow);

}

fn get_len(s: &String) -> usize {
    s.len()
}

fn append_to_end(s : &mut String) { 
    s.push_str(", borrower");
}

fn dangle() -> String {
    let s = String::from("borrow");

    //&s (not valid, returning a reference to the string but it goes out of scope at the end of the
    //function so memory gets deallocated and we are left with a dangling pointer, rust catches it)

    s // correct way to return the string we give up ownership to where this function is returning 
}
