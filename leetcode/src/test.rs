#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn two_sum_tests() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
