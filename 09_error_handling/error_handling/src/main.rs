use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // Trying and calling panic on failure

    // Using unwrap(): Most basic
    let greeting_file_result = File::open("hello.txt").unwrap();

    // Using expect(): Much the same as unwrap() but we define the
    // error message sent to panic!()
    let greeting_file_result =
        File::open("hello.txt").expect("hello.txt should be included in this project!");

    // Using match: This is the most complete example
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },

            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
