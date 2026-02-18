// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::{env, str::FromStr};

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl FromStr for PowerState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "off" => Ok(PowerState::Off),
            "sleep" => Ok(PowerState::Sleep),
            "reboot" => Ok(PowerState::Reboot),
            "shutdown" => Ok(PowerState::Shutdown),
            "hibernate" => Ok(PowerState::Hibernate),
            _ => Err(format!("Invalid power state: {}", s)),
        }
    }
}

fn main() {
    let value: PowerState = env::args()
        .nth(1)
        .expect("Missing argument")
        .parse()
        .unwrap_or_else(|e| panic!("Error: {}", e));

    match value {
        PowerState::Off => println!("Turning off"),
        PowerState::Sleep => println!("Sleeping"),
        PowerState::Reboot => println!("Rebooting"),
        PowerState::Shutdown => println!("Shutting down"),
        PowerState::Hibernate => println!("Hibernating"),
    }
}
