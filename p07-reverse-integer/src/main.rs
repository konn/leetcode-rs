use std::iter;
pub struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut acc = x.abs();
        let digits = iter::from_fn(move || {
            if acc == 0 {
                None
            } else {
                let digit = acc % 10;
                acc /= 10;
                Some(digit)
            }
        });
        x.signum() * digits.fold(0, |acc, x| acc * 10 + x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        for (input, output) in vec![(123, 321), (-123, -321), (120, 21)] {
            assert_eq!(Solution::reverse(input), output);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
