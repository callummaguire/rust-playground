use std::collections::HashMap;

// https://leetcode.com/problems/isomorphic-strings/
pub fn is_isomorphic(s: String, t: String) -> bool {
   let mut complements = HashMap::new();
    
   for (i,letter) in s.chars().enumerate() {
     let alternative_letter  = t.chars().nth(i).unwrap();
      if complements.contains_key(&letter) && alternative_letter != complements[&letter] {
        return false
      }
      if !complements.contains_key(&letter) && complements.values().any(|&x| x == alternative_letter) {
        return false
      }
      complements.insert(letter, alternative_letter);
   }
   true
}