use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("-------------------");
    println!("*** Guess Game ***");
    println!("-------------------");
    
    let secret_number = rand::thread_rng().
        gen_range(1..=100);

    loop {
        let mut user_guess = String::new(); // mut keyword makes a variable mutable 
                                            //
        io::stdin().read_line(&mut user_guess)
            .expect("Failed to read input");

        // convert the number into an integer
        let user_guess :u32 = match user_guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        // now match the given number to an Ordering
        match user_guess.cmp(&secret_number) {
            Ordering::Less => println!("Smaller than number"),
            Ordering::Greater => println!("Bigger than number"),
            Ordering::Equal => { println!("Correct !"); break;},
        }
    }

    println!("You won! congratulations!");

}
