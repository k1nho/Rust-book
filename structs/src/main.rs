struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
}

// unnamed Structs (used when we need a type, otherwise use tuple)
#[derive(Debug)]
struct RGB(u32, u32, u32);
struct Point(u32, u32);

impl Point {
    fn slope(&self, p: &Point) -> u32 {
       if p.0 - self.0 == 0 {return 0;}
       else {
            return (p.1 - self.1)/(p.0 -self.0);
       }
    }
}

fn main() {
    
    // instance of User
    let mut usr = User {
        active: true,
        email: String::from("Ocha@example.com"),
        username: String::from("NoUser"),
        sign_in_count: 1,
    };

    usr.username = String::from("Ocha");
    
    let new_usr = build_user(String::from("Ginro@example.com"), String::from("Ginro"));
    println!("User has been created! name: {}, email: {}", new_usr.username, new_usr.email);
    
    // we can copy fields into another object instance
    // however, note that if we copy mutable fields then the other object mutable field will no longer
    // exists(ownership system)
    
    let copy_usr = User {
        email: String::from("copy@example.com"),
        ..new_usr
    };
    
    // username field of new_usr is no longer accessible (its ownership was moved)
    println!("the user of copy is {}, is currently {}, and it has sign in {} times", copy_usr.username, copy_usr.active, copy_usr.sign_in_count);

    let blue = RGB(0, 0, 255);
    println!("blue position is {}", blue.2);
    let origin  = Point(0, 0);
    let slope = origin.slope(&Point(5,5));
    println!("The point coordinates are x: {}, y: {}, and its slope is {}", origin.0, origin.1, slope);
    // printing entire structs with the debug trait
    println!("The color blue is defined as {:?}", blue);
    // or by using dbg macro
    dbg!(&blue);
    
}

fn build_user(email: String, username: String) -> User {
    // using field shorhand
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
