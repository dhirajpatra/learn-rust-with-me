use core::panic;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::net::IpAddr;

fn main() {
    // panic!("This is a panic in abort mode");
    let v = vec![1, 2, 3];
    println!("{}", v[99]);

    // A simplified version of the Result enum for demonstration purposes
    let greeting_file_result = File::open("hello.txt");

    // A simplified version of the Result enum for demonstration purposes
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // Handle the error case
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("Problem opening the file: {:?}", error),
            _ => panic!("Problem opening the file: {:?}", error),
        },
    };
    print!("{:?}", greeting_file);

    // Using the read_username_from_file function
    let username_result = read_username_from_file();
    // Handle the Result using match
    match username_result {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading username: {:?}", e),
    }

    // Using the expect method to handle errors
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    println!("Home IP Address: {}", home);
}

// this function returns a Result type
fn read_username_from_file() -> Result<String, io::Error> {
    // the ? operator can be used to propagate errors
    let mut f = File::open("hello.txt")?;
    // create a new empty String
    let mut s = String::new();
    // read the file contents into the String
    f.read_to_string(&mut s)?;
    // return the String wrapped in an Ok variant
    Ok(s)
}
