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

    pub fn print(&self) {
        println!("{}", self.val);

        match self.next {
            None => {},
            Some(ref n) => n.print()
        }
    }

    pub fn append(&mut self, new_value: i32) {
        match self.next {
            None => self.next = Some(Box::new(Node::new(new_value))),
            Some(ref mut n) => n.append(new_value),
        }
    }
}

fn main() {
    let mut x = Node::new(1);
    x.append(2);
    x.append(3);
    x.append(4);
    x.append(5);
    x.append(6);
    x.append(7);
    x.append(8);
    x.print();
}
