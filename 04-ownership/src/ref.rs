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

    // we also can't mix immutable and mutable references at the same time
    // if the usage of the immutable reference comes after the mutable one
    //
    // let mut s = String::from("hello");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    //
    //let mut s = String::from("hello");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    // let r3 = &mut s; // no problem

    /*
    Dangling References
    */
    // fn dangle() -> &String { // dangle returns a reference to a String

    //     let s = String::from("hello"); // s is a new String
    
    //     &s // we return a reference to the String, s
    // } // Here, s goes out of scope, and is dropped. Its memory goes away.
    //   // Danger!
}

fn calc_length(s: &String) -> usize {
    s.len()
}

fn add_punctuation(s: &mut String) {
    s.push_str("!")
}
