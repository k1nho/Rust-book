#[derive(Debug)]
struct Rectangle { 
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name : &str) -> String {
    format!("Hello! {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
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

}
