use std::sync::{Arc, Mutex};
use std::thread;

pub fn parallel_sum() {
    let data: Arc<[i32]> = Arc::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]); // Shared immutable slices 
    let sum = Arc::new(Mutex::new(0)); // Shared sum variable protected by Mutex
    let mut handles = vec![];
    // Start multiple threads, each one sums a part of the data
    for i in 0..4 {
        let data_clone = Arc::clone(&data);
        let sum_clone = Arc::clone(&sum);

        let handle = thread::spawn(move || {
            let start = i * (data_clone.len() / 4);
            let end = if i == 3 { data_clone.len() } else { (i + 1) * (data_clone.len() / 4) };

            let partial_sum: i32 = data_clone[start..end].iter().sum();

            // Lock the mutex and add the partial sum to the total
            let mut total = sum_clone.lock().unwrap();
            *total += partial_sum;
        });
        handles.push(handle);
    }
    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Total sum: {}", *sum.lock().unwrap());
}

fn main() {
    parallel_sum();
}
