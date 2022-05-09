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
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    if rect1.width() {
        print!("The area of the rectangle is {}", rect1.area());
    }
}
