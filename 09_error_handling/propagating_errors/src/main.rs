use std::fs::{self, File};
use std::io::{self, Read};

fn main() {
    println!("Hello, world!");
}

fn read_username_from_file() -> Result<String, io::Error> {
    // Propagating errors the hard way..
    let username_file_result = File::open("users.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_question_mark() -> Result<String, io::Error> {
    // Propagating errors useing the question mark operator
    //
    // A key difference is that calling ? will call the "from" function on the Error
    // converting it to the error type defined by the signature
    let mut username_file = File::open("users.txt")?;

    let mut username = String::new();
    username_file.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_question_mark_chained() -> Result<String, io::Error> {
    // Propagating errors useing the question mark operator
    //
    // A key difference is that calling ? will call the "from" function on the Error
    // converting it to the error type defined by the signature
    let mut username = String::new();
    File::open("users.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    // This hides all the error handling but is very short
    // The standard library includes a convience method to do the above
    fs::read_to_string("users.txt")
}
