fn main() {
    let s1: String = get_string();  // s1 is the owner of `Hello`
    println!("This is s1: {}", s1);

    let s2: String = String::from("World!");  // Here, s2 is the owner of `World!` string
    let s3: String = send_get_string(s2);  // But here, s2's ownership transfered to s3 and s2 is no longer valid
    println!("This is s3: {}", s3);

    // println!("This is s2: {}", s2); // This would give an error because s3 is the original owner of s2
}

fn get_string() -> String {
    let new_string = String::from("Hello");  // Owner: new_string
    return new_string; // Transfer ownership to s1
}

// recieved_string is the owner of `World!`
fn send_get_string(recieved_string: String) -> String {
    return recieved_string;  // Transfer s2's ownership from `recieved_string` to s3
}

// In s2's case, there are three owners made [s2 -> recieved_string -> s3], but valid or final owner is s3