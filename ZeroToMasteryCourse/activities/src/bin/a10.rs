// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print_var(boolean_var: bool) {

    match boolean_var {
        true => println!("its big"),
        false => println!("its small"),
    }
}

fn main() {

    let value = 200;
    let boolean_var = value > 100;

    print_var(boolean_var);
    
}
