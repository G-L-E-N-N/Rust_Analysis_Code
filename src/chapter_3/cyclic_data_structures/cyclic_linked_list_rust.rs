use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

struct CyclicList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Weak<RefCell<Node>>>,
}

impl CyclicList {
    fn new() -> Self {
        CyclicList { head: None, tail: None }
    }

    fn add_node(&mut self, val: i32) {
        let new_node = Rc::new(RefCell::new(Node { value: val, next: None }));

        if let Some(head) = self.head.clone() {
            if let Some(tail) = self.tail.take().and_then(|w| w.upgrade()) {
                tail.borrow_mut().next = Some(new_node.clone());
            }
            self.tail = Some(Rc::downgrade(&new_node));
            new_node.borrow_mut().next = Some(head);
        } else {
            new_node.borrow_mut().next = Some(new_node.clone());
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
