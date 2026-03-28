fn main() {
    let mut x = 10;
    let r = &mut x;
    *r = 20;
    println!("r: {}", r);
    println!("*r: {}", *r);
    println!("x: {}", x);
}
