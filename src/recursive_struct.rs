struct RecursiveNode {
    next: Option<Box<RecursiveNode>>
}

fn add(node: &mut RecursiveNode) {
    match node.next {
        Some(ref mut n) => add(&mut *n),
        None => {
            node.next = Some(Box::new(RecursiveNode {
                next: None
            }))
        }
    }
}

fn calculate_depth(node: &RecursiveNode) -> i32 {
    return recursive_calculation(node, 0);
}

fn recursive_calculation(node: &RecursiveNode, i: i32) -> i32 {
    return match &node.next {
        Some(n) => recursive_calculation(&n, i + 1),
        None => i
    }
}

fn main() {
    let mut s = RecursiveNode {
        next: None
    }; 
    
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);
    println!("{}", calculate_depth(&s));
    add(&mut s);

}

