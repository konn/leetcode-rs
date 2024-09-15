use crate::solution::Solution;

// Body starts here
use std::iter;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = iter::once('#')
            .chain(
                s.chars()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join("#")
                    .chars(),
            )
            .chain(iter::once('#'))
            .collect::<Vec<_>>();

        dbg!(&s);
        // start_poss[i] records all the start positions of the palindrom ending at [i].
        // invariant: start_poss[i] is strictly decreasing.
        let mut start_poss = (0..s.len()).map(|v| vec![v]).collect::<Vec<_>>();
        let mut longest_slice = [0, 0];
        for (i, &c) in s.iter().enumerate().skip(2) {
            start_poss[i] = start_poss[i - 1]
                .iter()
                .filter_map(|&j| {
                    if j > 0 && s[j - 1] == c {
                        Some(j - 1)
                    } else {
                        None
                    }
                })
                .chain(iter::once(i))
                .collect();
            if let Some(&j) = start_poss[i].first() {
                let range = [j, i];
                if i - j > longest_slice[1] - longest_slice[0] {
                    longest_slice = range;
                }
            }
        }
        let [start, end] = longest_slice;
        s[start..=end].iter().filter(|v| **v != '#').collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrom() {
        assert_eq!(
            Solution::longest_palindrome("babad".to_string()),
            "bab".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
    }
}
