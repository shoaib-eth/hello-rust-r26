use std::fmt::Display;

// This syntax will help to print all data type values
// This is Generic Type
// Here we use <T: Displat>, T, this will help to read all the data types and printing it all.
fn print_data<T: Display>(data: T) -> T {
    // println!("Data: {}", data);
    data
}

fn main() {
    let x = 5;
    let y = true;
    let z = "Hello".to_owned();

    println!("Data: {}", print_data(x));
    println!("Data: {}", print_data(y));
    println!("Data: {}", print_data(z));
}
