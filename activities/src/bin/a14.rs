// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i8,
    name: String,
    color: Color,
}

// this is technically a string in the end
enum Color {
    Red,
    Blue,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Red => write!(f, "Red"),
            Color::Blue => write!(f, "Blue"),
        }
    }
}

impl Person {
    fn new(age: i8, name: String, color: Color) -> Self {
        Person { age, name, color }
    }
}

fn main() {
    let people = vec![
        Person::new(5, "Alice".to_string(), Color::Red),
        Person::new(7, "Bob".to_string(), Color::Blue),
        Person::new(12, "Charlie".to_string(), Color::Red),
    ];

    for person in people {
        if person.age <= 10 {
            println!("Name: {}", person.name);
            println!("Favorite Color: {}", person.color);
        }
    }
}
