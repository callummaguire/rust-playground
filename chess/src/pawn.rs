trait Move {
  fn move_piece(&self) {
    println!("the move has not been implemented yet");
  }
}

pub struct Pawn {
  name: &'static str,
  starting_place_x: i32,
  starting_place_y: i32,
  number: i32
}

impl Pawn {
  pub fn number(&self) -> &i32 {
    &self.number
  }
  pub fn starting_place_x(&self) -> &i32 
  {
    &self.starting_place_x  
  }

  pub fn starting_place_y(&self) -> &i32 
  {
    &self.starting_place_y  
  }
}
  
pub fn generate_pawn() -> Pawn {
  return Pawn{name: "Pawn", starting_place_x: 1, starting_place_y: 1, number: 1};
}
