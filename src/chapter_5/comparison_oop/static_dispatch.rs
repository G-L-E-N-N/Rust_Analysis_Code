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

// Rust: Static dispatch via generics
fn print_area<T: Shape>(shape: &T) {
    println!("{}", shape.area());
}

fn main() {
    let circle = Circle { radius: 5.0 };
    print!("Circle area: ");
    print_area(&circle); // Statischer Dispatch zur Compile-Zeit
}
