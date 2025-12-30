fn main() {
    let mut s1: String = String::from("Hello");
    let r2 = &s1;
    println!("r1 = {}", r2);

    let w1 = &mut s1;
    w1.push_str("World");
    println!("w1 = {}", w1);
}

// This code will successfully executed because it is not implement multiple read and write operations

// Summary 🗒️

// Multiple read and write operations is not allowed in borrow
