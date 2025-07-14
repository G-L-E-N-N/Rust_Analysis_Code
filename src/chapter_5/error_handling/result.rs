// Define a custom error type representing possible failure cases
enum FileError {
    NotFound,
    PermissionDenied,
    Unknown,
}

// Function that attempts to read the content of a file at a given path.
fn read_file(path: &str) -> Result<String, FileError> {
    // Simulated error conditions for demonstration purposes
    if path == "/forbidden.txt" {
        return Err(FileError::PermissionDenied);
    } else if path == "/missing.txt" {
        return Err(FileError::NotFound);
    }

    // On success, return dummy file content wrapped in Ok
    Ok("File content here".to_string())
}

fn main() {
    let path = "/forbidden.txt";

    // Call the function and handle the possible outcomes explicitly
    match read_file(path) {
        Ok(content) => println!("File content: {}", content),
        Err(FileError::NotFound) => println!("Error: File not found."),
        Err(FileError::PermissionDenied) => println!("Error: Permission denied."),
        Err(FileError::Unknown) => println!("Error: Unknown error occurred."),
    }
}
