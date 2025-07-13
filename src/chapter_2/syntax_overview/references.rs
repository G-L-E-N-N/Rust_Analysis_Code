pub fn references() {
    let w = 10;
    let x = &w;  // x is a reference to w
    println!("Value via reference: {}", x);

    let mut y = 5;
    let z = &mut y;  // z is a mutable reference to y
    *z += 1;         // dereference z to change the value
    println!("y is now: {}", y);
}

fn main() {
    references();
}
