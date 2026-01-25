// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

use std::env;

fn main() {
    for arg in env::args().skip(1) {
        let value = arg.parse::<i32>().unwrap_or_else(|_| {
            panic!("Error: Argument must be a valid number");
        });

        match value {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => {}
        }

        match value > 3 || value < 3 {
            true => println!("not in range"),
            false => println!("in range"),
        }
    }
}
