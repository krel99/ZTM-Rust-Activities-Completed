// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

use rand::Rng;

fn print_is_over_100(val: bool) {
    match val {
        true => println!("This number is over 100"),
        false => println!("This number is below 100"),
    }
}

fn main() {
    let mut rng = rand::rng();
    let val = rng.random_range(0..=180);

    println!("Generated random value: {}", val);

    let is_over_100 = val > 100;

    print_is_over_100(is_over_100);
}
