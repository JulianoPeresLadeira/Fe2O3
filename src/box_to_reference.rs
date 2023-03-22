fn test(x: &mut String) {
    x.push_str(" appended");
    println! ("{}", x);
}

fn main() {
    let mut x = Box::new(String::from("Test"));
    test(&mut *x);
    println! ("{}", x);
}

