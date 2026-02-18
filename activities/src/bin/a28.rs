// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

// Topic: New type pattern

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

// --- Newtype wrappers ---
#[derive(Debug)]
struct ShoesColor(Color);

impl ShoesColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct ShirtColor(Color);

impl ShirtColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct PantsColor(Color);

impl PantsColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

// --- Functions that accept only their specific clothing color type ---
fn shoes(color: ShoesColor) {
    println!("Shoes: {:?}", color);
}

fn shirt(color: ShirtColor) {
    println!("Shirt: {:?}", color);
}

fn pants(color: PantsColor) {
    println!("Pants: {:?}", color);
}

fn main() {
    // At least one of each clothing item + color usage examples
    let shoes_black = ShoesColor::new(Color::Black);
    let shirt_white = ShirtColor::new(Color::White);
    let pants_custom = PantsColor::new(Color::Custom("Olive".to_string()));

    shoes(shoes_black);
    shirt(shirt_white);
    pants(pants_custom);

    // More examples (optional)
    shoes(ShoesColor::new(Color::Brown));
    shirt(ShirtColor::new(Color::Blue));
    pants(PantsColor::new(Color::Gray));
}
