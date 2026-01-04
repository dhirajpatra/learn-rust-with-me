use core::panic;
use std::fs::File;
use std::io::ErrorKind;

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
}
