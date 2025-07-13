struct Point {
    x: f64,
    y: f64,
}

pub fn structs() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("x = {}, y = {}", p.x, p.y);
}

fn main() {
    structs();
}
