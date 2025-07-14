// Rust: Static dispatch via generics
fn print_area<T: Shape>(shape: &T) {
    println!("{}", shape.area());
}
