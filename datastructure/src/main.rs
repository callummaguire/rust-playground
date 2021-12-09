struct Point {
   x: f64,
   y: f64 
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8)  
}
fn enums() {
    let c: Color = Color::RgbColor(200,0,0);

    match c {
        Color::Red => println!("r"),
        Color::Blue => println!("b"),
        Color::Green => println!("r"),
        Color::RgbColor(0,0,0) => println!("blaack"),
        Color::RgbColor(r,g,b) => println!("rgb color: {}, {}, {}", r,g,b),
    }
}

fn main() {
    let Point { x: top_edge, y: left_edge } = Point { x: 3.0, y: 4.0};

    enums();
    println!("Hello, world! {}", top_edge);
}
