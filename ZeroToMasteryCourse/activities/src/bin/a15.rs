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
#[derive(Debug)]
enum Acceslevel {
    Standard(f64),
    Vip(f64, String),
    Backstage(f64, String),
}


#[derive(Debug)]
struct Ticket {
    number: i32,
    info: String,
    acceslevel: Acceslevel,

}
fn main() {


    let vector_of_tickets = vec![
        Ticket {number: 1, info: String::from("Ticket to an event"), acceslevel: Acceslevel::Standard(15.99)},
        Ticket {number: 2, info: String::from("Ticket to an event"), acceslevel: Acceslevel::Vip(49.99, String::from("Patrick"))},
        Ticket {number: 3, info: String::from("Ticket to an event"), acceslevel: Acceslevel::Backstage(0.0, String::from("Johannes"))},

    ];

    // for i in &vector_of_tickets {
    //     println!("{:?}", i);
    // }

    for ticket in &vector_of_tickets {
        println!("\nTHIS IS TICKET INFO from ticket nr.{:?}", ticket.number);
        println!("info: {}", ticket.info);
        //println!("price: {:?}$", ticket.price);
        match &ticket.acceslevel {
            Acceslevel::Standard(price) =>  { println!("price: {:?}$", price) }
            Acceslevel::Vip(price, name) => println!("price: {:?}$\n{:?} got a vip ticket", price, name),
            Acceslevel::Backstage(price, name) => println!("price: {:?}$\n{} got a backstage ticket", price, name),

        }
    }

}
