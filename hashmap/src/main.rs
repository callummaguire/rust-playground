use std::collections::HashMap;

fn main() {
    let mut shapes = HashMap::new();
    shapes.insert("triangle", 3);
    shapes.insert("square", 4);

    println!("a square has so many sides {}", shapes["square"]);
   
    for (key, value) in &shapes {
        println!(" key => {} :  value => {}", key, value)
    }
    
    shapes.entry("circles").or_insert(1);
    println!("{:?}", shapes);
}
