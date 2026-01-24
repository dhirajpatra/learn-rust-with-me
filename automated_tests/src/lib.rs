#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

// automated_tests/src/lib.rs
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
}

// Function that prints a value and returns 10
fn prints_and_returns_10(a: i32) -> i32 {
    println!("a is {}", a);
    10
}

#[cfg(test)]
// The tests module is only compiled when running tests
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-2, -3), -5);
    }

    #[test]
    fn test_anoter() {
        panic!("Make this test fail")
    }

    #[test]
    fn test_rectangle_can_hold() {
        let rect1 = Rectangle {
            width: 10,
            height: 5,
        };
        let rect2 = Rectangle {
            width: 5,
            height: 3,
        };
        assert!(rect1.can_hold(&rect2));
    }

    #[test]
    fn test_rectangle_cannot_hold() {
        let rect1 = Rectangle {
            width: 5,
            height: 3,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 5,
        };
        assert!(!rect1.can_hold(&rect2));
    }

    #[test]
    // Test for the greeting function
    fn test_greeting() {
        let result = greeting("Alice");
        assert_eq!(result, "Hello, Alice!");
    }

    #[test]
    // Test to ensure the greeting contains the name provided
    fn greeting_contains_name() {
        let result = greeting("Bob");
        assert!(
            result.contains("Bob"),
            "Greeting did not contain name, value was: {}",
            result
        );
    }

    #[test]
    fn test_guess_new() {
        let guess = Guess::new(50);
        assert_eq!(guess.value, 50);
    }

    #[test]
    // Test that Guess::new panics when value is less than 1 with expected message
    #[should_panic(expected = "Guess value must be between 1 and 100, got 0.")]
    fn test_guess_greater_than_100() {
        // this will panic as value is greater than 100
        Guess::new(200);
    }

    #[test]
    // Test that Guess::new panics when value is greater than 100 with expected message
    fn it_works() -> Result<(), String> {
        let result = 2 + 2;
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn test_prints_and_returns_10() {
        let value = prints_and_returns_10(5);
        assert_eq!(value, 10);
        assert_eq!(prints_and_returns_10(42), 10);
    }
}
