use std::fs::remove_file;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};


enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    // Cause an instant kernel panic. Fun.

    // panic!("A man stands above the city he will destroy.")

    let v = vec![1, 2, 3];

    // Gonna fuck this kernel up.

    //v[99];

    // Note if you want to see a backtrace use the command RUST_BACKTRACE=1 cargo run. 
    // It won't stick past that commmand


    // Primitive but workable way of error handling using match

    println!("Testing error handling with hello.txt file...");
    let greeting_file_text = File::open("hello.txt");

    let greeting_file = match greeting_file_text {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    println!("Created/Opened file.");

    remove_file("hello.txt");

    println!("Deleted file, trying again w/ better error handling...");

    let greeting_file = File::open("hello.txt").unwrap().expect("hello.txt is not present, please add it!");



}


fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
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