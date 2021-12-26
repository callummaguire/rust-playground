mod board;
use std::io::stdin;

fn main() {

    board::start_board_state();

    println!("please make a move");
    let mut buffer = String::new();
    match stdin().read_line(&mut buffer) {
        Ok(_) => {
            let parsed = buffer.trim_end().parse::<String>();
            match parsed {
                Ok(player_move) => {
                    println!("move {}" , player_move);
                }
                Err(e) => {
                    println!("cound not read your input {}", e);
                }
            }
        },
        Err(e) => {
            println!("{}", e)
        }
    }
    println!("Hello, world!");
}
