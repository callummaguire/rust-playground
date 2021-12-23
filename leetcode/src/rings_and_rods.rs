// leetcode question link: https://leetcode.com/contest/weekly-contest-271/problems/rings-and-rods/

// step 1: write a loop that can get the string and the index
// step 1a: check the length of the string to see if it is even worth checking
// step 2: write a loop that enumerates by 2 
// step 3: create a index to push the rings into the right rods
// step 4: when adding rings append to the string so "RB" then when G comes then it is "RGB"
// step 5: check if the vector string has all the correct values

  pub fn count_points(rings: String) -> i32 {
    if rings.len() < 5 {
      return 0;
    }

    let mut vec = vec![String::from(""); 10];
    let mut chars = rings.chars().collect::<Vec<_>>();

    for (i, rod) in chars.iter().step_by(2).enumerate() {
        let index: usize = (chars[i * 2 + 1].to_string()).parse::<usize>().unwrap();

        vec[index].push(*rod);
    }

    vec.iter().filter(|s| s.contains("R") && s.contains("G") && s.contains("B")).count() as i32
  }