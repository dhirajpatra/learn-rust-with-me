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
}
