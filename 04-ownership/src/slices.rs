fn main () {
    // Slices are partial references to data

    let s = String::from("Hello World");

    let hello = &s[..5];
    let world = &s[6..];
    
    println!("{} {}", hello, world);   
    println!("The first word of \"{}\" is {}", s, first_word(&s));

    let my_literal = "hello";

    first_word(&my_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}