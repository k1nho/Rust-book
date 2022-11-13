use std::collections::HashMap;

fn main() {
    // VECTORS
    let mut v: Vec<i32> =  Vec::new();
    let mut v1 = vec![1,2,3];
    
    // modifying
    v.push(32);
    v1.push(50);
    
    //accesing (two ways : indexing, or option matching)
    let i : &i32 = &v[0];
    let j : Option<&i32> = v1.get(2);

    println!("Number accessed from v at index 0 is {}", i);
    match j {
        Some(j) => println!("Number was accesed successfully {}", j),
        None => println!("Index out of bounds"),
    }
    
    // iterating
    for el in &v1 {
        println!("element {}", el);
    }
    v1.pop();
    println!("after removal of last element");
    for el in &v1 {
        println!("element {}", el);
    }


    // modifying values while iterating
    for el in &mut v1 {
        println!("element {}", el);
        *el += 10;
        println!("element after {} ", el);
    }

    // To store multiple types in a vector we can use an Enum
    #[derive(Debug)]
    enum CellType {
        Int(i32),
        Float(f64),
        Text(String),
    } 

    let row = vec![CellType::Int(32), CellType::Float(3.2), CellType::Text(String::from("hello"))];

    for i in &row {
        match i {
           CellType::Int(val) => println!("CellType is {}", val),
           CellType::Float(val) => println!("CellType is {}", val), 
           CellType::Text(val) => println!("CellType is {}", val), 
        }
    }
    
    // STRINGS
    let s = "Linux, Ubuntu and Arch";
    let mut cs = s.to_string(); // converts a literal string to a String type
    println!("s is {}, and cs is {}", s, cs);                            

    // modifying String
    cs.push_str(". I use Arch");
    cs.push('b');
    println!("cs is now {}", cs);                            

    // using the + operator to concatenate, there is a few catches
    // 1. if we use it directly and do not pass teh strings as references we take ownership
    // 2. we can use the format! macro to concatenate all the strings without worrying about ownner
    
    // let concats = cs + s; this will give ownership of cs
    let concats = format!("{} {}", s, cs);
    let dog = "clifford";
    println!("concats is now {}", concats);                            
    
    // Since strings are UTF-8 we need to be aware that accesing a string by indexing might not
    // return the character itself (some can be encoded as more than 1 byte) thus we use chars or
    // bytes to access the actual parts of the string correctly
    for ch in dog.chars() {
        println!("this is the character {}", ch);
    }

    // HASH MAP
    let mut scores = HashMap::new();
    scores.insert(String::from("Nvy"), 10);
    scores.insert(String::from("Optic"), 20);
    
    let score = scores.get("Optic").copied().unwrap_or(0);
    println!("The score of Optic is {}", score);

    // iterating
    for (key, value) in &scores {
        println!("k: {}, v: {}", key, value);
    }



}
