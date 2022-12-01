#![allow(unused)]

/// Parses an integer string
///
/// This is faster than `str::parse` because it assumes the input consists of only digits.
pub fn fast_parse_int(s: &str) -> usize {
    debug_assert!(!s.is_empty());
    debug_assert!(s.len() < usize::MAX.to_string().len()); // err on the side of caution
    debug_assert!(s.chars().all(|c| c.is_ascii_digit()));

    s.bytes().fold(0, |a, c| a * 10 + (c & 0x0f) as usize)
}

/// Parses an int string with a max length of 8
///
/// This is faster than `str::parse` because it assumes the input consists of only digits and is up
/// to 8 characters long.
pub fn fast_parse_int_8(s: &str) -> usize {
    debug_assert!(!s.is_empty());
    debug_assert!(s.len() <= 8);
    debug_assert!(s.chars().all(|c| c.is_ascii_digit()));

    s.bytes()
        .take(8)
        .fold(0, |a, c| a * 10 + (c & 0x0f) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_parse_int() {
        assert_eq!(fast_parse_int("1"), 1);
        assert_eq!(fast_parse_int("0"), 0);
        assert_eq!(fast_parse_int("1234"), 1234);
        assert_eq!(fast_parse_int("0001"), 1);
        assert_eq!(fast_parse_int("0000"), 0);
        assert_eq!(fast_parse_int("12345678"), 12345678);
    }

    #[test]
    fn test_fast_parse_int_8() {
        assert_eq!(fast_parse_int_8("1"), 1);
        assert_eq!(fast_parse_int_8("0"), 0);
        assert_eq!(fast_parse_int_8("1234"), 1234);
        assert_eq!(fast_parse_int_8("0001"), 1);
        assert_eq!(fast_parse_int_8("0000"), 0);
        assert_eq!(fast_parse_int_8("12345678"), 12345678);
    }
}
