static mut VAL: i32 = 0;

fn get_val() -> i32 {
    unsafe {
        VAL += 1;
        VAL
    }
}

macro_rules! clamp {
    ($x:expr, $low:expr, $high:expr) => {{
        let val = $x; // Evaluate $x exactly once
        if val < $low {
            $low
        } else if val > $high {
            $high
        } else {
            val
        }
    }};
}

fn main() {
    let result = clamp!(get_val(), 2, 5);
    unsafe {
        println!("Result: {}, VAL: {}", result, VAL);
    }
}
