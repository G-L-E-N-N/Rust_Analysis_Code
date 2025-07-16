use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>, // Next node wrapped in Rc<RefCell> for shared mutable ownership
}

struct CyclicList {
    head: Option<Rc<RefCell<Node>>>, // Start of the cyclic list
    tail: Option<Weak<RefCell<Node>>>, // Weak reference to the last node to avoid reference cycles
}

impl CyclicList {
    fn new() -> Self {
        CyclicList { head: None, tail: None }
    }

    fn add_node(&mut self, val: i32) {
        // Create a new node wrapped in Rc<RefCell> for shared ownership and interior mutability
        let new_node = Rc::new(RefCell::new(Node { value: val, next: None }));

        if let Some(head) = self.head.clone() {
            // If the list is not empty, update the current tail to point to the new node
            if let Some(tail_weak) = &self.tail {
                if let Some(tail) = tail_weak.upgrade() {
                    tail.borrow_mut().next = Some(new_node.clone());
                }
            }
            // New node points back to the head to maintain the cycle
            new_node.borrow_mut().next = Some(head);
            // Update tail to be the new node (as a weak reference)
            self.tail = Some(Rc::downgrade(&new_node));
        } else {
            // First node in the list points to itself to form a cycle
            new_node.borrow_mut().next = Some(new_node.clone());
            // Head and tail both point to the new node
            self.head = Some(new_node.clone());
            self.tail = Some(Rc::downgrade(&new_node));
        }
    }
}



fn main() {
    let mut list = CyclicList::new();

    list.add_node(1);
    list.add_node(2);
    list.add_node(3);

    if let Some(head) = &list.head {
        let mut current = Rc::clone(head);
        loop {
            print!("{} ", current.borrow().value);
            let next = current.borrow().next.as_ref().unwrap().clone();
            current = next;

            if Rc::ptr_eq(&current, &head) {
                break;
            }
        }
        println!();
    }
}
