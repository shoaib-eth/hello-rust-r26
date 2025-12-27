fn main() {
    let mut num:u16 = 23;
    num = 500;
    println!("The Number is: {}", num);
}


// In Rust, the variables are immutable by default. This means that once a value is assigned to a variable, it cannot be changed unless the variable is explicitly declared as mutable using the `mut` keyword. In the provided code, the variable `num` is immutable, so its value cannot be altered after its initial assignment.

// Here we declare `num` as mutable using `let mut num:u16 = 23;`, which allows us to change its value later in the code. After declaring it as mutable, we can successfully assign a new value of `500` to `num`. Finally, we print the value of `num`, which will output "The Number is: 500".

// Now this code will give an yellow warning message because the initial value is `23` but it never used. but our program will print new value `500` without any error. this is the true behavior of mutable variables in Rust.