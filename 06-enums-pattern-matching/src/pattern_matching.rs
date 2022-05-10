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
