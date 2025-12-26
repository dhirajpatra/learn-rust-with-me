fn main() {
    println!("Hello, world!");

    another_function();

    another_function_with_params(5, 10);

    let result = yet_another_function_with_return(10);
    println!("The result is: {}", result);

    print_labeled_measurement(42, 'm');

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_params(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn yet_another_function_with_return(x: i32) -> i32 {
    x + 1
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
