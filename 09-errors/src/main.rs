use std::{fs::{File, self}, io::{ErrorKind, self, Read}};

fn main() {
    // let vec = vec![1,2,3];

    // panics
    // vec[6];

    let f = File::open("Hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // unwrap_or_else in combination with closures can be used to write the same
    // code above but less verbose

    // unwrap can be used on result but panics on error

    // except can be used to handle an error explicitly 
}

// this function uses error propagation to return the 
// error back to the caller (or the result if any)
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// same functionality as above but uses the ? operator
fn read_username_from_file_question_mark_operator() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// using the standard library to read a file
fn read_username_from_file_std() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// creating a customs struct to validate the correctness of a guess
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}