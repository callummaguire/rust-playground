use rand::prelude::SliceRandom;

fn main() {
    let consonants: Vec<_> = "abcdfghjklmnpqrstvwxyz".chars().collect();

    let mut rng = rand::thread_rng();

    println!("{}", consonants.choose(&mut rng).unwrap());
}
