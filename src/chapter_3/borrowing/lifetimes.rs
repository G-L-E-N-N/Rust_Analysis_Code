// Function demonstrating automatic lifetime elision
fn first_char(s: &str) -> &str {
    &s[0..1]
}

// Function demonstrating explicit lifetime annotation
fn longest(s1: &str, s2: &str) -> &str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {

    // Using automatic lifetime elision
    let word = String::from("Rust");
    let ch = first_char(&word);
    println!("The first character is: {}", ch);

    // Using explicit lifetime annotations
    let string1 = String::from("Programming");
    let string2 = String::from("Language");

    let result = longest(&string1, &string2);
    println!("The longer string is: {}", result);
}
