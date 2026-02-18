// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
#[allow(dead_code)]
enum VehicleType {
    Car,
    Truck,
    Van,
}

#[derive(Debug, PartialEq)]
enum VehicleStatus {
    Available,
    #[allow(dead_code)]
    Unavailable,
    Maintenance,
    Rented,
}

#[derive(Debug)]
struct Rental {
    vehicle_type: VehicleType,
    vin: String,
    status: VehicleStatus,
}

type SharedRentals = Rc<RefCell<Vec<Rental>>>;

struct Corporate(SharedRentals);

struct StoreFront(SharedRentals);

impl Corporate {
    fn new(rentals: SharedRentals) -> Self {
        Self(rentals)
    }

    fn rentals(&self) -> &SharedRentals {
        &self.0
    }
}

impl StoreFront {
    fn new(rentals: SharedRentals) -> Self {
        Self(rentals)
    }

    fn rentals(&self) -> &SharedRentals {
        &self.0
    }
}

fn print_rentals(rentals: &SharedRentals) {
    let rentals = rentals.borrow();
    for rental in rentals.iter() {
        println!(
            "  VIN: {}, Type: {:?}, Status: {:?}",
            rental.vin, rental.vehicle_type, rental.status
        );
    }
}

fn main() {
    let rentals: SharedRentals = Rc::new(RefCell::new(vec![
        Rental {
            vehicle_type: VehicleType::Car,
            vin: "1HGCM82633A123456".to_owned(),
            status: VehicleStatus::Available,
        },
        Rental {
            vehicle_type: VehicleType::Truck,
            vin: "5TFEV54198X456789".to_owned(),
            status: VehicleStatus::Available,
        },
    ]));

    let corporate = Corporate::new(Rc::clone(&rentals));
    let storefront = StoreFront::new(Rc::clone(&rentals));

    println!("=== Initial rental status ===");
    print_rentals(corporate.rentals());

    // StoreFront rents out the car
    {
        let mut rentals = storefront.rentals().borrow_mut();
        rentals[0].status = VehicleStatus::Rented;
    }
    println!("\n=== After storefront rents out the car ===");
    print_rentals(corporate.rentals());

    // Corporate puts the truck in maintenance
    {
        let mut rentals = corporate.rentals().borrow_mut();
        rentals[1].status = VehicleStatus::Maintenance;
    }
    println!("\n=== After corporate puts truck in maintenance ===");
    print_rentals(storefront.rentals());
}
