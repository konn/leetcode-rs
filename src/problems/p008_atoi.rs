use crate::solution::Solution;

// Body starts here

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut iter = s.chars().skip_while(|v| v.is_whitespace()).peekable();
        let (sign, bound) = match iter.peek() {
            Some('+') => {
                iter.next();
                (1, i32::MAX as i64)
            }
            Some('-') => {
                iter.next();
                (-1, (i32::MIN as i64).abs())
            }
            _ => (1, i32::MAX as i64),
        };
        let mut digits = iter
            .skip_while(|v| *v == '0')
            .map_while(|c| c.to_digit(10))
            .fuse();
        let parsed = digits
            .by_ref()
            .take(10) // i32::MAX is 2147483647, so we only need first 11 digits
            .try_fold(0, |acc, i| {
                let i = i as i64;
                if acc * 10 + i > bound {
                    None
                } else {
                    Some(acc * 10 + i)
                }
            });

        (if let Some(_) = digits.next() {
            sign * bound
        } else if let Some(d) = parsed {
            sign * d
        } else {
            sign * bound
        }) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atoi() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("   -042".to_string()), -42);
        assert_eq!(Solution::my_atoi("1337c0d3".to_string()), 1337);
        assert_eq!(Solution::my_atoi("0-1".to_string()), 0);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), i32::MIN);
        assert_eq!(Solution::my_atoi("21474836460".to_string()), i32::MAX);
        assert_eq!(Solution::my_atoi("-2147483647".to_string()), -2147483647);
    }
}
