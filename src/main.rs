fn main() {
    // Tuple
    let emp_info: (&str, u8) = ("Alice", 30);
    println!("Employee Name: {}, Age: {}", emp_info.0, emp_info.1);  // Accessing tuple elements by index

    // we can do destructuring
    let (name, age) = emp_info;  
    println!("Employee Name: {}, Age: {}", name, age);
}


// Notes 📝:

// 1. Tuples are fixed-size collections of values of different types.
// 2. Elements in a tuple can be accessed using dot notation with their index (starting from 0).
// 3. Tuples can be destructured into individual variables for easier access.
// 4. Tuples are useful for grouping related values together without creating a custom struct.