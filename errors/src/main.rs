use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };

    // unwrap, is a shortcut method that will return the Result-object if Ok, and will panic! if Err.
    let f = File::open("hello.txt").unwrap();

    // Expect is like unwrap, except we can choose the panic-message
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// This function will propegate the error to the caller
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

// This function has the same functionality as the above
// if Err is returned from a Result-call with a question mark
// behind it, the caller will return/propagate the Err object
// This is fucking awesome
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

//There is a difference between what the match expressions and ? do: error values taken by ? go through the from function, defined in the From trait in the standard library, This will convert from one error to the Error which will be returned from the calling-function

// You can even chain ? for even shorter calls:
fn even_shorter_read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// Of course, there's already a convenience function in the std-lib for this:
fn std_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

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

fn take_guess(guess: &Guess) -> i32 {
    guess.value()
}
