fn main() {
    /*
    ------------------------------------------------------------------
    Variables and Mutability
    ------------------------------------------------------------------
    */

    /*
    Variables
    */

    // // will throw error because x is immutable by default
    // let x = 5;
    // println!("{}", x); // 5
    // x = 6;
    // println!("{}", x); // error

    // // this will because x is explicitly mutable
    let mut a = 5;
    println!("{}", a); // 5
    a = 6;
    println!("{}", a); // 6

    /*
    Constants
    */

    // // - are immutable
    // // - must have a type annotation
    // // - can only use constant expressions as values, not computed ones
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECONDS); // 10800

    /*
    Shadowing
    */
    let x = 5;
    let x = x + 1; // shadowed
                   // block scope
    {
        let x = x * 2; // shadowed but only in this scope
        println!("{}", x); // 12

        // end of scope x = 6 again
    }
    println!("{}", x); // 6

    /*
    ------------------------------------------------------------------
    Data Types
    ------------------------------------------------------------------
    */

    /*
    Scalar Types
    */

    // Integer Types
    // In signed and unsigned form
    // 8 - 128 bit + arch sized
    // default inferred to i32
    let num_signed: i8 = 127;
    println!("{}", num_signed);
    let num_unsigned: u8 = 255;
    println!("{}", num_unsigned);

    // Literals
    let dec = 98_000u128; // uses a visual separator and a suffix for the type
    let hex = 0xfffffff;
    let oct = 0o777777;
    let bin = 0b1001001;
    let byte = b'A';

    println!("{} {} {} {} {}", dec, hex, oct, bin, byte);

    // Floating-Point Types
    let f1 = 1.5; // f64 by default
    let f2: f32 = 4.0; // integer that should be float must be written with a dot
    let f3 = 4.7f64; // with suffix

    println!("{} {} {}", f1, f2, f3);

    // Numeric Operations
    let sum = 4 + 6;
    // operations can only be done on equal types the following doesn't work
    // let diff = 50 - 4.5;
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    let remainder = 43 % 5;

    println!("{} {} {} {}0", sum, quotient, floored, remainder);

    // Character Type
    let c: char = 'h'; // denoted with single quotes

    println!("{}", c);

    // Compound Types

    // Tuples
    let tup: (i32, f64, String) = (42, 3.14, "Hello".to_string());

    // values can be retrieved with destructuring or dot notation;
    let (x1, x2, _) = tup;
    let x3 = tup.2;

    println!("{} {} {}", x1, x2, x3);

    // unit type
    // special type that is an empty tuple
    // expressions implicitly return the unit value if they don’t return any other value.
    let unit = ();

    println!("{:?}", unit);

    // Arrays

    // similar to a tuple but all values need to be the same type
    // they are fixed in length
    let arr = [1, 2, 3, 4, 5];
    let arr_str: [String; 2] = ["Hello".to_string(), "World".to_string()];

    println!("{:?}, {:?}", arr, arr_str);

    // array of values where all values are the same
    let arr_repeat = [1; 5];
    println!("{:?}", arr_repeat);

    // array access with dot and destructuring
    let a1 = arr[0];
    let [_, a2, a3, _, _] = arr;

    println!("{:?} {:?} {:?}", a1, a2, a3);

    /*
    Functions
    */

    println!("{:?}", add(30, -15)); // 15
    println!("{:?}", sub(30, -15)); // 45
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn sub(a: i32, b: i32) -> i32 {
    a - b // implicit return
}