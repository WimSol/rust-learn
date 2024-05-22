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

struct GroceryItems {

    quantity: i32,
    id_number: i32,
}

fn print_quantity(grocery_item: &GroceryItems) {
    println!("quantity: {:?}", grocery_item.quantity);
}

fn print_id_number(grocery_item: &GroceryItems) {
    println!("id_number: {:?}", grocery_item.id_number);
}

fn main() {

    let grocery_item = GroceryItems {
        quantity: 4,
        id_number: 6,
    };
    print_quantity(&grocery_item);
    print_id_number(&grocery_item);

}
