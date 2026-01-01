use std::io;

fn main() {
    let mut input = String::new();
    println!("please Enter Your Input: ");
    io::stdin()
       .read_line(&mut input)
       .expect("Failed To Read Input");

    println!("User Input: {}", input);
}

// Input/Output Method In Rust
// This is how we take input from user