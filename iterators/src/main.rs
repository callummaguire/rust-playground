fn main() {
    let vec = vec![1,2,3];

    // it has to be with & otherwise it moved and changes the reference above
    for x in &vec {
        println!("{}", x);
    }

    // by reference
    for x in vec.iter() {
        println!("we got {}", x);
    }

    // by reference and mutation
    let mut vec2 = [1,2,3];

    for y in vec2.iter_mut() {
        *y += 2;
    }

    println!("{:?}", vec2);
}
