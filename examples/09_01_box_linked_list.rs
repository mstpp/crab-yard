#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    pub fn new(val: i32) -> Self {
        LinkedList {
            head: Some(Box::new(Node {
                value: val,
                next: None,
            })),
        }
    }

    pub fn push_head(&mut self, val: i32) {
        let old = self.head.take();
        self.head = Some(Box::new(Node {
            value: val,
            next: old,
        }));
    }
}

impl std::fmt::Display for LinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vals: Vec<String> = vec![];
        let mut current = self.head.as_ref();
        while let Some(following) = current {
            vals.push(following.value.to_string());
            current = following.next.as_ref();
        }
        write!(f, "[{}]", vals.join(" > "))
    }
}

pub fn main() {
    let mut l = LinkedList::new(1);
    println!("List init {:?}", &l);
    l.push_head(2);
    l.push_head(3);
    println!("Linked list: {}", &l);
}

// TODO
// len()
// Default trait
// pop_head
// push_tail
// pop_tail
// over generic type <T>
