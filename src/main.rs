use std::collections::HashMap;

fn main() {
    let mut students: HashMap<String, u32> = HashMap::new();

    students.insert("Ravi".to_owned(), 100);
    students.insert("Raju".to_owned(), 10);
    students.insert("Lalu".to_owned(), 1);

    for (student, marks) in students.iter() {
        println!("Student name: {:?} marks={}", student, marks);
    }

    students.insert("Raju".to_owned(), 200);

    match students.get("Raju") {
        Some(marks) => println!("Found: {}", marks),
        None => println!("Not Found"),
    }
}
