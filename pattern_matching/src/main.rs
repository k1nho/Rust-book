struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // the match expression is usually used in the following:
    // match Arms
    // if let conditions
    // while let
    // for loops
    // let statements
    // function parameters

    let mut stk = vec![];
    stk.push(1);
    stk.push(2);
    stk.push(3);

    while let Some(x) = stk.pop() {
        println!("top of the stack {}", x);
    }

    // matching ranges of values
    let x = 5;

    match x {
        1..=10 => println!("one through 10"),
        20 | 30 => println!("20 or 30"),
        _ => println!("it is out of the scope"),
    }

    let letter = 'C';

    match letter {
        'a'..='z' => println!("it is lowercase"),
        'A'..='Z' => println!("it is uppercase"),
        _ => println!("not a letter"),
    }

    // Destructuring structs
    let p = Point { x: 5, y: 10 };
    let Point { x, y } = p;
    println!("the coordinates of the point are ({}, {})", x, y);

    // matching only part of a struct
    match p {
        Point { x, .. } => println!("operating on the x coordinate {}", x),
    }

    let num = Some(4);

    // match guards
    match num {
        Some(x) if x % 2 == 0 => println!("it is even {}", x),
        Some(x) => println!("it is odd {}", x),
        None => (),
    }
}
