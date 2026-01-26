// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

#![allow(dead_code)]
#[derive(Debug)]
struct Box {
    dimensions: (i32, i32, i32),
    weight: i32,
    color: Color,
}

#[derive(Debug)]
enum Color {
    Red,
}

impl Box {
    fn new(dimensions: (i32, i32, i32), weight: i32, color: Color) -> Self {
        Box {
            dimensions,
            weight,
            color,
        }
    }
}

fn main() {
    let box1 = Box::new((10, 15, 7), 32, Color::Red);

    println!("{:?}", box1);
}
