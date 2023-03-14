fn main() {
    let mut x: i32 = 1;
    inc_val(&mut x);
    println!("{}", x);
}

fn inc_val(y: &mut i32) {
    *y += 1
}
