fn main() {
    let s = String::from("hello world");
    let idx = first_word(&s);
    println!("the index of the first word is {}" , idx);

    // problem here is that after a call to s.clear(), we would be holding incorrect position of
    // the end of the first string
    // instead we can use string slices to maintain the first word, rather than the index (which
    // leads to out of bounds in the case of the string being erased)

    let word = slice_word(&s); // now if we try to call s.clear() after this we get error (mutable
                               // ref and immutable ref
    let literal_word = slice_word("literal string");
    println!("The first word of the string is {}, we also passed a literal string and its first word is {}", word, literal_word);

}

fn first_word(s : &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// slice word takes in &str as parameter to be more generic rather than &String
// since slices of String are of type &str, and also string literal are also slices
fn slice_word(s : &str) -> &str {
    let bytes = s.as_bytes();
    
    for(i , &item) in bytes.iter().enumerate() {
        if item ==  b' ' {
            return &s[0..i] 
        }
    }

    &s[..]
}
