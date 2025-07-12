fn variables() {
    let x = 5;         // immutable variable: value cannot be changed after initialization
    let mut y = 10;    // mutable variable: value can be modified after initialization

    y = y + 1;         // valid operation: mutable variable reassigned
    // x = x + 1;      // error: cannot assign to an immutable variable
    // x = 6;          // error: immutable variable cannot be modified

    println!("x = {}", x);
    println!("y = {}", y);
}
