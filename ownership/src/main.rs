fn main() {
    let mut x = String::from("Hello,");
    x.push_str(" world!");
    println!("{}", x);

    {
        // let y = &x; // it will fail as ownership of x is not moved
        let y = x; // ownership of x is moved to y
        println!("{}", y);
    }

    // out of scope of y, x is no longer valid
    let s1 = String::from("hello");
    let s2 = s1; // ownership of s1 is moved to s2
                 // println!("{s1}, world!"); // this line will cause a compile-time error
    println!("{s2}, world!");

    // scope and assignment
    // let s3 = String::from("hello");
    // s3 = String::from("world"); // this line will cause a compile-time error
    // println!("{s3}, world!");

    // deep copy using clone
    let s4 = String::from("hello");
    let s5 = s4.clone(); // deep copy of s4 is made to s5
    println!("s4 = {}, s5 = {}", s4, s5);

    // copy trait or stack only data copy
    let x = 5;
    let y = x; // copy of x is made to y and x is still valid not moved to y
    println!("x = {}, y = {}", x, y);

    // ownership with functions
    let s6 = give_ownership(); // ownership of the returned String is moved to s6
    println!("{}", s6);

    let s7 = String::from("takes and gives back");
    let s8 = takes_and_gives_back(s7); // ownership of s7 is moved to the function and then back to s8
                                       // println!("{}", s7); // this line will cause a compile-time error
    println!("{}", s8);

    let s9 = String::from("calculate length");
    let len = calculate_length(&s9); // passing reference of s9 to the function
    println!("The length of '{}' is {}.", s9, len);
}

fn give_ownership() -> String {
    let some_string = String::from("gives ownership");
    some_string // ownership of some_string is moved to the caller
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // ownership of a_string is moved back to the caller
}

// function that calculates length of a string without taking ownership
fn calculate_length(s: &String) -> usize {
    s.len()
}
