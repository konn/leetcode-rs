use crate::solution::Solution;

// Body starts here
use std::iter;

#[derive(Clone, Eq, PartialEq, Debug)]
enum Atom {
    Char(char),
    Wild,
}

impl Atom {
    pub fn derive(self, c: char) -> Pats {
        let pats = match self {
            Atom::Char(d) => {
                if c == d {
                    Some(vec![])
                } else {
                    None
                }
            }
            Atom::Wild => Some(vec![]),
        };
        Pats { pats }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Pat {
    Atom(Atom),
    Repeat(Atom),
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Pats {
    pats: Option<Vec<Pat>>,
}

impl Pat {
    pub fn matches_empty(self) -> bool {
        match self {
            Pat::Atom(_) => false,
            Pat::Repeat(_) => true,
        }
    }
}

impl Pats {
    pub fn parse(s: String) -> Self {
        let mut prev: Option<Atom> = None;
        let mut pats = Vec::new();
        for c in s.chars() {
            match c {
                '*' => {
                    pats.push(Pat::Repeat(prev.take().expect("* not followed by atom!")));
                }
                _ => {
                    let atom = match c {
                        '.' => Atom::Wild,
                        _ => Atom::Char(c),
                    };
                    if let Some(e) = prev.replace(atom) {
                        pats.push(Pat::Atom(e))
                    }
                }
            }
        }
        if let Some(e) = prev {
            pats.push(Pat::Atom(e))
        }
        Pats { pats: Some(pats) }
    }

    pub fn matches_empty(self) -> bool {
        match self.pats {
            Some(ps) => ps.into_iter().all(Pat::matches_empty),
            None => false,
        }
    }

    pub fn matches(self, s: String) -> bool {
        let pat = s.chars().fold(self, Self::derive);
        pat.matches_empty()
    }

    // Brzozowski derivative
    pub fn derive(self, c: char) -> Self {
        let pats = if let Some(ps) = self.pats {
            let mut ps = ps.into_iter();
            if let Some(e) = ps.next() {
                match e {
                    Pat::Atom(Atom::Wild) => Some(ps.collect::<Vec<_>>()),
                    Pat::Atom(Atom::Char(d)) => {
                        if c == d {
                            Some(ps.collect())
                        } else {
                            None
                        }
                    }
                    Pat::Repeat(a) => {
                        let b = a.clone();
                        a.derive(c).pats.map(|qs| {
                            qs.into_iter()
                                .chain(iter::once(Pat::Repeat(b)))
                                .chain(ps)
                                .collect()
                        })
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
        .map(|v| v.to_vec());
        Pats { pats }
    }
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Pats::parse(p).matches(s)
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
            Solution::is_match("abc".to_string(), "a.*".to_string()),
            true
        );
    }
}
