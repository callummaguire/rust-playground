fn functions_example(x: i32) {
    println!("value = {}", x);
}

fn main() {
    functions_example(32);
    let x = product(10, 5);
    println!("Hello, world! {}", x);
    let chair_legs = methods();
    println!("Hello, world! {:?}", chair_legs);
}

fn product(x: i32, y: i32) -> i32 {
    x * y
}

struct Chair {
    legs: i32,
}

impl Chair {
    fn double_chair(&self) -> i32 {
        self.legs * 2
    }
}

fn methods() -> i32 {
    let chair = Chair { legs: 2 };
    let chair_legs = chair.double_chair();
    chair_legs
}

fn say_hello() {
    println!("hello");
}

fn closures() {
    let sh = say_hello;
    sh();

    let plus_one =  |x: i32| -> 
}
