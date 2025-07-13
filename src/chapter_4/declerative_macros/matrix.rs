macro_rules! matrix {
    ($ty:ty; $($row:expr),*) => {{ 
        let mut _matrix = Vec::new();
        $(
            let _row: Vec<$ty> = $row.into_iter().collect(); // Convert each row
            _matrix.push(_row); // Add to matrix
        )* 
        _matrix
    }};
}

fn main() {
    let m = matrix!(i32; 
        vec![1, 2, 3],  // Vec
        (4..=6),        // Range 
        [7, 8, 9].iter().copied()  // Array iterator
    );
    println!("{:?}", m);
}
