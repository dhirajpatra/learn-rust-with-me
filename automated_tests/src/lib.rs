// automated_tests/src/lib.rs
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[cfg(test)]
// The tests module is only compiled when running tests
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }
}
