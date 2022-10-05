fn main() {
    let mut s = String::from("Hello");
    // mutating String
    s.push_str(", World");
    let s2 = s; // s2 is now the owner (this allows rust to avoid double memory freeing)
    println!("{}", s2);
}
