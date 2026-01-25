// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn var_print(value: bool) {
    match value {
        true => println!("it's true"),
        false => println!("it's false"),
    }
}

fn parse_bool(arg: &str) -> Option<bool> {
    match arg {
        "true" | "1" => Some(true),
        "false" | "0" => Some(false),
        _ => None,
    }
}

use std::env;

fn main() {
    for arg in env::args().skip(1) {
        let value = parse_bool(&arg).unwrap_or_else(|| panic!("Invalid argument: {}", arg));

        var_print(value);
    }

    var_print(true);
    var_print(false);
}
