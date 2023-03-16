
fn main() {
    let mut x = create_string("Craig");
    append_last_name(&mut x, "John");
    println!("{}", x);
    
}

fn create_string(inp: &str) -> String {
    let x = String::from(inp);
    x
}

fn append_last_name(name: &mut String, last_name: &str) {
    name.push_str(" ");
    name.push_str(last_name);
}


