use crate::solution::Solution;

// Body starts here
use std::iter;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == i32::MIN || x == i32::MAX || x == 0 {
            return 0;
        }
        let sign = x.signum();
        let mut x = x.abs();
        iter::from_fn(move || {
            if x == 0 {
                None
            } else {
                let digit = x % 10;
                x /= 10;
                Some(digit)
            }
        })
        .try_fold(0, |acc, i| {
            if Self::check_bound(sign, acc, i) {
                Some(acc * 10 + i)
            } else {
                None
            }
        })
        .map_or(0, |r| sign * r)
    }

    fn check_bound(sign: i32, acc: i32, i: i32) -> bool {
        if i < sign {
            acc >= i32::MIN / 10 && (i32::MIN + i) <= -acc * 10
        } else {
            acc <= i32::MAX / 10 && (i32::MAX - i) >= acc * 10
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(i32::MAX), 0);
        assert_eq!(Solution::reverse(i32::MIN), 0);
    }
}
