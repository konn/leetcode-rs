use crate::solution::Solution;

// Body starts here
use std::iter;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x == 0 {
            true
        } else if x < 0 {
            false
        } else {
            let n = f32::log10(x as f32) as usize + 1;
            let mut x = x;
            let mut digits = iter::from_fn(|| {
                if x == 0 {
                    None
                } else {
                    let digit = x % 10;
                    x /= 10;
                    Some(digit)
                }
            });
            let mut first_half = digits.by_ref().take(n / 2).collect::<Vec<_>>();
            first_half.reverse();
            if n % 2 == 1 {
                digits.next();
            }
            first_half.iter().all(|d| Some(*d) == digits.next())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
