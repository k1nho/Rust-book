#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red, 
    Blue,
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
}
