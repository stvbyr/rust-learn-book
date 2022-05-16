use std::collections::HashMap;

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // Given a list of integers, use a vector and return the median (when
    // sorted, the value in the middle position) and mode (the value that occurs
    // most often; a hash map will be helpful here) of the list.

    let list_of_int = vec![
        1, 1, 9, 15, 8, 8, 8, 7, 4, 3, 7, 6, 5, 3, 1, 1, 32, 5, 3, 2, 2, 2, 4, 2, 4, 9, 9, 9, 9, 9,
        9, 9,
    ];

    println!("Median is {}", median(list_of_int.clone()));
    println!("Mode is {}", mode(&list_of_int));

    fn median(mut list: Vec<i32>) -> i32 {
        list.sort();

        list[list.len() / 2]
    }

    fn mode(list: &Vec<i32>) -> i32 {
        let mut num_counters: HashMap<i32, i32> = HashMap::new();

        for num in list {
            let counter = num_counters.entry(*num).or_default();

            *counter += 1;
        }

        let biggest_value = num_counters.iter().max_by_key(|&(_, counter)| counter);

        match biggest_value {
            Some((int, _)) => *int,
            None => panic!("There is no mode for 0 elements"),
        }
    }

    // Convert strings to pig latin. The first consonant of each word is moved
    // to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
    // Words that start with a vowel have “hay” added to the end instead
    // (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8
    // encoding!

    assert_eq!(pig_latin("first"), "irst-fay");
    assert_eq!(pig_latin("apple"), "apple-hay");
    assert_eq!(pig_latin("ölll"), "lll-öay");

    fn pig_latin(string: &str) -> String {
        let vowels = ["a", "e", "i", "o", "u"];

        let mut graphemes = string.graphemes(true);
        let first_char = graphemes.clone().next().unwrap();
        let is_first_char_vowel = vowels.contains(&first_char);

        if is_first_char_vowel {
            format!("{}{}", string, "-hay")
        } else {
            graphemes.next();

            format!("{}{}{}{}", graphemes.as_str(), "-", first_char, "ay")
        }
    }

    // Using a hash map and vectors, create a text interface to allow a user to
    // add employee names to a department in a company. For example, “Add Sally
    // to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list
    // of all people in a department or all people in the company by department,
    // sorted alphabetically.
}
