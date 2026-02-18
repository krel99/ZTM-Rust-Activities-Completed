// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

#[derive(Debug)]
struct Adult {
    name: String,
    age: u32,
}

impl Adult {
    fn new(name: String, age: u32) -> Result<Self, String> {
        if age >= 21 {
            Ok(Self { name, age })
        } else {
            Err(format!(
                "{} is not old enough to be an adult (age: {})",
                name, age
            ))
        }
    }
}

fn main() {
    let adult1 = Adult::new("Eldrith".to_string(), 18);
    let adult2 = Adult::new("Horus".to_string(), 2555);

    match adult1 {
        Ok(adult) => println!("{} is an adult (age: {})", adult.name, adult.age),
        Err(err) => println!("{}", err),
    }

    match adult2 {
        Ok(adult) => println!("{} is an adult (age: {})", adult.name, adult.age),
        Err(err) => println!("{}", err),
    }
}
