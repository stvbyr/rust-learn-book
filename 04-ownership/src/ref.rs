fn main() {
    
    /*
    ----------------------------------------------------------
    References
    ----------------------------------------------------------
    */

    let string = String::from("hello");

    let length = calc_length(&string);

    println!("{}", length);

    
    /*
    ----------------------------------------------------------
    Mutable References
    ----------------------------------------------------------
    */

    // non mutable reference cannot be modified
    // mutable references can
    let mut string2 = String::from("world");
    
    add_punctuation(&mut string2);

    /*
    Multiple mutable references to the same value (is not allowed)
    */

    // there can only be one mutable reference to one piece of data
    let mut s = String::from("hello");

    let r0 = &mut s;

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    // we can't make reference here because we already have a mutable reference in this scope (r0)
    // let r2 = &mut s;


}

fn calc_length(s: &String) -> usize {
    s.len()
}

fn add_punctuation(s: &mut String) {
    s.push_str("!")
}
