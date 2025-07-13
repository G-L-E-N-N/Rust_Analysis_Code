pub fn pass_by_value() {
    let order = String::from("Order #1234: 2x Coffee");
    send_order(order); // Ownership is moved to the function

    println!("{}", order); // ERROR: order was moved and is no longer valid
}

fn send_order(order: String) { // Ownership of order is moved here
    println!("Sending order: {}", order); // Processing the moved order string
}
