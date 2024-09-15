use crate::solution::Solution;

// Body starts here

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut iter = s.chars().skip_while(|v| v.is_whitespace()).peekable();
        let sign: i64 = match iter.peek() {
            Some('+') => {
                iter.next();
                1
            }
            Some('-') => {
                iter.next();
                -1
            }
            _ => 1,
        };

        let digits = iter
            .skip_while(|v| *v == '0')
            .map_while(|c| c.to_digit(10))
            .enumerate()
            .try_fold(0, |acc, (idx, i)| {
                if idx >= 10 {
                    None
                } else {
                    Some(acc * 10 + i as i64)
                }
            });
        match digits {
            None => {
                if sign == 1 {
                    i32::MAX
                } else {
                    i32::MIN
                }
            }
            Some(i) => {
                let ub = i32::MAX as i64;
                let lb = i32::MIN as i64;
                let value = sign * i;
                if value < lb {
                    i32::MIN
                } else if value > ub {
                    i32::MAX
                } else {
                    value as i32
                }
            }
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
