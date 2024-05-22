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



fn main() {

    let mut warehouse = HashMap::new();

    warehouse.insert("Chairs", 5);
    warehouse.insert("Beds", 3);
    warehouse.insert("Tables", 2);
    warehouse.insert("Couches", 0);

    let mut total = 0;

    for (key, val) in warehouse.iter() {
        println!("{key}");

        total = total + val;
        if *val == 0 {
            println!("out of stock");
        } else {
            println!{"{val}"};
        }
    }
    println!("{total}");
}
