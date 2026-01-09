type Kilometer = i32;

fn add_distance(x: Kilometer, y: Kilometer) -> Kilometer {
    return x + y;
}

fn main() {
    let a: Kilometer = 5;
    let b: Kilometer = 10;
    println!("Total: {}", add_distance(a, b));
}

// Here we defined Kilometer type
