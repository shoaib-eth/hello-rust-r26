fn main() {
    let s1 = String::from("Hello");  
    let len = calculate_string(&s1); // Here we done borrow option by applying `&` with `s1`, which means we borrow s1's refrence for few time

    println!("The length of {} is {}", s1, len);
}

fn calculate_string(s: &String) -> usize {  // `&` also applied here in data type
    let length = s.len();
    return length;
}


// Note 📝
