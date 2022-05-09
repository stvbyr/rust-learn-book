#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 30;
    
    let rect = Rectangle { 
        width: width1, 
        height: height1 
    };

    // println!("The area of rect is {} square pixels.", area(width1, height1));
    // println!("The area of rect is {} square pixels.", area((width1, height1)));
    println!("The area of rect is {} square pixels.", area(&rect));
    println!("{:#?}", rect);

    // debugging with dbg! macro
    // takes ownership and gives it back
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(2 * scale),
        height: 10
    };

    dbg!(&rect2);
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     let (width, height) = dimensions;

//     width * height
// }

fn area(rect: &Rectangle) -> u32 {
    let Rectangle { width, height } = rect;

    width * height
}