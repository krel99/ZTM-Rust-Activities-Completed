// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

use std::env;

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn main() {
    println!("Result of 1+6 is: {:?}", add(1, 6));

    let args: Vec<String> = env::args().collect();
    // Will panic with "ParseIntError" if the argument isn't a valid i32
    let num1 = args[1].parse::<i32>().unwrap_or_else(|_| {
        panic!("Error: First argument must be a valid integer");
    });
    let num2 = args[2].parse::<i32>().unwrap();

    println!("Result of {num1}+{num2} is: {:?}", add(num1, num2));
}
