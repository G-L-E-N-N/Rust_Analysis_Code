macro_rules! report {
    () => {
        println!("[INFO] Operation executed.");
    };
    ($action:expr) => {
        println!("[INFO] Operation: {}", $action);
    };
    ($action:expr, $duration:expr) => {
        println!("[INFO] Operation: {} (took {} ms)", $action, $duration);
    };
}

fn main() {
    report!();
    report!("DB Insert");
    report!("File Export", 128);
}
