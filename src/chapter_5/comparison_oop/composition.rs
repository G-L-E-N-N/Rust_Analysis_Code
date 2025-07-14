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

fn main() {
    let circle = Circle { radius: 5.0 };
    println!("Circle area: {:.2}", circle.area());
}
