#[derive(Debug)]
// we can attach data directly to a enum variant
// it looks a little bit like a constructor for this variant
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    println!("{:?} {:?}", home, loopback);

    // common use is a value that is something or nothing
    // other languages use null but this has it own implications
    // rust does not a have a null value but in rust 
    // this can be expressed with the Option enum
    let _some_number = Some(5);
    let _some_string = Some("a string");

    let _absent_number: Option<i32> = None;

    // Option is basically a wrapper around some type
    // that ensures that we don't have to worry about the
    // absence of a value until the point that we need to know
    // because it a wrapper we can't use an option like a valid 
    // value
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // will not work because rust doesn't know how to sum an i8 with an Option<i8>
    // let sum = x + y;
    
    
}
