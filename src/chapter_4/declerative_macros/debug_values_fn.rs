fn debug_value<T: std::fmt::Debug>(label: &str, value: T) {  // Takes label + value
    println!("{} = {:?}", label, value); // Manual string-value pairing
}

fn main() {
    let a = 10;
    let b = true;
    let c = vec![1, 2, 3];
    
    debug_value("a", a); // Must repeat variable name as string
    debug_value("b", b);
    debug_value("c.len()", c.len());
}
