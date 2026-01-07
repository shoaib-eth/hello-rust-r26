trait Greet {
    fn greet(&self) {
        println!("Hello 👋 From Rust Developers ✌️");
    }
}

struct Person;
struct Student;

impl Greet for Person {}

impl Greet for Student {
    fn greet(&self) {
        println!("Hello 👋 From Students ✌️");
    }
}

fn main() {
    let p: Person = Person;
    p.greet();

    let s: Student = Student;
    s.greet();
}

// We can implement functions in `trait` also``

// We can do the override the function of trait, for eg. we copy the `greet()` function from `trait` function pasted it into Student `impl` and changed the message
