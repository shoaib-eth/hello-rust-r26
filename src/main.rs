use std::usize;

fn main() {
    let s1 = String::from("Hello");  // here s1 is the owner
    let (s2, len) = calculate_string(s1); // Transfer ownership from s1 to len

    println!("The length of {} is {}", s2, len);
}

fn calculate_string(s: String) -> (String, usize) {  // s will be the new owner
    let length = s.len();
    return (s, length);
}