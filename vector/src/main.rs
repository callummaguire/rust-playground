fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    println!("a = {:?}", a);

    let b = a.pop();
    println!("b => {:?}", b);
}

fn main() {
    vectors();
    println!("Hello, world!");
}
