pub fn types() {
    let x = 42;         // type inferred as i32
    let flag = true;    // inferred as bool

    let y: f64 = 3.14;  // explicit type declaration

    println!("x = {}", x);
    println!("flag = {}", flag);
    println!("y = {}", y);
}
