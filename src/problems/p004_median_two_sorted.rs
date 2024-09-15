use crate::solution::Solution;

// Body starts here
use std::cmp::min;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total = nums1.len() + nums2.len();
        let half = total / 2;
        let is_even = total % 2 == 0;
        let stop = if is_even { half - 1 } else { half };
        let mut total = 0;
        let mut nums1 = &nums1[..];
        let mut nums2 = &nums2[..];

        while let Some(l) = nums1.first() {
            if let Some(r) = nums2.first() {
                if total == stop {
                    if is_even {
                        return (*l + *r) as f64 / 2.0;
                    } else {
                        return min(*l, *r) as f64;
                    }
                } else {
                    total += 1;
                    if l < r {
                        nums1 = &nums1[1..];
                    } else {
                        nums2 = &nums2[1..];
                    }
                }
            } else {
                break;
            }
        }
        let rest = if nums1.is_empty() { nums2 } else { nums1 };
        let half = half - total;
        if is_even {
            (rest[half - 1] + rest[half]) as f64 / 2.0
        } else {
            rest[half] as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.);
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![-1, 3]),
            1.5
        );
    }
}
