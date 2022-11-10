fn main() {
    // Enum: A value is one of the possible values enumerated
    
    #[derive(Debug)]
    enum IpAddressKind {
        V4(String),
        V6(String),
    }

    let four = IpAddressKind::V4(String::from("127.0.0.0"));
    let six = IpAddressKind::V6(String::from("::1"));
    
    dbg!(four);
    dbg!(six);
    
}
