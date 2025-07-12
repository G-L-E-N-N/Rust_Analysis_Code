use std::io;

pub fn drop_scope_simple() {
    let s = String::from("Natural scope end");
    println!("{}", s); // s is valid within this function
} // s is dropped here automatically at the end of the function

pub fn drop_scope_block() {
    {
        let s = String::from("Hello, Rust!");
        println!("{}", s); // s owns the string here
    } // s goes out of scope here, and its memory is automatically freed

    // println!("{}", s); // ERROR: s does not exist here anymore
}

pub fn drop_password_prompt() {
    println!("Enter your username:");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read username");
    let username = username.trim();

    println!("Welcome, {}!", username); // Last use of 'username'

    let max_retries = 3;
    for attempt in 1..=max_retries {
        println!("Attempt {}/{}: Enter password:", attempt, max_retries);

        let mut password = String::new();
        io::stdin()
            .read_line(&mut password)
            .expect("Failed to read password");
        if password.trim() == "secret" {
            println!("Access granted.");
            break;
        } else {
            println!("Incorrect password.");
            if attempt == max_retries {
                println!("Too many failed attempts.");
            }
        }
    }
}
