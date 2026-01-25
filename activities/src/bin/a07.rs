// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Red,
    Green,
    Blue,
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("\x1b[31mRed\x1b[0m"),
        Color::Green => println!("\x1b[32mGreen\x1b[0m"),
        Color::Blue => println!("\x1b[34mBlue\x1b[0m"),
    }
}

use std::str::FromStr;

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(format!("'{}' is not a valid color", s)),
        }
    }
}

use std::env;

fn main() {
    let color = env::args()
        .nth(1)
        .and_then(|s| s.parse::<Color>().ok())
        .unwrap_or_else(|| panic!("Expected a color parameter"));
    print_color(color);
}
