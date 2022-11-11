fn main() {
    // vectors
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


}
