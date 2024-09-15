use crate::solution::Solution;

// Body starts here

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total = nums1.len() + nums2.len();
        let half = total / 2;
        let is_even = total % 2 == 0;
        let stop = if is_even { half - 1 } else { half };
        let mut count = 0;
        let mut pair: Option<i32> = None;
        let mut nums1 = &nums1[..];
        let mut nums2 = &nums2[..];

        while !nums1.is_empty() || !nums2.is_empty() {
            let l = nums1.first();
            let r = nums2.first();
            if let Some(x) = pair {
                if let Some(l) = l {
                    if let Some(r) = r {
                        if l < r {
                            return (x + l) as f64 / 2.;
                        } else {
                            return (x + r) as f64 / 2.;
                        }
                    } else {
                        return (x + l) as f64 / 2.;
                    }
                } else {
                    if let Some(r) = r {
                        return (x + r) as f64 / 2.;
                    } else {
                        // At least one of nums{1,2} non-empty.
                        unreachable!()
                    }
                }
            } else if count == stop {
                if is_even {
                    if let Some(&l) = l {
                        if let Some(&r) = r {
                            if l < r {
                                pair = Some(l);
                                nums1 = &nums1[1..];
                            } else {
                                pair = Some(r);
                                nums2 = &nums2[1..];
                            }
                        } else {
                            pair = Some(l);
                            nums1 = &nums1[1..];
                        }
                    } else {
                        pair = Some(*r.expect("r must be non-empty!"));
                        nums2 = &nums2[1..];
                    }
                } else {
                    if let Some(&l) = l {
                        if let Some(&r) = r {
                            if l < r {
                                return l as f64;
                            } else {
                                return r as f64;
                            }
                        } else {
                            return l as f64;
                        }
                    } else {
                        return *r.expect("r must be non-empty!") as f64;
                    }
                }
            } else {
                count += 1;
                if let Some(l) = l {
                    if let Some(r) = r {
                        if l < r {
                            nums1 = &nums1[1..];
                        } else {
                            nums2 = &nums2[1..];
                        }
                    } else {
                        nums1 = &nums1[1..];
                    }
                } else {
                    nums2 = &nums2[1..];
                }
            }
        }
        unreachable!()
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
