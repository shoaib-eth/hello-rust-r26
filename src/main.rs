fn main() {
    // It is &str type
    let greeting: &str = "Hello 👋, Rust Developers!";
    println!("{}", greeting);

    // It is String type
    let farewell: String = String::from("Goodbye 👋, Rust Developers!");
    println!("{}", farewell);

    // If we need to push more data, we use String
    let mut dynamic_string = String::from("Welcome 🙏");
    println!("{}", dynamic_string);

    dynamic_string.push_str(" to the Rust developers!");
    println!("{}", dynamic_string);

    // Notes 📝 - 
    // 1. This is not possible in `&str` because it holds only fixed-size string
    // 2. before push more string data, we need to make it mutable using `mut` keyword

    // Summary 📚 -
    // String - Dynamic Length String -> Heap Allocated
    // &str - Fixed Length String -> Stack Allocated
}
