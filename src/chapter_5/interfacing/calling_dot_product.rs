// Rust code
extern "C" {
    fn dot_product(a: *const f32, b: *const f32, len: usize) -> f32;
}

fn main() {
    let vec1 = [1.0_f32, 2.0, 3.0];
    let vec2 = [4.0_f32, 5.0, 6.0];

    unsafe {
        let result = dot_product(vec1.as_ptr(), vec2.as_ptr(), vec1.len());
        println!("Dot product result: {}", result);
    }
}
