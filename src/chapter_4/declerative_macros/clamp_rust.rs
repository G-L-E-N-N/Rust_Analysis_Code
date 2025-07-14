macro_rules! clamp {
    ($x:expr, $low:expr, $high:expr) => {{
        let val = $x;
        if val < $low {
            $low
        } else if val > $high {
            $high
        } else {
            val
        }
    }};
}

fn get_val(val: &mut i32) -> i32 {
    *val += 1;
    *val
}

fn main() {
    let mut val = 0;

    let result = clamp!(get_val(&mut val), 2, 5);

    println!("Result: {}, val: {}", result, val);
}

