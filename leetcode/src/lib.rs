mod rings_and_rods;

mod is_isomorphic;
mod binary_search;

mod count_matches;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rings_and_rods_small_string_returns_0() {
      let value = rings_and_rods::count_points("G4".to_string());
      assert_eq!(0, value);
    }

    #[test]
    fn rings_and_rods_small_string() {
      let value = rings_and_rods::count_points("G4B0R0G0R9R0B0G0".to_string());
      println!("{}", value);
      assert_eq!(1, value);
    }

    
    #[test]
    fn is_isomorphic_return_true() {
      let value = is_isomorphic::is_isomorphic("title".to_string(),"paper".to_string());
      assert_eq!(value, true)
    }

    #[test]
    fn is_isomorphic_return_false() {
      let value = is_isomorphic::is_isomorphic("foo".to_string(),"bar".to_string());
      assert_eq!(value, false)
    }

    #[test]
    fn binary_search_return_index() {
      let value = binary_search::search(vec![-1,0,3,4,5,9,12], 3);
      assert_eq!(value, 2)
    }
    
    #[test]
    fn binary_search_return_index_not_found() {
      let value = binary_search::search(vec![-1,0,3,4,5,9,12], 20);
      assert_eq!(value, -1)
    }

    fn count_matches_good_path() {
      assert_eq!(count_matches::count_matches(vec![vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()]], "type".to_string(), "phone".to_string()), 1)
    }
}

