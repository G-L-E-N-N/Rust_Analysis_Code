pub fn dangling_reference_example() -> &String {
    let s = String::from("Hello");
    
    &s // Error: returns reference to local variable that will be dropped
}
