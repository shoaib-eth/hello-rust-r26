const GLOBAL_VALUE: u8 = 100;

fn main() {
    let outside_variable = 10;

    {
        let inside_variable = 20;
        println!("inside_variable = {}", inside_variable);
        println!("outside_variable = {}", outside_variable);

    } // inside_variable goes out of scope here

    // println!("inside_variable = {}", inside_variable); // This would cause a compile-time error

    println!("outside_variable = {}", outside_variable);
    print_value();
}

fn print_value() {
    println!("GLOBAL_VALUE = {}", GLOBAL_VALUE);
}

// GLOBAL_VALUE is accessible throughout the entire module and in all functions within it.
// outside_variable is accessible only within main and its inner scopes.
// inside_variable is accessible only within the inner block where it is defined.
