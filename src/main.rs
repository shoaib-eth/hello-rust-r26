fn main() {
    let mut s1: String = String::from("Hello");
    let w1 = &mut s1;
    w1.push_str(" World");
    println!("w1 = {}", w1);

    let w2 = &mut s1;
    w2.push_str(" Code");
    println!("w2 = {}", w2);

    // println!("w1 = {}, w2 = {}", w1, w2);  // This line will give an error, because we write all borrowed value at once
}

// Here write performance can be easily executed
// but if we write only once, it would give an error 
