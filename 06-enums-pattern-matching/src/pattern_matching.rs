use std::fmt::Debug;

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    // we can enhance this variant with additional data
    Quarter(UsState),
}
fn main() {
    let dime = Coin::Dime;
    println!(
        "The value of the coin {:?} is {}",
        &dime,
        value_in_cents(&dime)
    );

    // now we need to provide the data to the variant
    let quarter = Coin::Quarter(UsState::Alabama);
    println!(
        "The value of the coin {:?} is {}",
        &quarter,
        value_in_cents(&quarter)
    );

    let num: Option<i32> = Some(4);
    let absence_num: Option<i32> = None;

    eval_option(num);
    eval_option(absence_num);

    let x = Some(1);
    let res = plus_one(plus_one(plus_one(x)));
    println!(
        "Added three time one to {} makes {}",
        // the real value of the option can be retrieved with
        // the unwrap function 
        x.unwrap(),
        res.unwrap()
    );

    // this code panics because we can't unwrap a none value
    // let none = plus_one(None);
    // println!("Oops {}", none.unwrap());

    catch_all_patterns(3);
    catch_all_patterns(9);

    // does nothing because we just ignore the 25
    ignore_cases(25);
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // because we defined data for the variant we need to match the pattern
        // in order to use it, we define a variable for the UsState that we
        // provided in the initial enum definition to access its value
        // the data is also mandatory, we can't create a Quarter without
        // a valid UsState
        Coin::Quarter(state) => {
            println!("State Quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn eval_option<T: Debug>(option: Option<T>) {
    match option {
        Some(value) => println!("The value is {:?}", value),
        None => println!("There is no value!"),
    }
}

// we can assign a default case to the match statement
// if no other value matches this branch is used
// order is important, the default case must be the last one
// if we do not want to use the value of a default match
// we can use the _ variable to tell rust that we want
// to ignore that value, rust won't complain about unused
// variables in this case
fn catch_all_patterns(dice_roll: u8) {
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {
        println!("Add fancy hat");
    }
    fn remove_fancy_hat() {
        println!("Remove fancy hat");
    }
    fn move_player(num_spaces: u8) {
        println!("Move player {} spaces", num_spaces);
    }
}

// if we do not want to use the value of a default match
// we can use the _ variable to tell rust that we want
// to ignore that value, rust won't complain about unused
// variables in this case
// further we can use the unit tuple type to completely ignore
// all cases that we do not want to handle
fn ignore_cases(dice_roll: u8) {
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {
        println!("Add fancy hat");
    }
    fn remove_fancy_hat() {
        println!("Remove fancy hat");
    }
    fn move_player(num_spaces: u8) {
        println!("Move player {} spaces", num_spaces);
    }
}