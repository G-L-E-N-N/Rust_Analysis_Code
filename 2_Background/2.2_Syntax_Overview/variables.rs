fn variables() {
    let x = 5;         // immutable variable
    let mut y = 10;    // mutable variable

    y = y + 1;         // allowed: mutable variable can be reassigned
    // x = 6;          // error: cannot assign twice to immutable variable

    println!("x = {}", x);
    println!("y = {}", y);
}
