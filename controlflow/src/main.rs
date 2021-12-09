fn if_statement() {
    let temp = 35;
    if temp > 30 {
        println!("really hot inside")
    }
    let day = temp ? "sunny" : "cloudy";
    println!("today is {}", day);
}  
fn while_and_loop() {
    let mut x = 1;
    while x < 1000
    {
        x *= 2;

        println!("x = {}", x);
    }
}

fn for_loop() {
    for x in 1...11 {
        println!("x = {}", x);
    }
}

fn main() {
    if_statement();
    println!("Hello, world!");
}
