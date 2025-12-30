fn main() {
    let my_color = Color::Green;

    match my_color {
        Color::Red => println!("The color is Red"),
        Color::Green => println!("The color is Green"),
        Color::Blue => println!("The color is Blue"),
    }

    let ipv_four = IpAddrKind::V4;
    let ipv_six = IpAddrKind::V6;
    route_kind(ipv_four);
    route_kind(ipv_six);

    // using struct with enum
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    route_struct(home);
    route_struct(loopback);

    // using enum with associated data
    let home = IpAddress::V4(String::from("127.0.0.1"));
    let loopback = IpAddress::V6(String::from("::1"));

    route(home);
    route(loopback);

    // using enum with multiple associated data types
    let home = IpAddressWithParts::V4(127, 0, 0, 1);
    let loopback = IpAddressWithParts::V6(());

    // matching enum with multiple associated data types
    let message1 = Message::Quit;
    let message2 = Message::Move { x: 10, y: 20 };
    let message3 = Message::Write(String::from("Hello, World!"));
    let message4 = Message::ChangeColor(255, 0, 0);
    message1.call();
    message2.call();
    message3.call();
    message4.call();

    // using structs instead of enum
    let quit_msg = QuitMessage;
    let move_msg = MoveMessage { x: 10, y: 20 };
    let write_msg = WriteMessage(String::from("Hello from struct!"));
    let color_msg = ChangeColorMessage(255, 0, 0);
    quit_msg.call();
    move_msg.call();
    write_msg.call();
    color_msg.call();

    // give generic enum Option a try with a value and without a value
    let some_number = Option::Some(5);
    let some_string = Option::Some("a string");
    // without a value
    let absent_number: Option<i32> = Option::None;

    let x: i32 = 5;
    let y: Option<i32> = Option::Some(5);
    // adding x and y requires handling the Option enum
    let sum = x + match y {
        Option::Some(value) => value,
        Option::None => 0,
    };
    println!("The sum of x and y is: {}", sum);
}

// custom datatype with enum
enum Color {
    Red,
    Green,
    Blue,
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn route_struct(ip: IpAddr) {
    match ip.kind {
        IpAddrKind::V4 => println!("This is an IPv4 address: {}", ip.address),
        IpAddrKind::V6 => println!("This is an IPv6 address: {}", ip.address),
    }
}

fn route_kind(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("This is an IPv4 address"),
        IpAddrKind::V6 => println!("This is an IPv6 address"),
    }
}

// another way to define enum with associated data
// so no need for separate struct
enum IpAddress {
    //  The name of each enum variant that we define also becomes a function
    // that constructs an instance of the enum
    V4(String),
    V6(String),
}

fn route(ip: IpAddress) {
    match ip {
        IpAddress::V4(address) => println!("This is an IPv4 address: {}", address),
        IpAddress::V6(address) => println!("This is an IPv6 address: {}", address),
    }
}

// enum with multiple associated data types
enum IpAddressWithParts {
    V4(u8, u8, u8, u8),
    V6(()),
}

// this enum holding four variants in different types
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// all below structs holding the same data as above enum
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// impl block for enum Message from structs
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move message to x: {}, y: {}", x, y),
            Message::Write(text) => println!("Write message: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Change color to red: {}, green: {}, blue: {}", r, g, b)
            }
        }
    }
}

impl QuitMessage {
    fn call(&self) {
        println!("Quit message");
    }
}

impl MoveMessage {
    fn call(&self) {
        println!("Move message to x: {}, y: {}", self.x, self.y);
    }
}

impl WriteMessage {
    fn call(&self) {
        println!("Write message: {}", self.0);
    }
}

impl ChangeColorMessage {
    fn call(&self) {
        println!(
            "Change color to red: {}, green: {}, blue: {}",
            self.0, self.1, self.2
        );
    }
}

// defining generic enum option
// rust do not have null values but we can use Option enum to represent a value that can be either something or nothing
enum Option<T> {
    Some(T),
    None,
}
