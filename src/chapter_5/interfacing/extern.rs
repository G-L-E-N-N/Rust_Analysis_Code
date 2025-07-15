extern "C" {
    fn multiply(a: i32, b: i32) -> i32;
}

fn main() {
    let x = 6;
    let y = 7;
    let result = unsafe { multiply(x, y) };
    println!("{} * {} = {}", x, y, result);
}
