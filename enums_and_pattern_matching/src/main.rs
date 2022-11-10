fn main() {
    // Enum: A value is one of the possible values enumerated

    impl IpAddressKind {
        fn get_addr(&self) -> &str {
            "ip address"
        }
    }
    
    #[derive(Debug)]
    enum IpAddressKind {
        V4(String),
        V6(String),
    }

    let four = IpAddressKind::V4(String::from("127.0.0.0"));
    let six = IpAddressKind::V6(String::from("::1"));
   
    println!("This is the {}" , four.get_addr());
    dbg!(four);
    dbg!(six);

    // THE OPTION ENUM (a value could be something or it could be nothing)
    // PL design done right! avoid NULL references but maintain the concept
    // Key here is that Option<T> is not the same as T

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

   fn value_in_cents(coin : Coin) -> u8 {
       match coin {
           Coin::Penny => 1,
           Coin::Nickel => {println!("Nickel!"); 5},
           Coin::Dime => 10,
           Coin::Quarter => 25,
       }
   } 
    

   let nickel = Coin::Nickel;
   let nickel_val = value_in_cents(nickel);
   println!("The value of nickel is {}", nickel_val);

   // For cases where we do not need to match multiple cases but just one case we can use the if
   // let syntax
   let max_heat = Some(3u8);
   if let Some(max) = max_heat {
       println!("The max heat is {}", max);
   }
}
