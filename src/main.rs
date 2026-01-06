enum Color {
    Red,
    Green,
    Yellow,
    Blue,
}

fn main() {
    let color: Color = Color::Green;

    match color {
        // it check the which color is in `color` option, and if it matches it will print it.
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
        Color::Red => println!("Red"),
        Color::Yellow => println!("Yellow"),
    }
}
