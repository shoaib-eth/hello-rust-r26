fn main() {
    let mut s1 = String::from("Hello");   // Make this mutable by `mut` keyword
    append_string(&mut s1);    // Applied `&mut`
    println!("The New String is {} ", s1);

    // let len = calculate_string(&mut s1); 
    // println!("The length of {} is {}", s1, len);
}

// fn calculate_string(s2: &String) -> usize {
//     return s2.len();
// }

fn append_string(s3: &mut String) {  // Applied `&mut` 
    s3.push_str(" World");
}


// Note 📝

// If we want to change the borrowed string, we have to apply `&mut` keyword.

// Without `&mut` keyword or only with `&` keyword, the string changes in borrow is not possible, gives an error
