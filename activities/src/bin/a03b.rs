// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn calculate(input: i32) {
    if input > 5 {
        println!("Input {input} is bigger than 5")
    } else if input < 5 {
        println!("Input {input} is smaller than 5")
    } else {
        println!("Input {input} is 5")
    }
}

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in args.iter().skip(1) {
        let num = arg.parse::<i32>().unwrap_or_else(|_| {
            panic!("Error: Argument must be a valid number");
        });

        calculate(num);
    }

    calculate(4);
    calculate(5);
    calculate(6);
}
