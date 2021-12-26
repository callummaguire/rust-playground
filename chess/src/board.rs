#[path = "pawn.rs"]
mod pawn;

trait GameBoard {
  fn new(&self) -> Self;
}

#[derive(Debug)]
struct Board {
  game_state:  [[i32; 7]; 7]
}

pub fn start_board_state() {
  let mut board = Board {
    game_state: [
      [0, 0, 0, 0, 0, 0, 0 ], 
      [0, 0, 0, 0, 0, 0, 0 ],
      [0, 0, 0, 0, 0, 0, 0 ],
      [0, 0, 0, 0, 0, 0, 0 ],
      [0, 0, 0, 0, 0, 0, 0 ],
      [0, 0, 0, 0, 0, 0, 0 ],
      [0, 0, 0, 0, 0, 0, 0 ],
    ],
  };

 let pawn1 = pawn::generate_pawn();

  board.game_state[*pawn1.starting_place_x() as usize][*pawn1.starting_place_y() as usize] = *pawn1.number();

  println!("{:?}", board);
}