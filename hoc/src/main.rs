fn is_even(number: u32) -> bool {
    number % 2 == 0
}
// currying
fn greater_than(limit: u32) -> impl Fn(u32) -> bool {
   move |y| y > limit
}
// take even number and sum them up to 500
fn main() {

    let limit = 500;
    let mut sum = 0;

    // let above_limit = |y| y < limit;

    let above_limit = greater_than(500);
    for i in 0.. {
        let isq = i*i;

        if above_limit(isq) {
            break;
        } 

        if is_even(isq) {
            sum += isq;
        }
    }
    println!("loop sum => {}", sum);

    let sum2 = (0..).map(|x| x*x).take_while(|&x| x< limit).filter(|x: &u32| is_even(*x)).fold(0, |sum, x| sum + x);
    println!("hoc => {}", sum2);
}
