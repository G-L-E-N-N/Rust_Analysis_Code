extern "C" {
    fn multiply(a: i32, b: i32) -> i32;
}

fn main() {
    unsafe {
    let result = multiply(3, 4);
    println!("3 * 4 = {}", result);
    }
}
