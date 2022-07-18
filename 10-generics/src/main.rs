// impl block can be used to implement a function for the whole type
// or for a generic version
// or for a specific type, all other types will not have this function available

use std::fmt::{Display, Debug};

use crate::lifetimes::longest;

mod lifetimes;

fn main() {
    // generic types represent a set types (what),
    // traits represent a set of behaviors (how) in a generic way
    // both can be combined to represent generic behavior on generic types

    // code that can be extracted to a function
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
    println!(
        "The largest number is {} (function call)",
        largest_fun(&vec![49, 23, 42, 69])
    );

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let slice = [1,2,3];
    println!("{}", largest_generic(&slice));
    println!("{:?}", slice[2]);
    println!("{}", largest_no_heap(&slice));
    println!("{:?}", slice[2]);


    println!("longest is {}", longest("asc", "gbhd"));
}

fn largest_fun(number_list: &[i32]) -> i32 {
    let mut largest = number_list[0];

    for &number in number_list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn largest_generic<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_no_heap<T: PartialOrd>(list: &[T]) -> &T {
    let mut i = 0;

    for (index, item) in list.iter().enumerate() {
        if item > &list[i] {
            i = index;
        }
    }

    &list[i]
}

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// traits

// traits are most similar to interfaces but are actually more
// traits represent behavior

pub trait Summary {
    // a default implementation that all implementors can use out of the box
    fn summarize(&self) -> String {
        format!("read more from {}", self.summarize_author())
    }
    
    // a function that the type having this trait must implement
    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} {}", self.headline, self.summarize_author(), self.location)
    }

    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// with trait bound a trait can be required by functions
fn notify(sum: &impl Summary) {
    println!("Summary: {}", sum.summarize())
}

// same as above
// this is the longer version and does exactly the same
fn notify2<T: Summary>(sum: &T) {
    
}


// multiple traits must be implemented
fn notify_and_display(item: &(impl Summary + Display)) {}
fn notify_and_display2<T: Summary + Display>(item: &T) {}

// where clauses for more complex trait bounds
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}
fn some_function2<T, U>(t: &T, u: &U)
    where T: Display + Clone,
          U: Clone + Debug
{}

// traits as return type
fn return_sum() -> impl Summary {
    Tweet {
        content: String::from("Stuff"),
        username: String::from("Stuff"),
        reply: false,
        retweet: false,
    }
}


