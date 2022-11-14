use std::fs::File;
use std::io::ErrorKind;
fn main() {
    // unrecoverable errors (RUST_BACKTRACE to 1 will give us the backtrace of the error)
    // panic!("stop! unrecoverable error has been encountered");

    // recoverable errors with Result Enum
    let data = File::open("hello.txt");

    let file = match data {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => match  File::create("hello.txt"){ 
                        Ok(fc) => fc,
                        Err(e) => panic!("Could not create file {:?}", e),
                },
                other_error => panic!("cannot open the file {:?}", other_error),
            }
        }
    };
    
}
