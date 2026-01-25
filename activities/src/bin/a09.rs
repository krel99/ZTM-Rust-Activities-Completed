// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

use rand::Rng;

fn get_coords() -> (i32, i32) {
    let mut rng = rand::rng();
    (rng.random_range(0..=180), rng.random_range(0..=360))
}

fn main() {
    let (x, y) = get_coords();
    if y > 5 {
        println!("y is greater than 5 --- {}", y);
    } else if y < 5 {
        println!("y is less than 5 --- {}", y);
    } else {
        println!("y is equal to 5 --- {}", y);
    }

    if x > 5 {
        println!("x is greater than 5 --- {}", x);
    } else if x < 5 {
        println!("x is less than 5 --- {}", x);
    } else {
        println!("x is equal to 5 --- {}", x);
    }
}
