pub fn pass_by_reference() {
    let order = String::from("Order #1234: 2x Coffee");
    send_order(&order); // Ownership is not moved; just borrowed as immutable reference
    println!("{}", order); // order is still valid here
}

fn send_order(order: &String) { // Immutable borrow
    println!("Sending order: {}", order);
}

pub fn multiple_pass_by_reference() {
    let mut order = Order {
        details: String::from("Order #1234: 2x Coffee, 1x Croissant"),
        price: 12.50, // Initialize order price
    };
    print_order(&order); // Print order details (immutable borrow)
    apply_discount(&mut order); // Check and apply discount (mutable borrow)
    println!(
        "Final order summary:\nDetails: {}\nTotal price: ${:.2}",
        order.details, order.price
    ); // Print final updated order
}

struct Order {
    details: String,
    price: f32,
}

fn print_order(order: &Order) { // Takes an immutable reference for read-only access
    println!(
        "Customer order:\n{}\nTotal price: ${:.2}\n",
        order.details, order.price
    );
}

fn apply_discount(order: &mut Order) { // Takes a mutable reference to modify the order
    if order.price > 20.0 {
        order.details.push_str(", incl. free cookie"); // Add free cookie to details
        order.price -= 2.0; // Apply discount
        println!("Big spender bonus! Free cookie + $2 discount applied.\n");
    } else if order.price > 10.0 {
        order.details.push_str(", incl. free cookie"); // Add free cookie to details
        println!("Free cookie added for your order!\n");
    } else {
        println!("No discount or free cookie this time."); // No changes applied
    }
}

fn main() {
    pass_by_reference();
    multiple_pass_by_reference();
}
