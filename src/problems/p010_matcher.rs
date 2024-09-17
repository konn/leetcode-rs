use crate::solution::Solution;

// Body starts here
#[derive(Clone, Eq, PartialEq, Debug)]
enum Atom {
    Char(char),
    Wild,
}

impl Atom {
    pub fn derive(self, c: char) -> Pat {
        match self {
            Atom::Char(d) => {
                if c == d {
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
    Alt(Box<Pat>, Box<Pat>),
    Seq(Box<Pat>, Box<Pat>),
}

impl Pat {
    pub fn alt(l: Pat, r: Pat) -> Self {
        match (l, r) {
            (Pat::Fail, r) => r,
            (l, Pat::Fail) => l,
            (l, r) => Pat::Alt(Box::new(l), Box::new(r)),
        }
    }

    pub fn seq(l: Pat, r: Pat) -> Pat {
        match (l, r) {
            (Pat::Fail, _) | (_, Pat::Fail) => Pat::Fail,
            (Pat::Empty, r) => r,
            (l, Pat::Empty) => l,
            (l, r) => Pat::Seq(Box::new(l), Box::new(r)),
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
        let mut prev: Option<Atom> = None;
        let mut pat = Pat::Empty;
        for c in s.chars() {
            match c {
                '*' => {
                    pat = Self::seq(
                        pat,
                        Pat::Repeat(prev.take().expect("* not followed by atom!")),
                    );
                }
                _ => {
                    let atom = match c {
                        '.' => Atom::Wild,
                        _ => Atom::Char(c),
                    };
                    if let Some(e) = prev.replace(atom) {
                        pat = Pat::seq(pat, Pat::Atom(e))
                    }
                }
            }
        }
        if let Some(e) = prev {
            pat = Pat::seq(pat, Pat::Atom(e))
        }
        pat
    }

    pub fn matches(self, s: String) -> bool {
        let pat = s.chars().fold(self, Self::derive);
        pat.matches_empty()
    }

    // Brzozowski derivative
    pub fn derive(self, c: char) -> Self {
        match self {
            Pat::Seq(l, r) => {
                if l.matches_empty() {
                    let dl_r = Pat::seq(l.derive(c), *r.clone());
                    let dr = r.derive(c);
                    Pat::alt(dl_r, dr)
                } else {
                    Pat::seq(l.derive(c), *r)
                }
            }
            Pat::Fail => Pat::Fail,
            Pat::Empty => Pat::Fail,
            Pat::Alt(l, r) => Pat::alt(l.derive(c), r.derive(c)),
            Pat::Atom(a) => a.derive(c),
            Pat::Repeat(a) => {
                let b = a.clone();
                let da = a.clone().derive(c);
                Pat::seq(da, Pat::Repeat(b))
            }
        }
    }
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Pat::parse(p).matches(s)
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
