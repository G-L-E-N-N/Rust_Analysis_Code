fn foo(x: &mut i32) -> i32 {
    *x += 1;
    println!("foo called, x is now {}", x);
    *x
}

fn bar(y: &mut i32) -> i32 {
    *y += 2;
    println!("bar called, y is now {}", y);
    *y
}

macro_rules! min {
    ($x:expr, $y:expr) => {{
        let x_val = $x;
        let y_val = $y;
        if x_val < y_val { x_val } else { y_val }
    }};
}

fn main() {
    let mut a = 5;
    let mut b = 5;

    let result = min!(foo(&mut a), bar(&mut b));

    println!("Result: {}", result);
    println!("Final a: {}", a);
    println!("Final b: {}", b);
}
