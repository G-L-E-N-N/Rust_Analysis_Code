struct Node {
    value: i32,
    next: Option<*mut Node>, // raw pointer!
}

struct CyclicList {
    head: Option<*mut Node>,
}

impl CyclicList {
    fn new() -> Self {
        CyclicList { head: None }
    }

    fn add_node(&mut self, val: i32) {
        // allocate node on the heap manually (like new in C++)
        let new_node = Box::into_raw(Box::new(Node { value: val, next: None }));

        unsafe {
            if self.head.is_none() {
                (*new_node).next = Some(new_node); // self-cycle
                self.head = Some(new_node);
            } else {
                let mut temp = self.head.unwrap();
                while let Some(next_ptr) = (*temp).next {
                    if next_ptr == self.head.unwrap() {
                        break;
                    }
                    temp = next_ptr;
                }

                (*temp).next = Some(new_node);
                (*new_node).next = self.head;
            }
        }
    }
}

fn main() {
    let mut list = CyclicList::new();

    list.add_node(10);
    list.add_node(20);
    list.add_node(30);

    unsafe {
        if let Some(mut current) = list.head {
            for _ in 0..3 {
                println!("{}", (*current).value);
                current = match (*current).next {
                    Some(next_ptr) => next_ptr,
                    None => break,
                };
            }
        } else {
            println!("Liste ist leer");
        }
    }
}


