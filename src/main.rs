#[derive(Debug)]

struct Circle {
    radius: f64,
}

const PI: f64 = 3.14;

trait Shape {
    fn area_of_circle(&self) -> f64;
}

impl Shape for Circle {
    fn area_of_circle(&self) -> f64 {
        return PI * self.radius * self.radius;
    }
}

fn main() {
    let circle: Circle = Circle { radius: 5.0 };
    println!("Area of circle: {:?}", circle.area_of_circle());
}
