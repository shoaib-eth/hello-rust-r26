fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        return Err("Y is Zero".to_owned());
    } else {
        return Ok(x / y);
    }
}

fn main() {
    let result: Result<i32, String> = divide(10, 5);
    println!("Result: {:?}", result);
}
