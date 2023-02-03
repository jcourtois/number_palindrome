#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    // 4ms
    pub fn is_palindrome(mut x: i32) -> bool {
        if x.is_negative() {
            return false;
        }
        let x = x.to_string();
        let length = x.len() / 2;
        let front_slice = x[..length].chars();
        let back_slice = x[length..].chars().rev();
        front_slice
            .zip(back_slice)
            .all(|(front_char, back_char)| front_char == back_char)
    }

    // 16ms
    pub fn is_palindrome_no_string(mut x: i32) -> bool {
        if x.is_negative() {
            return false;
        }
        let mut digit = 0;
        let mut hash = HashMap::new();
        while x > 0 {
            hash.insert(digit, x % 10);
            digit += 1;
            x /= 10;
        }

        (0..digit / 2)
            .map(|n| (hash.get(&n), hash.get(&(digit - 1 - n))))
            .all(|(start, end)| start == end)
    }
}

#[cfg(test)]
mod palindromes {
    use super::*;

    #[test]
    fn _0_is() {
        assert_eq!(Solution::is_palindrome(0), true);
    }

    #[test]
    fn _single_digit_numbers_are() {
        for n in 1..=9 {
            assert_eq!(Solution::is_palindrome(n), true);
        }
    }

    #[test]
    fn _negatives_are_not() {
        assert_eq!(Solution::is_palindrome(-1), false);
        assert_eq!(Solution::is_palindrome(-111), false);
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    #[test]
    fn _longer_numbers_work() {
        assert_eq!(Solution::is_palindrome(111), true);
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(12222221), true);
    }

    #[test]
    fn _non_palindromes_are_not() {
        assert_eq!(Solution::is_palindrome(122), false);
        assert_eq!(Solution::is_palindrome(12224678), false);
    }

    #[test]
    fn _10_is_not() {
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
