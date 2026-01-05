struct Rectangle {
    length: u16,
    breadth: u16,
}

fn area_rec(rec: Rectangle) -> u16 {
    return rec.length * rec.breadth;
}

fn main() {
    let rec_one: Rectangle = Rectangle {
        length: 10,
        breadth: 5,
    };

    let rec_two: Rectangle = Rectangle {
        length: 20,
        breadth: 15,
    };

    // rec_one -> rec -> result_one
    let result_one: u16 = area_rec(rec_one); // Transfer of ownership of `rec_one` to `rec` in `area_rec()` parameter 
    println!("Result One: {}", result_one);

    // rec_two -> rec -> result_two
    let result_two: u16 = area_rec(rec_two); // Transfer of ownership of `rec_two` to `rec` in `area_rec()` parameter 
    println!("Result Two: {}", result_two);
}

// Notes 📝

// Ownership in struct works like ownership in `String`
