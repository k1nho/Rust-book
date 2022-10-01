use std::io;

fn main() {
    println!("-------------------");
    println!("*** Guess Game ***");
    println!("-------------------");

    let mut user_guess = String::new(); // mut keyword makes a variable mutable 

    io::stdin().read_line(&mut user_guess).expect("Failed to read input");

    println!("Number guessed {user_guess}");
}
