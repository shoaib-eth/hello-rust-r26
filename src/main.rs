#[derive(Debug)]

struct Student {
    name: String,
    age: u8,
    result: bool,
}
fn main() {
    let student_details: Student = Student {
        name: "Alice".to_owned(),
        age: 21,
        result: true,
    };

    println!("Student = {:?}", student_details);

    // print all the values manually
    println!(
        "Student Name = {}, Student Age = {}, Student Result = {}",
        student_details.name, student_details.age, student_details.result
    );
}

// Notes 📝

// If we want to print struct values, we have to use `#[derive(Debug)]`
