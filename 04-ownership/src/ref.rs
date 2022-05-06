fn main() {
    let string = String::from("hello");

    let length = calc_length(&string);

    println!("{}", length);

    // non mutable reference cannot be modified
    // mutable references can
    let mut string2 = String::from("world");

    add_punctuation(&mut string2);

    // there can only be one mutable reference to one piece of data
    let mut s = String::from("hello");

    // we can't make reference here because we already have a mutable reference in this scope

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

fn calc_length(s: &String) -> usize {
    s.len()
}

fn add_punctuation(s: &mut String) {
    s.push_str("!")
}
