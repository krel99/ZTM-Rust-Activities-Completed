// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn stock() -> HashMap<String, u32> {
    let mut stock = HashMap::new();
    stock.insert("Chairs".to_string(), 5);
    stock.insert("Beds".to_string(), 3);
    stock.insert("Tables".to_string(), 2);
    stock.insert("Couches".to_string(), 0);
    stock
}

fn main() {
    let stock = stock();

    for (item, count) in stock.iter() {
        println!(
            "{}: {}",
            item,
            if *count == 0 {
                "out of stock".to_string()
            } else {
                format!("{}", count)
            }
        );
    }
}
