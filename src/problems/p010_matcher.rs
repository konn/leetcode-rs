use std::{borrow::Borrow, rc::Rc};

use crate::solution::Solution;

// Body starts here
#[derive(Clone, Eq, PartialEq, Debug)]
enum Atom {
    Char(char),
    Wild,
}

impl Atom {
    pub fn derive(&self, c: char) -> Pat {
        match self {
            Atom::Char(d) => {
                if c == *d {
                    Pat::Empty
                } else {
                    Pat::Fail
                }
            }
            Atom::Wild => Pat::Empty,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Pat {
    Fail,
    Empty,
    Atom(Atom),
    Repeat(Atom),
    Alt(Rc<Pat>, Rc<Pat>),
    Seq(Rc<Pat>, Rc<Pat>),
}

impl Pat {
    pub fn alt(l: Rc<Pat>, r: Rc<Pat>) -> Rc<Self> {
        match (l.borrow(), r.borrow()) {
            (Pat::Fail, _) => r,
            (_, Pat::Fail) => l,
            _ => Rc::new(Pat::Alt(l, r)),
        }
    }

    pub fn seq(l: Rc<Pat>, r: Rc<Pat>) -> Rc<Pat> {
        match (l.borrow(), r.borrow()) {
            (Pat::Fail, _) | (_, Pat::Fail) => Rc::new(Pat::Fail),
            (Pat::Empty, _) => r,
            (_, Pat::Empty) => l,
            _ => Rc::new(Pat::Seq(l, r)),
        }
    }

    pub fn matches_empty(&self) -> bool {
        match self {
            Pat::Fail => false,
            Pat::Empty => true,
            Pat::Seq(l, r) => l.matches_empty() && r.matches_empty(),
            Pat::Atom(_) => false,
            Pat::Repeat(_) => true,
            Pat::Alt(l, r) => l.matches_empty() || r.matches_empty(),
        }
    }

    pub fn parse(s: String) -> Self {
        // We build a pattern in right-associative form,
        // which would give a better performance for Brzozowski derivative.
        // To do this, we reverse the string and build the pattern from right to left.
        let mut is_rep = false;
        s.chars()
            .rev()
            .flat_map(|c| match c {
                '*' => {
                    is_rep = true;
                    None
                }
                c => {
                    let p = match c {
                        '.' => Atom::Wild,
                        _ => Atom::Char(c),
                    };
                    if is_rep {
                        is_rep = false;
                        Some(Pat::Repeat(p))
                    } else {
                        Some(Pat::Atom(p))
                    }
                }
            })
            .fold(Pat::Empty, |acc, pat| {
                if let Pat::Empty = acc {
                    pat
                } else if let Pat::Empty = pat {
                    acc
                } else {
                    Pat::Seq(Rc::new(pat), Rc::new(acc))
                }
            })
    }

    pub fn matches(&self, s: &str) -> bool {
        let mut pat_raw;
        let mut pat = self;
        for c in s.chars() {
            pat_raw = pat.derive(c);
            pat = &pat_raw;
        }
        pat.matches_empty()
    }

    // Brzozowski derivative
    pub fn derive(&self, c: char) -> Rc<Self> {
        match self {
            Pat::Seq(l, r) => {
                if l.matches_empty() {
                    let dl_r = Pat::seq(l.derive(c), r.clone());
                    let dr = r.derive(c);
                    if let Pat::Fail = dr.borrow() {
                        dl_r
                    } else {
                        Pat::alt(dl_r, dr)
                    }
                } else {
                    Pat::seq(l.derive(c), r.clone())
                }
            }
            Pat::Fail => Rc::new(Pat::Fail),
            Pat::Empty => Rc::new(Pat::Fail),
            Pat::Alt(l, r) => Pat::alt(l.derive(c), r.derive(c)),
            Pat::Atom(a) => Rc::new(a.derive(c)),
            Pat::Repeat(a) => {
                let b = a.clone();
                let da = a.derive(c);
                Pat::seq(Rc::new(da), Rc::new(Pat::Repeat(b)))
            }
        }
    }
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Pat::parse(p).matches(&s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
        assert_eq!(
            Solution::is_match("abc".to_string(), "a.*c".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("abcd".to_string(), "a.*c".to_string()),
            false
        );
    }
}
