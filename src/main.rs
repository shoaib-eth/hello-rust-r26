use std::usize;

fn main() {
    let s1 = String::from("Hello");  
    let len = calculate_string(s1.clone()); // We've copied the s1 value and send to the function, here ownership does'nt exists

    println!("The length of {} is {}", s1, len);
}

fn calculate_string(s: String) -> usize {  
    let length = s.len();
    return length;
}

// `clone()` method is a expensive approach for memory, because it copies the value and consume extra memory slot with the same value.