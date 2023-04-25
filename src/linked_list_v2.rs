struct Node {
    val: i32,
    next: Option<Box<Node>>
}

impl Node {
    fn new(val: i32) -> Self {
        Node {
            val, 
            next: None 
        }
    }
}

struct List {
    head: Option<Box<Node>>
}

impl List {
    fn new() -> Self {
        List {
            head: None
        }
    }

    fn insert(&mut self, val: i32) {
        if self.head.is_none() {
            self.head = Some(Box::new(Node::new(val)));
            return;
        }

        let mut curr = &mut self.head;

        loop {
            if let Some(ref mut curr_node) = curr {
                if curr_node.next.is_none() {
                    curr_node.next = Some(Box::new(Node::new(val)));
                    return;
                }
    
                curr = &mut curr_node.next;
    
            }
        }
    }

    fn remove(&mut self, val: i32) {
        if let None = self.head {
            return;
        }

        if let Some(ref mut head_node) = &mut self.head {
            if head_node.val == val {
                self.head = head_node.next.take();
            }
        }

        let mut curr = &mut self.head;
        
        loop {
            match curr {
                None => return,
                Some(ref mut curr_node) => {
                    if let Some(ref mut next_node) = curr_node.next {
                        if next_node.val == val {
                            curr_node.next = next_node.next.take();
                            return;
                        }
                    }
                    curr = &mut curr_node.next;
                }
            }
        }  
    }

    fn includes(&self, val: i32) -> bool {
        let mut curr = &self.head;

        while let Some(node_box) = curr {
            if node_box.as_ref().val == val {
                return true;
            }

            curr = &node_box.as_ref().next;
        }

        false
    }
}

fn main() {
    let mut list = List::new();

    list.insert(1);
    println!("Has 1: {}", list.includes(1));
    println!("Has 2: {}", list.includes(2));
    println!("Has 3: {}", list.includes(3));

    list.insert(2);
    println!("Has 1: {}", list.includes(1));
    println!("Has 2: {}", list.includes(2));
    println!("Has 3: {}", list.includes(3));

    list.insert(3);
    println!("Has 1: {}", list.includes(1));
    println!("Has 2: {}", list.includes(2));
    println!("Has 3: {}", list.includes(3));

    list.remove(2);
    println!("");
    println!("Has 1: {}", list.includes(1));
    println!("Has 2: {}", list.includes(2));
    println!("Has 3: {}", list.includes(3));
}