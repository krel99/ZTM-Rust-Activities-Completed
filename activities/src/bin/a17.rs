// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

use std::env;

fn main() {
    let input: String = env::args()
        .nth(1)
        .and_then(|s| s.parse::<String>().ok())
        .unwrap_or_else(|| panic!("Expected String parameter"));

    let lowercase = input.to_lowercase();
    let uppercase = input.to_uppercase();

    println!("Lowercase: {}", lowercase);
    println!("Uppercase: {}", uppercase);
}
