struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

struct CyclicList {
    head: Option<Box<Node>>,
}

impl CyclicList {
    fn new() -> Self {
        CyclicList { head: None }
    }

    fn add_node(&mut self, val: i32) {
        let mut new_node = Box::new(Node { value: val, next: None });

        if self.head.is_none() {
            new_node.next = Some(new_node);        // Error 1
            self.head = Some(new_node);            // Error 2
        } else {
            let mut temp = self.head.as_mut().unwrap(); // mutable borrow

            while let Some(ref mut next_node) = temp.next {
                if std::ptr::eq(&**next_node, self.head.as_ref().unwrap()) { // Error 3
                    break;
                }
                temp = next_node;
            }

            temp.next = Some(new_node);
            temp.next.as_mut().unwrap().next = self.head.clone(); // Error 4
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
