pub fn use_after_free() {
    let s = String::from("hello");
    let r = &s;              // Immutable borrow of `s`
    drop(s);                 // Attempt to move `s`, which would deallocate the memory
    println!("{}", r);       // Use of `r`, which refers to `s`
}
