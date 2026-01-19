fn main() {
    let r;

    // outer scope
    {
        let x = 5;
        // an attempt to assign a reference to x to r whose value has gone out of scope
        r = &x; // r borrows x
        println!("r: {}", r);
    }

    // println!("r: {}", r); // This would fail because x is out of scope

    let string1 = String::from("abcd");
    // This would fail because string2 does not live long enough
    // let string2 = "xyz";
    // let result = longest(string1.as_str(), string2); // error!
    {
        // inner scope
        let string2 = String::from("xyz");
        // now both string1 and string2 are valid here
        let result = longest(string1.as_str(), string2.as_str());

        println!("The longest string is {}", result);
    }

    // a reference with explicit lifetime annotation
    let a = 5;
    // 'a is the lifetime annotation
    let b: &i32 = &a;
    // 'a lives at least as long as b
    let c: &mut i32 = &mut 6;

    println!("a, b, c: {}, {}, {}", a, b, c);

    println!("First word: {}", first_word(&string1));
}

// This function won't compile because it returns a reference to one of its parameters
// without specifying lifetimes, which is required in this case.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// A function that returns the first word in a string slice
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
