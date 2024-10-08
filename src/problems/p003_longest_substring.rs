use crate::solution::Solution;

// Body starts here
use std::cmp::max;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut len = 0;
        let mut last_pos_map: std::collections::HashMap<char, usize> =
            std::collections::HashMap::new();
        let mut start = 0;
        for (end, c) in s.chars().enumerate() {
            if let Some(last) = last_pos_map.insert(c, end) {
                if last >= start {
                    start = last + 1;
                }
            }
            len = max(len, end - start + 1);
        }
        len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        for (input, output) in vec![
            ("abcabcbb", 3),
            ("bbbbb", 1),
            ("pwwkew", 3),
            ("abba", 2),
            (" ", 1),
        ] {
            assert_eq!(
                Solution::length_of_longest_substring(input.to_string()),
                output,
                "input: {input}"
            );
        }
    }
}
