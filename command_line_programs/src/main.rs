use std::env;
use std::fs;

fn main() {
    // collect the arguments into a vector of strings
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    // dbg!(args);

    let query = &args[1];
    let file_path = &args[2];

    println!("Query: {}", query);
    println!("File Path: {}", file_path);

    // read the file
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{}", contents);
}
