const g:i32 = 123;
fn scope() {
    let a = 123;
    {
        let b =456;
        println!("inside, b = {}", b);
        println!("inside, a = {}", a);
        println!("inside, G = {}", g);
    }
}
fn main() {
    println!("Hello, world!");
    let mut a = 1;
    a += 1;
    let a_cubed = i32::pow(a, 3);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; // true

    let x = 5;
    let x_is_5 = x == 5;
    scope();
}
