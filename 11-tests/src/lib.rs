use std::{io::Read, thread::panicking};

pub mod math {
    pub fn add(left: usize, right: usize) -> usize {
        left + right
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        if width < 2 || height < 2 {
            panic!("can not create Rectangle with width < 2");
        }
        Self { width, height }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    // use super::*
    use crate::Rectangle;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };

        let smaller = Rectangle {
            width: 2,
            height: 3,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    #[should_panic(expected = "can not create Rectangle with width < 2")]
    fn rectangle_cannot_be_smaller_than_1() {
        Rectangle::new(1, 1);
    }
}
