mod two_sum;
mod roman_to_int;

fn main() {
    let nums = vec![2, 7, 11, 15];
    let output = two_sum::two_sum(nums, 9);
    
    let num = roman_to_int::roman_to_int("III".to_string());
    
    println!("Hello, world! {:?}", num);
}
