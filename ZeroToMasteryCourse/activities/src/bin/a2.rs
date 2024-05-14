// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result



fn main() {
    let value = add_numbers(5, 10);

    print_numbers(value)

}

fn add_numbers(a: i32, b: i32 ) -> i32 {
  a + b
}

fn print_numbers(value: i32) {
    println!("{:?}", value)
}
