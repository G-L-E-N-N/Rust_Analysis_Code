// Rust: Dynamic dispatch via trait objects
fn print_area(shape: &dyn Shape) {
    println!("{}", shape.area());
}
