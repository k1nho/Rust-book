// Children can access the context of parents, but parents cannot.
mod front_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn start_restaurant() {
    crate::front_house::hosting::add_to_waitlist();
    front_house::hosting::add_to_waitlist();
}

