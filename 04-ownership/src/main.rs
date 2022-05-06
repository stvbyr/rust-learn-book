fn main() {
    let mut s = String::from("hello");

    s.push_str(", world");

    println!("{}", s);

    /*
    ------------------------------------------------------------------
    Memory and allocation
    ------------------------------------------------------------------
    */

    /*
    Ways variables and data interact: MOVE
    */

    // both variables land on the stack because ints are primitive fixed sized
    // data types, furthermore y gets a copy of x and not a pointer to it
    let x = 5;
    let y = x;
    // we can print both variables as usual
    println!("{} {}", x, y);

    // now both variables are on the heap because String is not a primitive data type
    // furthermore str2 get a pointer to str1 instead of a copy
    let str1 = String::from("hello");
    let _str2 = str1;
    // the second we assign str1 to str2, str1 goes out of scope and is no longer usable
    // this is to ensure memory safety because if two variables point to the same data
    // the assignment of str1 to str2 is called a move because we do not (shallow) copy
    // the risk of freeing memory twice will cause errors
    // the following will not work
    // println!("{} {}", str1, str2);

    /*
    Ways variables and data interact: Clone
    */

    // to make the previous example work we have to clone the data
    // the data is then deep copied on the heap
    let str1 = String::from("hello");
    let str2 = str1.clone();
    println!("{} {}", str1, str2);

    /*
    Ownership and Functions
    */
    let string = String::from("hello"); // string comes into scope

    takes_ownership(string); // string's value moves into the function...
                             // ... and so is no longer valid here

    let num = 5; // x comes into scope

    makes_copy(num); // num would move into the function,
                     // but i32 is Copy, so it's okay to still
                     // use num afterward

    /*
    Return Values and Scopes
    */
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it
    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
