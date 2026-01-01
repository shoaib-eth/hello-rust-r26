fn main() {
    let number = 12;

    if number % 3 == 0 && number % 4 == 0 {
        println!("The number is divisible by both 3 and 4.");
    } else if number % 3 == 0 {
        println!("The number is divisible by 3 but not by 4.");
    } else if number % 4 == 0 {
        println!("The number is divisible by 4 but not by 3.");
    } else {
        println!("The number is neither divisible by 3 nor by 4.");
    }
}
