// Topic: Flow control using if..else
//
// Program requirements:
// * Displays a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
// Notes:
// * Use a variable set to either true or false
// * Use an if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

use std::env;

fn display_bool(decider: bool) {
    if decider {
        println!("YESSSSSSS")
    } else {
        println!("NOOOOO")
    }
}

fn main() {
    display_bool(true);
    display_bool(false);

    let args: Vec<String> = env::args().collect();

    let num1 = args[1].parse::<i32>().unwrap_or_else(|_| {
        panic!("Error: First argument must be a valid number");
    });

    if num1 == 1 {
        display_bool(true);
    } else {
        display_bool(false);
    }
}
