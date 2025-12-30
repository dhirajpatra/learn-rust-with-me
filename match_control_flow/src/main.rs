use std::os::linux::raw::stat;

fn main() {
    println!("Value in cents for a Dime: {}", value_in_cents(Coin::Dime));
    println!(
        "Value in cents for a Quarter from California: {}",
        value_in_cents(Coin::Quarter(UsState::California))
    );

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six: {:?}, none: {:?}", six, none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => {
            move_player(other);
        }
        _ => reroll(), // wildcard arm to catch all other values (not needed here because other catches all)
    }
}

fn add_fancy_hat() {
    println!("Adding a fancy hat to your character!");
}

fn remove_fancy_hat() {
    println!("Removing your fancy hat!");
}

fn move_player(num_spaces: u8) {
    println!("Moving player {} spaces.", num_spaces);
}

fn reroll() {
    println!("Rerolling the dice!");
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// function to return value in cents for each coin type
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        } // comma not needed here because it's the last arm
        Coin::Nickel => 5, // comma is needed here
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

// function to demonstrate matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,           // there is no value so no addition
        Some(i) => Some(i + 1), // there is a value, so add 1 to it
    }
}
