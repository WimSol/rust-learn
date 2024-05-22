// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

#[derive(Debug)]
struct Adult {
    name: String,
    age: i32,
}

fn check_adult(adult: &Adult) -> Result<&Adult, String> {
    match adult.age >= 21 {
        true => Ok(adult),
        false => Err(String::from("The persons age is under 21")),
    }
}
fn main() {

    let people = vec![Adult {name: String::from("Derek"), age: 15}, Adult {name: String::from("Joseph"), age: 28}];
    
    for person in people {
        let checked_adult = check_adult(&person);
        match checked_adult {
            Ok(adult) => println!("name: {} age: {}", adult.name, adult.age),
            Err(error_string) => println!("name: {} age: {}", person.name, error_string),
        }
    }

    
    


}
