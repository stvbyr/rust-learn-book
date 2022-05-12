// importing symbols with same name from different packages
use std::fmt;
use std::io;
// using aliasing
use std::array as Array;
// using nested paths
use std::{alloc, arch::asm};

// fn ex1() -> io::Result<()>{}
// fn ex2() -> fmt::Result{}
// fn ex3() -> Array{}

mod back_of_house;
mod front_of_house;

fn serve_order() {}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // full path to function
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
