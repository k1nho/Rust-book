use std::cmp;
use std::io;
use std::collections::{HashMap, HashSet};

struct Stats {
    median : f32,
    mode : i32,
}

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
    println!();
    
    // keep in mind that hashmap follow the same ownership rules of non-Copy trait types
    // if we pass a string into the hashmap it will take ownership of the string
    let team =  String::from("Hawk");
    scores.insert(team, 1);
    // team is no longer valid here

    // adding key, val pair if key already exists
    scores.entry(String::from("Hawk")).or_insert(10);
    scores.entry(String::from("Feing")).or_insert(20);
    
    for (key, value) in &scores {
        println!("k: {}, v: {}", key, value);
    }

    let s = String::from("abc one two three four");
    let mut freq = HashMap::new();

    for i in s.chars() {
        if i == ' ' {continue;}
        let val = freq.entry(i).or_insert(0);
        *val+=1;
    }

    for(key, val) in &freq {
        println!("k: {}, v: {}", key, val);
    }
    
    let list = vec![1,2,3,4];
    let stats = find_median_and_mode(&list);
    println!("Median: {}, Mode: {}", stats.median, stats.mode);

    let sentence = "hello pig conversion awesome";
    let pig_sentence = convert_string(sentence);
    
    println!("sentence: {}", sentence);
    println!("pig sentence: {}", pig_sentence);

    management_cli();

}

fn find_median_and_mode(v: &Vec<i32>) -> Stats {
    let mut freq = HashMap::new();
    let n = v.len();
    let m : f32;
    let mut max_freq = 0;

    if n%2 == 0 {
       m = (v[n/2 - 1] + v[(n/2)]) as f32/2.0; 
    }
    else {
       m = v[n/2] as f32;
    }

    for i in v {
        let val = freq.entry(i).or_insert(0);
        *val += 1;
        max_freq = cmp::max(max_freq, *val); 
    }    

    let res = Stats {
        median : m, 
        mode: max_freq,
    };

    return res;
   
}

fn convert_string(s : &str) -> String{
    let mut vowels = HashSet::new();
    let mut res = String::new();
    vowels.insert('a');
    vowels.insert('e');
    vowels.insert('i');
    vowels.insert('o');
    vowels.insert('u');

    for word in s.split_whitespace() {
        let letter = word.chars().next().unwrap();
        if vowels.contains(&letter){
            res.push_str(&format!(" {}-hay",word)[0..]);
        }
        else {
            let cut_word = &word[1..];
            res.push_str(&format!(" {}-{}ay", cut_word, letter)[0..]);
        }
    }
    
    res
}

fn management_cli() {
    
   // map a deparment to the personnel
   let mut system : HashMap<String, Vec<String>>  = HashMap::new();
   let mut cmd = String::from("");
   while cmd != "3" {
       println!("\n**********************"); 
       println!("management"); 
       println!("1. add personnel"); 
       println!("2. get deparment staff"); 
       println!("3. Quit"); 
       println!("**********************"); 
        cmd.clear(); // must clear the string otherwise is just going to append
        io::stdin().read_line(&mut cmd).expect("could not read the command");
        let cmd = cmd.trim();
    
       let mut name = String::from("");
       let mut deparment = String::from("");
        match cmd {
               "1" => {
                   println!("Personnel name:");
                   io::stdin().read_line(&mut name).expect("could not read data");
                   println!("Deparment name:");
                   io::stdin().read_line(&mut deparment).expect("could not read data");
                   let name = name.trim().to_string();
                   let deparment = deparment.trim().to_string();
                   system.entry(deparment).or_insert(Vec::new()).push(name);
               },
               
               "2" => {
                   println!("Deparment name:");
                   io::stdin().read_line(&mut deparment).expect("could not read data");
                   let deparment = deparment.trim().to_string();
                   if system.contains_key(&deparment) {
                        let v = system.get(&deparment); 

                        for person in &v {
                            println!("{:?}", person);
                        }
                   }
               },
               _ => (),
           }
   }   

   println!("Exit system");

        
} 
