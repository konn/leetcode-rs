use crate::solution::Solution;

// Body starts here
use std::collections::BTreeMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut dic: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
        for (i, &m) in nums.iter().enumerate() {
            dic.entry(m).or_default().push(i);
        }
        for (i, &m) in nums.iter().enumerate() {
            let j = dic
                .get(&(target - m))
                .and_then(|is| is.iter().find(|j| **j != i));
            match j {
                None => continue,
                Some(j) => return vec![i as i32, *j as i32],
            }
        }
        panic!("Invalid question")
    }
}
