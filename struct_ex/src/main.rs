fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of a rectangle with width {} and height {} is {} square pixels.",
        width1,
        height1,
        area(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "The area of a rectangle with width {} and height {} is {} square pixels.",
        rect1.0,
        rect1.1,
        area2(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    // Using a tuple to represent the dimensions of the rectangle
    println!(
        "The area of a rectangle with width {} and height {} is {} square pixels.",
        rect2.width,
        rect2.height,
        area2((rect2.width, rect2.height))
    );

    // Using a struct to represent the rectangle with reference to it
    println!(
        "The area of a rectangle with width {} and height {} is {} square pixels.",
        rect2.width,
        rect2.height,
        area3(&rect2)
    );

    println!("rect2 is {:?}", rect2); // Debug print of the struct

    dbg!(&rect1); // Debug print of the tuple using dbg! macro
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// Using a tuple to represent the dimensions of the rectangle
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
