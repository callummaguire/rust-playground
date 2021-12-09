mod two_sum;

fn main() {
    let nums = vec![2, 7, 11, 15];
    let output = two_sum::two_sum(nums, 9);
    println!("Hello, world! {:?}", output);
}
