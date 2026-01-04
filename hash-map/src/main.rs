use std::collections::HashMap;

fn main() {
    // Create a HashMap to store scores
    let mut scores = HashMap::new();
    scores.insert("Alice", 50);
    scores.insert("Bob", 30);

    // Insert entries using String references
    let binding = String::from("Charlie");
    // The type of `binding` is `String`, but we are inserting a `&String` reference
    scores.insert(&binding, 40);
    println!("{:?}", scores);

    let team_name = String::from("Blue");
    scores.insert(&team_name, 70);
    println!("{:?}", scores);

    scores.insert("Diana", 60);

    // print all scores in the HashMap loop
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Final state of the HashMap
    println!("{scores:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // Count the occurrences of each word
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
