// https://leetcode.com/problems/count-items-matching-a-rule/
pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
  let rule_index: usize = match &*rule_key {
    "type" => 0,
    "color" => 1,
    "name" => 2,
    _ => 0,
  };


  let number_of_items = items.iter().filter(|item| item[rule_index] == rule_value).count();
  number_of_items as i32
}