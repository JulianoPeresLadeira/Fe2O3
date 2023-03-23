struct Node {
    val: i32,
    next: Option<Box<Node>>
}

impl Node {
    pub fn new(value: i32) -> Self {
        Self {
            val: value,
            next: None
        }
    }
}

struct LinkedList {
    length: u32,
    head: Option<Box<Node>>
}

impl LinkedList {
    pub fn new() -> Self {
        Self {
            length: 0,
            head: None
        }
    }

    pub fn append(&mut self, value: i32) {
        match self.head {
            None => self.head = Some(Box::new(Node::new(value))),
            Some(ref mut n) => {
                let mut curr: &mut Node = n.as_mut();
                while curr.next.is_some() {
                    let curr_box = curr.next.as_mut().unwrap();
                    curr = curr_box.as_mut();
                }
                curr.next = Some(Box::new(Node::new(value)))
            },
        }

        self.length += 1;
    }

    pub fn print(&self) {
        let mut curr = &self.head;
        while curr.is_some() {
            let curr_node = curr.as_ref().unwrap();
            println!("{}", curr_node.val);
            curr = &curr_node.next;
        }
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.append(1);
    list.append(2);
    list.append(3);
    list.append(4);
    list.append(5);
    list.print();
}
