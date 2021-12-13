use std::ops::{Add};

trait Animal {
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

#[derive(Debug)]
struct Complex<T> {
    re: T,
    im: T
}


impl<T> Complex<T> {
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> {re, im}
    }    
}

impl Add for Complex<i32> {
    type Output = Complex<i32>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}



fn main() {

    let me = Human{name: "Callum"};
    
    me.talk();

    let mut a = Complex::new(1,2);
    let mut b = Complex::new(3,4);


    println!("{:?}", a + b);
}
