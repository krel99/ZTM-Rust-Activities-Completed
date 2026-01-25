// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    id: i32,
    stock: i32,
}

fn main() {
    let item = GroceryItem { id: 1, stock: 10 };

    print_quantity(&item);
    print_id(&item);
}

fn print_quantity(item: &GroceryItem) {
    println!("Quantity: {}", item.stock);
}

fn print_id(item: &GroceryItem) {
    println!("ID: {}", item.id);
}
