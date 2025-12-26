fn main() {
    let s = String::from("hello world");
    let word_index = first_word_index(&s);
    // s.clear(); // this empties the String, making it equal to ""
    // but also invalidates word_index because there are no more words!
    println!("The first word ends at index: {}", word_index);

    let hello = &s[0..5];
    let len = s.len();
    let world = &s[6..len];
    println!("{} {}", hello, world);

    // using shorthand for full range
    let slice = &s[..];
    println!("{}", slice);

    let first_word_slice = first_word(&s);
    println!("The first word is: {}", first_word_slice);

    let word = first_word(&s[0..6]);
    println!("The first word in the first six characters is: {}", word);
}

// function that returns the index of the first word in a string slice
fn first_word_index(s: &str) -> usize {
    let bytes = s.as_bytes();

    // iterate over the byte array with indexes enumerated
    for (i, &item) in bytes.iter().enumerate() {
        // check for space character
        if item == b' ' {
            return i;
        }
    }

    s.len() // if no space is found, return the length of the string no ; required on last line
}

// function that returns the first word as a string slice
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
