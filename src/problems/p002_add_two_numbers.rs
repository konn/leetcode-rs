use crate::solution::Solution;
// Body starts here

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn unfold_right<A>(step: impl Fn(A) -> Option<(i32, A)>, a: A) -> Option<Box<ListNode>> {
    match step(a) {
        None => None,
        Some((val, a)) => {
            let mut a = a;
            let mut answer = Box::new(ListNode::new(val));
            let mut tail = &mut answer.next;
            while let Some((val, a_next)) = step(a) {
                a = a_next;
                let new_node = Box::new(ListNode::new(val));
                tail = &mut tail.insert(new_node).next;
            }
            Some(answer)
        }
    }
}

// Can we implement this without recursion? (is Endo bad?)
fn fold_right<A>(step: fn(i32, A) -> A, init: A, list: Option<Box<ListNode>>) -> A {
    match list {
        None => init,
        Some(node) => step(node.val, fold_right(step, init, node.next)),
    }
}

fn from_reverse_digits(rev_digs: Option<Box<ListNode>>) -> i64 {
    fold_right(|i, accum| accum * 10 + (i as i64), 0, rev_digs)
}

fn to_reverse_digits(i: i64) -> Option<Box<ListNode>> {
    if i == 0 {
        Some(Box::new(ListNode::new(0)))
    } else {
        unfold_right(
            |i| {
                if i == 0 {
                    None
                } else {
                    Some(((i % 10) as i32, i / 10))
                }
            },
            i,
        )
    }
}

fn align_with<'a, F>(mut f: F, l1: Box<ListNode>, l2: Box<ListNode>) -> Box<ListNode>
where
    F: FnMut(Option<i32>, Option<i32>) -> i32,
{
    let v = f(Some(l1.val), Some(l2.val));
    let mut answer = Box::new(ListNode::new(v));
    let mut tail = &mut answer.next;
    let mut l1 = l1.next;
    let mut l2 = l2.next;
    while l1.is_some() || l2.is_some() {
        let (l, l_next) = l1.map(|v| (v.val, v.next)).unzip();
        l1 = l_next.flatten();
        let (r, r_next) = l2.map(|v| (v.val, v.next)).unzip();
        l2 = r_next.flatten();
        let v = f(l, r);
        let new_node = Box::new(ListNode::new(v));
        tail = &mut tail.insert(new_node).next;
    }
    answer
}

fn push(list: &mut Box<ListNode>, val: i32) {
    let mut tail = &mut list.next;
    while tail.is_some() {
        tail = &mut tail.as_mut().unwrap().next;
    }
    *tail = Some(Box::new(ListNode::new(val)));
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut ds = align_with(
            |l, r| {
                let l = l.unwrap_or(0);
                let r = r.unwrap_or(0);
                let sum = l + r + carry;
                carry = sum / 10;
                sum % 10
            },
            l1.expect("lhs must be non-empty!"),
            l2.expect("rhs must be non-empty!"),
        );
        if carry > 0 {
            push(&mut ds, 1)
        }
        Some(ds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_reverse_digits() {
        assert_eq!(
            to_reverse_digits(342),
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 3, next: None }))
                }))
            }))
        );
    }

    #[test]
    fn test_from_reverse_digits() {
        assert_eq!(
            from_reverse_digits(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(3)))
                }))
            }))),
            321
        );
    }

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(
            from_reverse_digits(Solution::add_two_numbers(
                to_reverse_digits(12),
                to_reverse_digits(23)
            )),
            35
        );
        assert_eq!(
            from_reverse_digits(Solution::add_two_numbers(
                to_reverse_digits(9),
                to_reverse_digits(9999999991)
            )),
            10000000000
        );

        assert_eq!(from_reverse_digits(to_reverse_digits(342)), 342);
        assert_eq!(from_reverse_digits(to_reverse_digits(465)), 465);
        assert_eq!(
            from_reverse_digits(Solution::add_two_numbers(
                to_reverse_digits(342),
                to_reverse_digits(465)
            )),
            807
        );
        assert_eq!(
            from_reverse_digits(Solution::add_two_numbers(
                to_reverse_digits(0),
                to_reverse_digits(0)
            )),
            0
        );
    }
}
