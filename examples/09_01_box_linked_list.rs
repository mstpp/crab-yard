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

    pub fn pop_head(&mut self) {
        self.head = self.head.take().and_then(|node| node.next);
        // let current_head = self.head.take();
        // let next = match current_head {
        //     Some(nx) => nx.next,
        //     None => None,
        // };
        // self.head = next;
    }

    pub fn len(&self) -> u64 {
        let mut current = &self.head;
        let mut len: u64 = 0;
        while let Some(next) = current {
            len = len + 1;
            current = &next.next;
        }
        len
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

impl Default for LinkedList {
    fn default() -> Self {
        LinkedList::new(i32::default())
    }
}

pub fn main() {
    let mut l = LinkedList::new(1);
    println!("List init {:?}", &l);
    l.push_head(2);
    l.push_head(3);
    l.push_head(4);
    l.push_head(5);
    println!("Linked list: {}", &l);
    // let size = l.len();
    println!("len of linked list: {}", l.len());
    l.pop_head();
    println!("Popped: {}", &l);
}
