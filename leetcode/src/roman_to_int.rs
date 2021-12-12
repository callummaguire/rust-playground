pub fn roman_to_int(s: String) -> i32 {
  let mut sum: i32 = 0;
  let mut last: i32 = 0;
  for roman_char in s.chars().rev() {
    let curr = match_roman_to_int(roman_char);
    if curr >= last {
      sum += curr;
    } else {
      sum -= curr;
    }
    last = curr;
  }
  sum
}

fn match_roman_to_int(roman_char: char) -> i32 {
     match roman_char {
      'I' => 1,
      'V' => 5,
      'X' => 10,
      'L' => 50,
      'C' => 100,
      'D' => 500,
      'M' => 1000,
      _ => 0
    } 
}