struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    // associative function are functions defined on a type
    // but they do not reference self
    // they are the closest to static methods in other oop languages
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

// multiple impl block are possible
// in this case there is not really good reason for it other than demonstration
impl Rectangle {
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };
    let rect2 = Rectangle::new(1, 2);
    let rect3 = Rectangle {
        width: 15,
        height: 2,
    };

    if rect1.width() {
        println!("The area of the rectangle is {}", rect1.area());
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
