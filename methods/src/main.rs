#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementing methods for the Rectangle struct
impl Rectangle {
    // Method to calculate the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method to determine if one rectangle can hold another
    // The &self is actually syntactic sugar for self: &Self
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn width(&self) -> bool {
        self.width > 0 // no semicolon here because we want to return the value
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("Area of rect1: {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    if rect1.width() {
        println!("The rectangle has a non-zero width.");
    } else {
        println!("The rectangle has a zero width.");
    }
}
