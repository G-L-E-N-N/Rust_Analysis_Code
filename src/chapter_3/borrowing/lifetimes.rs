fn first_char(s: &str) -> &str {
    &s[0..1]
}

fn main() {
    let word = String::from("Rust");
    let ch = first_char(&word);
    println!("The first character is: {}", ch);
}
