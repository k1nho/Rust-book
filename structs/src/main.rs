struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
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
