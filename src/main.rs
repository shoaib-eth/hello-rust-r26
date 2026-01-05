#[derive(Debug)]

struct Rectangle {
    length: u16,
    breadth: u16,
}

fn area_rec(rec: &Rectangle) -> u16 {
    // value is borrowed
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

    println!("rec_one = {:?}", rec_one);
    let result_one: u16 = area_rec(&rec_one); // value is borrowed
    println!("Result One: {}", result_one);

    println!("rec_two = {:?}", rec_two);
    let result_two: u16 = area_rec(&rec_two); // value is borrowed
    println!("Result Two: {}", result_two);
}
