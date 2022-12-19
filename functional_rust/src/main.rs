#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red, 
    Blue,
}


#[derive(Debug)]
enum ShirtSize {
    S, 
    M,
    L, 
    XL,
}

#[derive(Debug)]
struct Shirt {
    size: ShirtSize,
    color: ShirtColor,
}

struct Inventory {
    shirts : Vec<ShirtColor>
}

impl Inventory {
    fn giveaway(&self, user_preferance: Option<ShirtColor>) -> ShirtColor {
        user_preferance.unwrap_or_else(|| self.most_stocked())
    }
    
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

            if num_red > num_blue {
                ShirtColor::Red
            } else {
                ShirtColor::Blue
            }
        }
}


fn main() {
   let inventory = Inventory{shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue, ShirtColor::Blue]};

    let user1_pref = Some(ShirtColor::Blue);
    let user2_pref = Some(ShirtColor::Red); 
    let user3_pref = None;
    println!("User 1 has preferance {:?} and obtains {:?}", user1_pref, inventory.giveaway(user1_pref));
    println!("User 2 has preferance {:?} and obtains {:?}", user2_pref, inventory.giveaway(user2_pref));
    println!("User 3 has no preferance and obtains {:?}", inventory.giveaway(user3_pref));

    // CLOSURES
    let add_one = |x: u32| -> u32 {x +1};
    let add_two = |x: u32| x+2;
    let mut v = vec![1,5,6];
    
    let num = 5;
    println!("The original number was {}, after calling closure add_one {}, after calling closure add_two {}", num, add_one(num), add_two(num));

    let mut mutate_vec = || v.push(10);
    mutate_vec();
    let read_vec = || println!("the list contains {:?}", v);
    read_vec();
    //ownership in closures
    //let own_vec =  move || println!("own_vec has now ownership of v {:?}", v);
    //own_vec();
    //
    
    /* Three traits that closures could implement
    * FnOnce: closures that can be called once
    * FnMut: don't move values out the body, but might mutate captured values
    * Fn: don't move captured values , don't mutate and don't capture anything from their environment 
    */
    
    // ITERATORS
    let v_iter = v.iter();

    //cosuming adaptors (methods that consume iterators)
    let sum : u32 = v_iter.sum();
    println!("we consumed the iterator into a sum {sum}");
    
    //iterator adaptors (methods that produce another iterator)
    let res : Vec<_> = v.iter().map(|x| x + 1).collect();
    println!("after using an iterator adaptor we have the following vector {:?}", res);

    let shirts = vec![Shirt{size: ShirtSize::S, color: ShirtColor::Red}, Shirt{size: ShirtSize::M, color: ShirtColor::Blue}, Shirt{size: ShirtSize::XL, color: ShirtColor::Blue}];
    
    // get the shirts that are only Blue
    let blue_shirts : Vec<_> = shirts.iter().filter(|shirt|  shirt.color == ShirtColor::Blue).collect();
    let red_shirts : Vec<_> = shirts.iter().filter(|shirt|  shirt.color == ShirtColor::Red).collect();
    dbg!(blue_shirts);
    dbg!(red_shirts);

}
