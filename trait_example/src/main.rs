
trait Animal {
    fn create(name: &'static str) -> self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

impl Animal for Human {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello", &self.name());
    }
}

fn main() {

    let me = Human{name: "Callum"};
    
    me.talk();
    println!("Hello, world! {}", me.name());
}
