// Simple user struct with an ID and a name
struct User {
    id: u32,
    name: String,
}

// Function to search for a user by ID in a slice of users.
fn find_user(id: u32, users: &[User]) -> Option<&User> {
    // Iterate over each user in the slice
    for user in users {
        if user.id == id {
            return Some(user); // Return the found user wrapped in Some
        }
    }
    return None; // Return None if no user matches the given ID
}

fn main() {
    // Example list of users
    let users = vec![
        User { id: 1, name: "Alice".to_string() },
        User { id: 2, name: "Bob".to_string() },
        User { id: 3, name: "Charlie".to_string() },
    ];

    let id_to_find = 2;

    // Handle the result of the search explicitly
    match find_user(id_to_find, &users) {
        Some(user) => println!("Found user: {} with ID {}", user.name, user.id),
        None => println!("No user found with ID {}", id_to_find),
    }

    let id_to_find_2 = 5;
    
    match find_user(id_to_find, &users) {
        Some(user) => println!("Found user: {} with ID {}", user.name, user.id),
        None => println!("No user found with ID {}", id_to_find),
    }
}
