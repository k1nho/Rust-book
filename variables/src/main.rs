fn main() {
    // Normal definition of variables can be done with let or const
    let num: u32 = 5;
    const PI: f32 = 3.1415;
    println!("num is {num}");
    println!("PI is {PI}");
    
    // a variable is by default immutable we need to add mutability by adding the keyword mut
    let mut name = "name";
    println!("name is  {name}");
    name = "Pikachu";
    println!("name is now {name}");

    // we can reuse the name of a variable in a program via shadowing
    let num = "32 in string";

    // NOTE: Mutability != Shadowing, the idea is that we can apply some operations to a variable
    // via shadowing and have it be immutable still. The other idea is that shadowing can change
    // datatypes for the variable after the transformation, which can not be done with mutability.

    println!("num is {num}");

    
}
