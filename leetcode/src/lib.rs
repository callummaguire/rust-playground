mod rings_and_rods;

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
}

