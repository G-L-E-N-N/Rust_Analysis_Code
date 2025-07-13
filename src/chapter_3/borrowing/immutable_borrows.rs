pub fn multiple_immutable_borrows_basic() {
    let s = String::from("Hello");

    let r1 = &s; // first immutable borrow of s
    let r2 = &s; // second immutable borrow of s

    println!("r1: {}, r2: {}", r1, r2);

    println!("Original: {}", s); // s is still valid here
}

pub fn multiple_immutable_borrows_config() {
    let config = String::from("server=localhost;port=8080");

    // Two immutable borrows to read different parts of the config
    let server = get_server(&config);
    let port = get_port(&config);

    println!("Server: {}, Port: {}", server, port);

    // 'config' remains valid and unchanged
}

fn get_server(config: &String) -> &str {
    config.split(';').next().unwrap_or("")
}

fn get_port(config: &String) -> &str {
    config.split(';').nth(1).unwrap_or("")
}

fn main() {
    multiple_immutable_borrows_basic();
    multiple_immutable_borrows_config();
}
