// Rust: Using traits and composition
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

// Rust: Dynamic dispatch via trait objects
fn print_area(shape: &dyn Shape) {
    println!("Area: {:.2}", shape.area());
}

fn main() {
    let circle = Circle { radius: 5.0 };
    print_area(&circle); // Dynamic dispatch via &dyn Shape
}

