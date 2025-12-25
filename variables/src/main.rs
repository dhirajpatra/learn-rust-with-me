fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    let y = 10;
    println!("The value of y is: {}", y);

    let sum = x + y;
    println!("The sum of {} and {} is {}", x, y, sum);

    x = 15; // This line will cause a compile-time error because x is immutable
    println!("The new value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {}", THREE_HOURS_IN_SECONDS);

    // Shadowing example
    let z = 7;
    let z = z + 3; // z is now 10 by redefining it
    println!("The value of z after shadowing is: {}", z);

    // let mut space = "   ";
    // println!("Space before trimming: '{}'", space);
    // space = space.len(); // This line will cause a compile-time error because space is a &str
    // println!("Length of space after shadowing: {}", space);

    let x = 2.0;
    let y: f32 = 3.0;

    // addition
    let sum = x + y;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!(
        "Sum: {}, Difference: {}, Product: {}, Quotient: {}, Truncated: {}, Remainder: {}",
        sum, difference, product, quotient, truncated, remainder
    );

    let t = true;
    let f: bool = false;
    println!("Boolean values: t = {}, f = {}", t, f);

    let c = 'z';
    let z: char = 'z';
    let heart_eyed_cat = '😻';
    println!("Characters: {}, {}, {}", c, z, heart_eyed_cat);
    println!("c == z: {}", c == z);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_a, _b, _c) = tup;
    println!("The value of _a, _b, _c are: {}, {}, {}", _a, _b, _c);

    let tup2 = (500, 6.4, 1);

    let (x, y, z) = tup2;
    println!("The value of x, y, z are: {}, {}, {}", x, y, z);
}
