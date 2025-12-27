fn main() {
    let num1: u8 = 10;
    let num2: u8 = 20;
    let result: u8 = add(num1, num2);
    println!("The sum of {} and {} is {}", num1, num2, result);

    hello("Alice"); 
    age(30);
}

fn add(a: u8, b: u8) -> u8 {  // It is compulsory to specify the return type by arrow -> 
    return a + b;
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn age(age: u8) {
    println!("You are {} years old.", age);
}