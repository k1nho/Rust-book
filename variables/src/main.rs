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

    // compound tuple type
    let tup :(i32, f64, char) = (32, 3.14, 'z');

    let (num, dec, c)  = tup;
    println!("Number is {num}, the decimal is {dec}, and the character is {c}");

    // Accessing individual elements of a tuple (also shadowing)
    let num = tup.0;
    let dec = tup.2;
    let c  = tup.1;
    
    println!("Number is {num}, the decimal is {dec}, and the character is {c}");

    // compound type(array)
    let arr: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    let arr_sum = sum_array(arr);
    println!("Sum of the array is {arr_sum}");
    let arr_sum = sum_with_for(arr);
    println!("Sum of the array with for is {arr_sum}");
    let temp = 32;
    let temp_to_f = c_to_fah(temp);
    let temp_to_c = fah_to_c(temp);
    println!("{temp} celsius is {temp_to_f} in farenheit");
    println!("{temp} farenheit is {temp_to_c} in celsius");

    let mut fibres;
    for el in arr {
        fibres = nth_fibonnacci(el);
        println!("Fibonnacci of {el} is {fibres}")
    }

    // Conditionals
    let y = 5;
    let t_mut: i32;
    if y == 10 {
        println!("Y is not 10");
    } 
    else {
        println!("Y is {y}");
    }
    
    t_mut = if y == 10 {5} else {6};
    println!("t is {t_mut}");

}

fn sum_array(arr : [i32;10]) -> i32 {
    let mut sum = 0;
    let mut n = arr.len();

    while n != 0 { 
        sum += arr[n-1];
        n -= 1;
    }

    sum // NOTE return types are expressions rather than statements
}

fn sum_with_for(arr: [i32;10]) -> i32 {
    let mut sum = 0;
    
    for x in arr {
        sum += x;
    }
    sum
}

fn fah_to_c(fah : i32) -> f64 {
    let celsius = ((fah-32)*5) as f64/9.0;
    celsius
}

fn c_to_fah(celsius : i32) -> f64 {
    let fah = ((celsius*9)as f64/5.0) + 32.0; 
    fah
}

fn nth_fibonnacci(num : i32) -> i32 { 
   let mut prev1 = 1;
   let mut prev2 = 1;
   let mut n = num -2;

   while n > 0 { 
    let x = prev1 + prev2;
    prev2 = prev1;
    prev1 = x;
    n -= 1;
   }

   prev1

}
