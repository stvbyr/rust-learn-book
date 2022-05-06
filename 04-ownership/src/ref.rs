fn main() {
    let string = String::from("hello");

    let length = calc_length(&string);

    println!("{}", length);

    // non mutable reference cannot be modified 
    // mutable references can
    let mut string2 = String::from("world");

    // there can only be one mutable reference to one piece of data
    add_punctuation(&mut string2);
}

fn calc_length(s: &String) -> usize{
    s.len()
}

fn add_punctuation(s: &mut String) {
    s.push_str("!")
}