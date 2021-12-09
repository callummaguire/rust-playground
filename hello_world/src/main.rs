use std::mem;

fn main() {
    let a: u8 = 123;
    println!("a = {}", a);
    let mut b: i8 = 0;
    println!("b = {}", b);
    b = 42; 
    println!("b = {}", b);

    let c = 12341241;
    println!("c = {}, takes up bytes = {}",c, mem::size_of_val(&c));
}
