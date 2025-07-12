pub fn ownership_move_simple() {
    let file_path = String::from("/tmp/data.txt");
    let new_path = file_path; // Ownership moved to new_path

    // println!("{}", file_path); // Error: file_path was moved!
    println!("File will be saved to: {}", new_path); // OK: new_path owns the string
}

pub fn ownership_move_conditional() {
    let data = String::from("fallback data");

    if data.contains("config") {
        let config = data; // Ownership is moved
        println!("Using configuration: {}", config);
    } else {
        println!("Using any data: {}", data); // No move in else
    }

    println!("Data: {}", data); // Error: Ownership was moved
}
