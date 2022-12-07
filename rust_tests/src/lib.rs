// useful flags
// -- --test-threads=1 (run each test in sequence not parallel)
// -- --show-ouput (show any console log output that is done in the functions)
// cargo test [name_of_test] to run a single test or tests that match the given parameter
// [ignore] can be used to ignore an specific test that can take too long

#[derive(Debug)]
struct Rectangle { 
    width: u32,
    height: u32
}

struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value : i32) -> Guess {
        if value > 100 || value <1 {
            panic!("Guess must in between 1 and 100, got {}", value)
        }
        Guess {value}
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

fn greeting(name : &str) -> String {
    format!("Hello! {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_to_two() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn add_two_to_three() {
        let result = add(3, 2);
        assert_eq!(result, 5);
    }
    
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle{
            width: 10,
            height: 10
        };

        let smaller = Rectangle {
            width: 5,
            height: 2
        };

        assert!(larger.can_hold(&smaller));
    }
    
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle{
            width: 10,
            height: 10
        };

        let smaller = Rectangle {
            width: 5,
            height: 2
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn check_greeting_contains_name() {
        let name = "kin";
        let greet = greeting(name);
        assert!(greet.contains("kin"), "Greeting is: {}, did not contain given name: {}", greet, name)
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(1000);
    }

}
