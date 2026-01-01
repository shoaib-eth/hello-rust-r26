// Shadowing

fn main() {
    /*
     * All Variable Names Are Same,
     * But There Types Are Different
     */
    let x = 10;  // Integer Type
    println!("x = {}", x);
    
    let x = "Hello 👋";  // String Type
    println!("x = {}", x); 

    let x = x.len();  // Integer Type
    println!("Length of x is {}", x); // it will access the last one `x` value
}

// Notes 📝

// In Shadowing, variable names could be same, but there data type should be different