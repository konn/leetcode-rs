use crate::solution::Solution;

// Body starts here

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut iter = s.chars().skip_while(|v| v.is_whitespace()).peekable();
        let (sign, bound) = match iter.peek() {
            Some('+') => {
                iter.next();
                (1 as i32, i32::MAX / 10)
            }
            Some('-') => {
                iter.next();
                (-1 as i32, (i32::MIN / 10).abs())
            }
            _ => (1, i32::MAX),
        };
        let parsed = iter
            .skip_while(|v| *v == '0')
            .map_while(|c| c.to_digit(10))
            .take(11) // i32::MAX is 2147483647, so we only need first 11 digits
            .try_fold(0, |acc, i| {
                let i = i as i32;
                if acc > bound - i {
                    None
                } else {
                    Some(acc * 10 + i)
                }
            });

        if let Some(d) = parsed {
            sign * d
        } else if sign == 1 {
            return i32::MAX;
        } else {
            return i32::MIN;
        }
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
    }
}
