// Figure 5.7
// Without ? operator (manual handling)
fn find_email(user: &User) -> Option<&str> {
    match user.profile {
        Some(ref profile) => match profile.email {
            Some(ref email) => Some(email),
            None => None,
        },
        None => None,
    }
}

// With ? operator (concise propagation)
fn find_email_short(user: &User) -> Option<&str> {
    Some(user.profile?.email?)
}

// Figure 5.8 
struct Order {
    customer: Option<Customer>,
}

struct Customer {
    address: Option<Address>,
}

struct Address {
    street: Option<String>,
}

fn get_street(order: &Order) -> Option<&str> {
    // The '?' operator propagates None early if any intermediate value is missing
    Some(order.customer?.address?.street?.as_str())
}

// Figure 5.9
fn read_configs() -> Result<String, std::io::Error> {
    // If either read fails, the error is propagated immediately by '?'
    let config1 = std::fs::read_to_string("config.toml")?;
    let config2 = std::fs::read_to_string("settings.toml")?;
    Ok(config1 + &config2)
}

struct User {
    profile: Option<Profile>,
}

struct Profile {
    email: Option<&'static str>,
}

// Main:
fn main() {
    println!("== Option handling ==");

    let user_with_email = User {
        profile: Some(Profile {
            email: Some("user@example.com"),
        }),
    };

    let user_without_email = User {
        profile: Some(Profile { email: None }),
    };

    println!("Manual find_email: {:?}", find_email(&user_with_email));
    println!("With `?` operator: {:?}", find_email_short(&user_with_email));
    println!("Missing email: {:?}", find_email_short(&user_without_email));

    println!("\n== Nested Option handling ==");

    let order = Order {
        customer: Some(Customer {
            address: Some(Address {
                street: Some("Ruststraße 42".to_string()),
            }),
        }),
    };

    let street = get_street(&order);
    println!("Street: {:?}", street);

    println!("\n== Result handling ==");

    match read_configs() {
        Ok(cfg) => println!("Configs loaded:\n{}", cfg),
        Err(e) => println!("Failed to read configs: {}", e),
    }
}
