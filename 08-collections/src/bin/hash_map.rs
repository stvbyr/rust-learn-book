use std::collections::HashMap;

fn main() {
    let blue = String::from("blue");
    let yellow = String::from("yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    // reading from the hashmap with defaults
    let entry = scores.get(&String::from("yellow"));
    match entry {
        Some(value) => println!("The value for yellow is {}", value),
        None => (),
    }

    // updating entries
    let entry = scores.entry("yellow".to_string());
    let entry = entry.and_modify(|s| *s = 42);

    println!("The value for yellow is {:?}", entry);
    println!(
        "The value for yellow is {:?}",
        scores.get(&"yellow".to_string()).unwrap_or(&0)
    );

    // if the key does not exist we can set a default to it
    scores.entry(String::from("red")).or_insert(255);

    println!(
        "{:?}",
        count_words_in_string("Hello this is a Hello World program")
    );
}

fn count_words_in_string(string: &str) -> HashMap<&str, i32> {
    let mut map = HashMap::new();

    for word in string.split_whitespace() {
        let count = map.entry(word).or_default();
        *count += 1;
    }

    map
}
