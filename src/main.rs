// fn main() {
//     let x: u8 = 5;
//     process_integer(x);

//     println!("The value of x in main() is: {}", x);
// }

// fn process_integer(x: u8) {
//     println!("The value of x in process_integer() is: {}", x);
// }

// This code will compile and run successfully, printing the value of `x` in both the `main` function and the `process_integer` function. because here we deals with stack-allocated data (u8), which implements the `Copy` trait in Rust. When `x` is passed to `process_integer`, a copy of the value is made, leaving the original `x` in `main` unchanged and still accessible after the function call.

fn main() {
    let x: String = String::from("Hello, Rust!"); // Here, x is the owner 
    process_string(x); // Now ownership transfered from x to process_integer() function, so x is no longer valid for accessing the string.

    // println!("The value of x in main() is: {}", x); // This line would cause a compile-time error
}

fn process_string(s: String) {
    println!("The value of s in process_string() is: {}", s);
}
// This code will not compile because `String` is a heap-allocated type that does not implement the `Copy` trait. When `x` is passed to `process_string`, ownership of the `String` is moved to the function, and `x` in `main` is no longer valid after the function call. Attempting to access `x` after the move results in a compile-time error.

// If we want to print both in main and in process_string, we can use `.clone()` to create a copy of the `String` before passing it to the function:
// fn main() {
//     let x: String = String::from("Hello, Rust!");
//     process_string(x.clone());
//     println!("The value of x in main() is: {}", x);
// }
// fn process_string(s: String) {
//     println!("The value of s in process_string() is: {}", s);
// }