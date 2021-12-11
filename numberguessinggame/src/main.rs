use rand::Rng;
use std::io::stdin;

fn main() {
    let guess_number = rand::thread_rng().gen_range(0,10);
    
    loop {
        println!("please guess a number between 1 and 10");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 10 {
                            println!("your guess is out of range");
                        } else if guess < guess_number {
                            println!("your guess is lower than the number");
                        } else if guess > guess_number {
                            println!("Your number is too high");
                        } else {
                            println!("you guessed correctly");
                            break;
                        }
                    }
                    Err(e) => {
                        println!("cound not read your input {}", e);
                    }
                }
            },
            Err(_) => {
                println!("error");
                continue
            }
        }

    }
}
