fn main() {
    let x = String::from("Hello");
    let consume_and_return_x = || &x;
    println!("{}", x);
    let y = consume_and_return_x();
    println!("{}", y);

    let z = y;
    println!("{}", z);
}
