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

}
