
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
    let f1 =  1.5; // f64 by default
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

}
