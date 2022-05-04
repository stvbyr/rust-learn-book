
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
    // let mut x = 5;
    // println!("{}", x); // 5
    // x = 6;
    // println!("{}", x); // 6

    /*
    Constants
    */

    // // - are immutable
    // // - must have a type annotation
    // // - can only use constant expressions as values, not computed ones
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("{}", THREE_HOURS_IN_SECONDS); // 10800

    /*
    Shadowing
    */
    // let x = 5;
    // let x = x + 1; // shadowed 
    // {
    //     let x = x * 2; // shadowed but only in this scope
    //     println!("{}", x); // 12
    // }
    // println!("{}", x); // 6
}
