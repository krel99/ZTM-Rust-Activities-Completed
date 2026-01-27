// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let student = Student {
        name: String::from("Nigerian Prince"),
        locker: Some(123),
    };

    match student.locker {
        Some(locker_number) => println!("{}'s locker number {}", student.name, locker_number),
        None => println!("{} does not have locker", student.name),
    }
}
