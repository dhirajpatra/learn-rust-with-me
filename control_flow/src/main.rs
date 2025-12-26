fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    if number != 0 {
        println!("number is non-zero");
    } else if number < 5 {
        println!("number is less than 5");
    } else {
        println!("number is 5 or greater");
    }

    // indefinite loop never use without break or return
    loop {
        println!("again!");
        break;
    }

    let mut count = 0;
    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };
    println!("The result is {}", result);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("number = {number}");
        number -= 1;
    }
    println!("End number = {number}");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}")
    }

    for number in 1..=5 {
        println!("the number is: {number}");
    }

    for number in (1..=5).rev() {
        println!("the number is: {number}");
    }
}
