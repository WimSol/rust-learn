// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function


struct PersonTraits {

    name: String,
    favcolor: String,
    age: i32,

}

fn print_info(info: &PersonTraits) {
    println!("{:?}", info.name);
    println!("{:?}", info.favcolor);

}



fn main() {

    let people = vec! [
        PersonTraits {
            name: String::from("Brian"),
            favcolor: String::from("blue"),
            age: 12,
        },
        PersonTraits {
            name: String::from("William"),
            favcolor: String::from("purple"),
            age: 10,
        },
        PersonTraits {
            name: String::from("Andrew"),
            favcolor: String::from("Red"),
            age: 7,
        },

    ];

    for p in &people {

        if p.age <= 10 {
            print_info(&p)
        }
        
    }
}
