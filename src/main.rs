// This code will give an compile time error, read its `notes.md` file

fn main() {
    let x = 10;
    let y = get_value();

    println!("x + y = {}", x + y);
}

fn get_value() -> &i32 {
    let y = 5;
    &y
}
