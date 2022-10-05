fn main() {
    let mut s = String::from("Hello");
    // mutating String
    s.push_str(", World");
    let s2 = s; // s2 is now the owner (this allows rust to avoid double memory freeing)
    // NOTE: this code: let s3 = s.clone(); will fail since a move of the string allocated happened
    // before on s2.
    
    let s3 = s2.clone(); // creates a copy of the heap
    println!("{} {}", s2, s3);

    // CONTRADICTION ?
    // The following seems like a contradiction of the ownership model
    let x = 10;
    let y = x;
    println!("x: {x} y : {y}");
    // However, this allocation happens on the stack because integers size are known at compile
    // time because of this Rust is smart enough to not invalidate x
    // The difference is in the annotation called 'Copy' trait that can be used on types stored in the
    // stack, whereas non-stack ones usually have the 'Drop' trait to deallocate memory
}
