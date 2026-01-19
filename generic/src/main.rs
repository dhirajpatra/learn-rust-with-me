use std::collections::HashMap;
use std::fs;

// Importing the necessary items from the generic module lib.rs
use generic::{NewsArticle, SocialPost, Summary};

// Result<T, E> - The "Success or Error" type
// Function to read file contents and return Result or error
fn read_file_contents(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = largest(&number_list);
    println!("The largest number is {}", largest);

    let largest_i32 = largest_i32(&number_list);
    println!("The largest i32 number is {}", largest_i32);

    // Create an instance of SocialPost
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", post.summarize());

    // Create an instance of NewsArticle
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("K. R."),
        content: String::from("The Pittsburgh Penguins are the best hockey team in the NHL."),
    };
    println!("New article: {}", article.summarize());

    // Options demonstration
    let student1 = find_student(1);
    let student2 = find_student(2);

    // Pattern matching to handle both cases
    match student1 {
        Some(name) => println!("Found student: {}", name),
        None => println!("Student not found"),
    }

    // Shorter way using unwrap_or
    println!("Student 2: {}", student2.unwrap_or("Not found".to_string()));

    // Vec<T> Dynamic Array as collection demonstration
    // Create a shopping list
    let mut shopping_list = Vec::new();

    // Add items
    shopping_list.push("Milk");
    shopping_list.push("Eggs");
    shopping_list.push("Bread");

    // Access items
    println!("First item: {}", shopping_list[0]); // Milk

    // Safe access with get() returns Option even if index is out of bounds
    match shopping_list.get(5) {
        Some(item) => println!("Item: {}", item),
        None => println!("No item at index 5!"),
    }

    // HashMap<K, V> - Key-Value Store demonstration
    // Create a phone book
    let mut phone_book = HashMap::new();

    // Insert entries
    phone_book.insert("Alice", "123-4567");
    phone_book.insert("Bob", "987-6543");

    // Look up a number - returns Option
    // let s: &str = "hello"; let ref_s = &s; // ref_s is &&str
    let alice_number = phone_book.get("Alice");

    match alice_number {
        // Pattern match to handle Some and None
        Some(number) => println!("Alice's number: {}", number),
        None => println!("Alice not in phone book"),
    }

    // Charlie doesn't exist
    println!("Charlie: {:?}", phone_book.get("Charlie")); // None

    // Try to read a file that exists
    match read_file_contents("hello.txt") {
        // Pattern match to handle Ok and Err
        Ok(contents) => println!("File contents: {}", contents),
        Err(error) => println!("Error reading file: {}", error),
    }

    // Try to read a file that doesn't exist
    match read_file_contents("missing.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(error) => println!("Error: {}", error), // This will run
    }

    // Shorter way using unwrap_or_else
    let contents =
        read_file_contents("hello.txt").unwrap_or_else(|_| "Could not read file".to_string());

    println!("File contents using unwrap_or_else: {}", contents);

    // Different ways to handle absence of value
    // Option: "I might have a value or not"
    fn divide(a: f64, b: f64) -> Option<f64> {
        if b == 0.0 {
            None // Can't divide by zero
        } else {
            Some(a / b)
        }
    }

    // Result: "I'll try to do this, but might fail with a reason"
    fn parse_number(s: &str) -> Result<i32, String> {
        s.parse().map_err(|_| format!("'{}' is not a number!", s))
    }

    // Using divide function
    match divide(10.0, 2.0) {
        Some(result) => println!("10 / 2 = {}", result),
        None => println!("Cannot divide by zero"),
    }
    match divide(5.0, 0.0) {
        Some(result) => println!("5 / 0 = {}", result),
        // it will fail as division by zero
        None => println!("Cannot divide by zero"),
    }

    // Using parse_number function
    match parse_number("42") {
        Ok(num) => println!("Parsed number: {}", num),
        Err(err) => println!("Error: {}", err),
    }
}

// Non-generic function to find the largest i32 in a list
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list.iter() {
        // Compare each item with the current largest item which are both &i32 references
        if *item > *largest {
            largest = item;
        }
    }
    largest
}

// Generic function to find the largest element in a list Parameterized over type T
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Function that returns a type implementing the Summary trait
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         SocialPost {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     } else {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("K. R."),
//             content: String::from("The Pittsburgh Penguins are the best hockey team in the NHL."),
//         }
//     }
// }

// Simple function to demonstrate Option
// Example: Finding a student in a class
fn find_student(roll_number: i32) -> Option<String> {
    // Option can hold either Some value or None
    if roll_number == 1 {
        Some(String::from("Alice")) // Found Alice
    } else {
        None // Student not found
    }
}
