struct Node {
    value: i32,
    next: *mut Node, // raw pointer for manual cyclic reference
}

impl Node {
    fn new(val: i32) -> *mut Node {
        let node = Box::new(Node {
            value: val,
            next: std::ptr::null_mut(),
        });
        Box::into_raw(node) // ownership now unmanaged
    }
}

struct CyclicList {
    head: *mut Node,
}

impl CyclicList {
    fn new() -> Self {
        CyclicList {
            head: std::ptr::null_mut(),
        }
    }

    fn add_node(&mut self, val: i32) {
        let new_node = Node::new(val);

        if self.head.is_null() {
            self.head = new_node;
            (*new_node).next = self.head; // Error 1: raw pointer to self
        } else {
            let mut temp = self.head;
            while !(*temp).next.is_null() && (*temp).next != self.head {
                temp = (*temp).next;
            }
            (*temp).next = new_node;
            (*new_node).next = self.head; // Error 2: forming a cycle manually
        }
    }
}


fn main() {
    let mut list = CyclicList::new();

    list.add_node(1);
    list.add_node(2);
    list.add_node(3);
}

