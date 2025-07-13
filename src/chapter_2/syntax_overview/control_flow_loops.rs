pub fn control_flow() {
    for number in 1..=5 {
        if number % 2 == 0 {
            println!("{} is even", number);
        } else {
            println!("{} is odd", number);
        }
    }
}

fn main() {
    control_flow();
}
