pub fn mutable_borrow_and_modify() {
    let mut s = String::from("Hello");

    let r1 = &mut s; // mutable borrow of s
    r1.push_str(", Rust!"); // modifying the borrowed value

    println!("Modified: {}", r1);
    println!("Original: {}", s); // now s holds the modified value
}

pub fn multiple_mutable_borrows_illegal() {
    let mut s = String::from("Hello");

    let r1 = &mut s; // first mutable borrow

    let r2 = &mut s; // ERROR: cannot borrow `s` as mutable more than once at a time
    let r3 = &s;     // ERROR: cannot borrow `s` as immutable while it is also borrowed as mutable

    println!("r1: {}", r1);

    println!("r2: {}", r2);
    println!("r3: {}", r3);
}

pub fn sorting_mut_slice_example() {
    let mut numbers = [5, 3, 8, 1, 2];
    println!("Before sorting: {:?}", numbers);

    bubble_sort(&mut numbers);

    println!("After sorting: {:?}", numbers);
}

fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
