macro_rules! debug_value {
    ($val:expr) => { // Takes any expression
        println!("{} = {:?}", stringify!($val), $val); // Auto-captures name and value
    };
}

fn main() {
    let a = 10;
    let b = true;
    let c = vec![1, 2, 3];
    debug_value!(a);
    debug_value!(b);
    debug_value!(c.len());
}
