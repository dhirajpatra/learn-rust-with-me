fn main() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // same as match above
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;

    // using match
    match config_max {
        Some(max) => {
            while count < max {
                println!("count is {}", count);
                count += 1;
            }
        }
        _ => (),
    }

    // using if let
    if let Some(max) = config_max {
        while count < max {
            println!("count is {}", count);
            count += 1;
        }
    }

    impl UsState {
        fn existed_in(&self, year: u16) -> bool {
            match self {
                UsState::Alabama => year >= 1819,
                UsState::Alaska => year >= 1959,
                // add other states as needed
            }
        }
    }
}

// defining enum for US states
fn describe_state(state: UsState, quarter: u8) {
    if let UsState::Alabama = state {
        match quarter {
            1 => println!("First quarter in Alabama"),
            2 => println!("Second quarter in Alabama"),
            3 => println!("Third quarter in Alabama"),
            4 => println!("Fourth quarter in Alabama"),
            _ => println!("Invalid quarter"),
        }
    } else {
        println!("Not Alabama");
    }
}

// defining enum for coins with associated data
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // add other states as needed
}

// how old the state quarter is and Option return type
fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} existed in 1900")) // return a String wrapped in Some
        } else {
            Some(String::from("{state:?} did not exist in 1900"))
        }
    } else {
        None
    }
}

fn describe_state_quarter_new(coin: Coin) -> Option<String> {
    // Using if let for pattern matching
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        return Some(format!("{state:?} existed in 1900")); // return a String wrapped in Some
    } else {
        return Some(String::from("{state:?} did not exist in 1900"));
    };
}
