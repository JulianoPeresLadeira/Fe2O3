struct Test {
    val: i32
}

fn main() {
    let mut x = create_test();
    inc_val(&mut x);
    println!("{}", x.val);
}

fn create_test() -> Test {
    return Test {
        val: 0
    }
}

fn inc_val(y: &mut Test) {
    y.val += 1
}

