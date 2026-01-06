#[derive(Debug)]

// we can define data types in enums variants
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

const PI: f64 = 3.14;

// `impl` is `Shape` type which is enum,
// it denotes the `Shape` enum
impl Shape {
    // Associative Functions
    fn new_circle(radius: f64) -> Self {
        Self::Circle(radius)
    }

    fn new_rectangle(length: f64, breadth: f64) -> Self {
        Self::Rectangle(length, breadth)
    }

    // we dont have to create two seperate method functions for calculate `circle` and `rectangle`, we can do these both task in only `match()` function, when circle is called, the area of circle will be calculated and vice versa.
    // Method
    fn calculate(&self) {
        match self {
            Shape::Circle(radius) => println!("Area Of Circle: {}", PI * radius * radius),
            Shape::Rectangle(length, breadth) => {
                println!("Area Of Rectangle: {}", length * breadth)
            }
        }
    }
}

fn main() {
    let circle = Shape::new_circle(5.0);
    println!("Circle: {:?}", circle);

    let rectangle = Shape::new_rectangle(5.0, 2.5);
    println!("Rectangle: {:?}", rectangle);

    circle.calculate();
    rectangle.calculate();
}
