/* A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference. */
fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1); // passing reference of s1 to the function
    println!("The length of '{}' is {}.", s1, len);

    // change function modifies the string via mutable reference
    change(&mut s1);
    println!("After change: {}", s1);

    let r1 = &s1; // no problem
    let r2 = &s1; // no problem
    println!("{} and {}", r1, r2);

    let r3 = &mut s1; // no problem
    // let r4 = &mut s1; // BIG PROBLEM - cannot borrow `s1` as mutable more than once at a time
    println!("{}", r3);

    {
        let r4 = &mut s1; // no problem - new scope
        println!("{}", r4);
    }

    let r5 = &s1; // no problem - new reference after mutable reference goes out of scope
    println!("{}", r5);

    let r6 = &s1; // no problem
    // let r7 = &mut s1; // BIG PROBLEM - cannot borrow `s1` as mutable because it is also borrowed as immutable
    // println!("{}, {}", r6, r7);

    let r8 = &s1; // no problem
    let r9 = &s1; // no problem
    println!("{}, {}", r8, r9);

    let r10 = &mut s1; // no problem
    println!("{}", r10);

    // dangling reference example
    // let reference_to_nothing = dangle(); // this line will cause a compile-time error
    let reference_to_nothing = no_dangle(); // this is fine
    println!("{}", reference_to_nothing);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// function that modifies the string via mutable reference
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// dangling reference example
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // return the String, not a reference to it
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s // return the String itself
}
