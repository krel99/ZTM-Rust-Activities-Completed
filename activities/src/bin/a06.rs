// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use break to exit the loop

use std::env;

fn main() {
    let start_num: i32 = env::args()
        .nth(1)
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or_else(|| panic!("Expected a positive i32 parameter"));

    let mut num: i32 = start_num;

    while num > 0 {
        println!("{}", num);
        num -= 1;
    }
}
