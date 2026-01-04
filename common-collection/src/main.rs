enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // Using an external crate named `v` that provides a vector type
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    // generic type from another crate named `a`
    let mut a = vec![1, 2];
    println!("{:?}", a);

    a.push(3);
    println!("{:?}", a);

    // Accessing elements in the vector
    let third: &i32 = &a[2];
    println!("The third element is {}", third);

    // Using get method to safely access elements
    let third: Option<&i32> = a.get(3);
    // Handling the Option type
    match third {
        Some(value) => println!("The fourth element is {}", value),
        None => println!("There is no fourth element."),
    }

    // Iterating over the elements in the vector
    for i in &a {
        println!("{}", i);
    }

    // Modifying elements in the vector
    for i in &mut a {
        *i += 50;
        println!("After adding 50 becomes {}", i);
    }

    // Creating a vector of SpreadsheetCell enum instances
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Iterating over the SpreadsheetCell vector and matching on each variant
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(value) => println!("Integer value: {}", value),
            SpreadsheetCell::Float(value) => println!("Float value: {}", value),
            SpreadsheetCell::Text(value) => println!("Text value: {}", value),
        }
    }

    // UTF-8 string handling
    let mut hello = String::from("नमस्ते");
    hello.push('!');
    for c in hello.chars() {
        println!("{}", c);
    }

    let s = &hello[0..3];
    println!("Slice of the string: {}", s);
    println!("Length of the string in bytes: {}", hello.len());
    println!(
        "Number of characters in the string: {}",
        hello.bytes().count()
    );
}
