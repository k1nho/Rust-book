use std::fs::File;
use std::io::{self, Read, ErrorKind};
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

    // better shortcuts (unwrap will return the value or it will panic depending on the result)
    let short_file = File::open("hello.txt").unwrap();
    // for prod expect is slightly better to know why the code failed
    let short_file_ex = File::open("hello.txt").expect("error opening a file"); 


    
    
}

fn read_username_from_file() -> Result<String, io::Error> {
    // propagating errors (effectively with ?)
    let mut user = String::from("");
    File::open("name.txt")?.read_to_string(&mut user)?;
    Ok(user)
}
