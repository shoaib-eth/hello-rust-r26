#[derive(Debug)]

struct Rectangle {
    length: u16,
    breadth: u16,
}

impl Rectangle {
    // Associative Functions
    fn new(new_length: u16, new_breadth: u16) -> Self {
        Self {
            length: new_length,
            breadth: new_breadth,
        }
    }

    // Methods
    fn area(&self) -> u16 {
        return self.length * self.breadth;
    }

    fn change_breadth(&mut self) {
        self.breadth = 5;
    }
}

fn main() {
    let rec_one: Rectangle = Rectangle::new(10, 5);
    let mut rec_two: Rectangle = Rectangle::new(20, 15);

    // Now we're calculating area of rectangle
    let result_one = rec_one.area(); // Instead of passing ownership, it borrows the value
    println!("rec_one: {:?}", rec_one);
    println!("Result One: {}", result_one);

    let result_two = rec_two.area(); // Instead of passing ownership, it borrows the value
    println!("rec_two: {:?}", rec_two);
    println!("Result Two: {}", result_two);

    // If we want to change something in `rec_two`, we have to make it `mut` and in `change_breadth()` make it `&mut`
    rec_two.change_breadth();
    println!("rec_two new breadth: {:?}", rec_two);
}

// Notes 📝

// To know which is the `Associative Function` and which os the `Methods`, there are some practices

// 1. In Associative Function we write capital `Self` param
// 2. In Methods we write small `self` param

// It is very efficient way of coding in rust because we have to specify our logic in `impl` and we can run that by calling, and we dont need to update our whole code we have to update only `impl` block code and everything will run smoothly.

// For More Details Read These Notes

// This Topic Notes Starts From 14th PPT

// https://docs.google.com/presentation/d/1t7IMRMdkR3an0-YofAF9M0r9od_0kcfl03UCjdE5njQ/edit?slide=id.g318101e5d17_0_61#slide=id.g318101e5d17_0_61
