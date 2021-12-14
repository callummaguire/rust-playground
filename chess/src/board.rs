trait GameBoard {
  fn new(&self) -> Self;
}

#[derive(Debug)]
struct Board {
  game_state:  [[i32; 7]; 7]
}

pub fn start_board_state() {
  let board = Board {
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

  println!("{:?}", board);
}