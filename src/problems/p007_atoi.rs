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
        let mut digits = iter
            .skip_while(|v| *v == '0')
            .map_while(|c| c.to_digit(10))
            // Without next two lines, !!! will match even if there are non-digit character other then digit!
            .collect::<Vec<_>>()
            .into_iter();
        // If there are more than 10 digits, it clearly exceeds the 32-bit integer.
        // So we only need to parse only first ten digits and return bounds if there are remaining digits.
        let parsed = digits
            .by_ref()
            .take(10)
            .fold(0, |acc, i| acc * 10 + i as i64);

        if let Some(_) = digits.next() {
            // !!! Should parse there are more consecutive digits.
            if sign == 1 {
                return i32::MAX;
            } else {
                return i32::MIN;
            }
        }

        // Even if it has <=10 digits, it can still exceed the bounds.
        // so we must check the bound.
        let ub = i32::MAX as i64;
        let lb = i32::MIN as i64;
        let value = sign * parsed;
        if value < lb {
            i32::MIN
        } else if value > ub {
            i32::MAX
        } else {
            value as i32
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
