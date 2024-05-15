// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavours {  
    Strawberry,
    Vanilla,
    Chocolate,
}

struct Drinks {
    flavor: Flavours,
    ounces: f64,
}

fn print_drinks(drink: Drinks) {

    println!("this the ounce of the drink:\n{}\nwith the taste of:", drink.ounces);
    //println!("This is the flavour {:?}", drink.flavor);

    match drink.flavor {
        Flavours::Strawberry => println!("strawberry"),
        Flavours::Vanilla => println!("vanilla"),
        Flavours::Chocolate => println!("chocolate"),

    }

}



fn main() {

    let drink = Drinks {
        flavor: Flavours::Vanilla,
        ounces: 45.5,

    };
    print_drinks(drink);


}
