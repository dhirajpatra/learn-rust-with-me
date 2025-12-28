struct AlwayEqual; // Unit-like struct when need to implement a trait but no data needed

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail1@example.com");

    let user2 = build_user(
        String::from("anotheremail2@example.com"),
        String::from("anotherusername456"),
        user1.sign_in_count,
        Some(user1.active), // Passing active status from user1
    );

    println!(
        "User1: {}, {}, {}, {}",
        user1.username, user1.email, user1.active, user1.sign_in_count
    );
    println!(
        "User2: {}, {}, {}, {}",
        user2.username, user2.email, user2.active, user2.sign_in_count
    );

    let user3 = User {
        email: String::from("anotheremail3@example.com"),
        ..user2 // Using struct update syntax
    };

    println!(
        "User3: {}, {}, {}, {}",
        user3.username, user3.email, user3.active, user3.sign_in_count
    );

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwayEqual;
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// this will not compile because the fields are &str, which require a lifetime
// struct User {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }

fn build_user(email: String, username: String, sign_in_count: u64, active: Option<bool>) -> User {
    User {
        email,
        username,
        active: active.unwrap_or(true), // Default to true if not provided
        sign_in_count,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
