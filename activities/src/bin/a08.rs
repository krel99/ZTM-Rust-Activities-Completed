// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

#[derive(Clone, PartialEq)]
enum Flavor {
    Good,
    Bad,
    Terrible,
    Unknown,
}

struct Drink {
    flavor: Flavor,
    weigh_grams: i32,
}

fn main() {
    for i in [0, 1, 2, 3] {
        let flavor: Flavor = match i {
            0 => Flavor::Good,
            1 => Flavor::Bad,
            2 => Flavor::Terrible,
            3 => Flavor::Unknown,
            _ => unreachable!(),
        };
        if flavor == Flavor::Unknown {
            println!("Unknown flavor");
            continue;
        }
        for i in [99, 150, 300, 950] {
            let drink = Drink {
                flavor: flavor.clone(),
                weigh_grams: i,
            };
            print_drink(&drink);
        }
    }
}

fn print_drink(drink: &Drink) {
    print!("The drink's flavor is: ");
    match drink.flavor {
        Flavor::Good => println!("\x1b[32mGood\x1b[0m"),
        Flavor::Bad => println!("\x1b[33mBad\x1b[0m"),
        Flavor::Terrible => println!("\x1b[31mTerrible\x1b[0m"),
        Flavor::Unknown => println!("\x1b[90mUnknown\x1b[0m"),
    }
    println!("The drink's weight is: {} grams", drink.weigh_grams);
}
