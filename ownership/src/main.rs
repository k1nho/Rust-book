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

    // Ownership in functions
    println!("{} {}", is_palindrome(s2), is_palindrome(String::from("racecar"))); 
    // println!("{}", s2); this code will fail as s2 is relinquished from ownership and the
    // lifecycle of s2 is part of the function is_palindrome() now
    
    let pk_string = create_string(); // ownership is of s from function createString is pk_string
    let give_string = String::from("give a string");
    let take_string = process_and_take_string(give_string);
    
    // give_string will not work since its ownership was relinquish by process_and_take_string
    // function
    println!("pk_string: {}, take_string : {}", pk_string, take_string);

}

fn is_palindrome(s : String) -> bool {
    let mut l = 0;
    let mut r = s.len() -1;
    
    while l <= r {
        if s.chars().nth(l) != s.chars().nth(r) { return false;}
        l += 1; r-=1;
    }
    true
}

fn create_string() -> String {
    let s = String::from("Pokemon"); // s is owner of the String created
    s // here ownership is relinquished
}

fn process_and_take_string(s : String) -> String {
    s // process and take string ownership of s is relinquished once it returns
}
