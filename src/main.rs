fn main() {
    let user_id_1 = 1;
    let user_id_2 = 2;

    // let value_1 = get_user_phone_number(user_id_1);
    // println!("value_1: {:?}", value_1);  // value_1: Some(89847)

    // let value_2 = get_user_phone_number(user_id_2);
    // println!("value_2: {:?}", value_2); // value_2: None

    // This is best approcah because it prints the value in number format for eg. mobile number `98083`, not print Some(89847)
    match get_user_phone_number(user_id_1) {
        Some(data) => println!("Date: {}", data),
        None => println!("User Mobile Number Does Not Exists!"),
    }
}

fn get_user_phone_number(user_id: i32) -> Option<i32> {
    // it is generic type, means in option< > we can define many data types
    let mob_num = 89847;
    if user_id == 1 {
        return Some(mob_num);
    } else {
        return None;
    }
}

// Note - https://dev.to/fadygrab/learning-rust-14-option-enum-an-enum-and-pattern-matching-use-case-1dgf
