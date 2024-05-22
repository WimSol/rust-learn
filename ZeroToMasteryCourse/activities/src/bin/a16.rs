// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct StudentInfo {
    name: String,
    locker_number: Option<i32>,
}
fn main() {

    let students = vec![StudentInfo {name: String::from("Phillip"), locker_number: Some(32)}, StudentInfo {name: String::from("Dylan"), locker_number: None}];

    for student in &students {
        println!("name: {}", student.name);

        match student.locker_number {
            Some(num) => println!("LockerNumber: {}", num),
            None => println!("LockerNumber: None"),
        }
    }

    
}
