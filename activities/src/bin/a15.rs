// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(String, f64),
    Vip(String, f64),
    Standard(f64),
}

impl Ticket {
    fn new_backstage(name: String, price: f64) -> Self {
        Ticket::Backstage(name, price)
    }

    fn new_vip(name: String, price: f64) -> Self {
        Ticket::Vip(name, price)
    }
}

fn main() {
    let tickets = vec![
        Ticket::new_backstage("Okolukun".to_string(), 100.0),
        Ticket::new_vip("Apophis".to_string(), 50.0),
        Ticket::Standard(25.0),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(name, price) => {
                println!("Backstage ticket for {} - ${}", name, price)
            }
            Ticket::Vip(name, price) => println!("VIP ticket for {} - ${}", name, price),
            Ticket::Standard(price) => println!("Standard ticket - ${}", price),
        }
    }
}
