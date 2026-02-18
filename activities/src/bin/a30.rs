// Topic: Generics & Structures
//
// Requirements:
// * Create a Vehicle structure that is generic over traits Body and Color
// * Create structures for vehicle bodies and vehicle colors and implement the
//   Body and Color traits for these structures
// * Implement a 'new' function for Vehicle that allows it to have any body
//   and any color
// * Create at least two different vehicles in the main function and print their
//   info
//
// Notes:
// * Examples of car bodies can be Truck, Car, Scooter
// * Examples of colors could be red, white, black
// * It is not necessary to have data fields or function implementations
//   for the vehicle bodies/colors

trait Body {}
trait Color {}

// Vehicle body types
#[derive(Debug)]
struct Truck;
#[derive(Debug)]
struct Car;
#[derive(Debug)]
struct Scooter;

impl Body for Truck {}
impl Body for Car {}
impl Body for Scooter {}

// Vehicle colors
#[derive(Debug)]
struct Red;
#[derive(Debug)]
struct White;
#[derive(Debug)]
struct Black;

impl Color for Red {}
impl Color for White {}
impl Color for Black {}

// Generic Vehicle structure
#[allow(dead_code)]
#[derive(Debug)]
struct Vehicle<B: Body, C: Color> {
    body: B,
    color: C,
}

impl<B: Body, C: Color> Vehicle<B, C> {
    fn new(body: B, color: C) -> Self {
        Self { body, color }
    }
}

fn main() {
    let red_truck = Vehicle::new(Truck, Red);
    println!("{:?}", red_truck);

    let white_car = Vehicle::new(Car, White);
    println!("{:?}", white_car);

    let black_scooter = Vehicle::new(Scooter, Black);
    println!("{:?}", black_scooter);
}
