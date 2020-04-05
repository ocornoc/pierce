use std::fmt;

pub type Index = u8;

#[derive(Clone)]
pub enum Term {
    Var(Index),
    Abs(Box<Term>),
    App(Box<Term>, Box<Term>),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(index) => write!(f, "{}", *index),
            Term::Abs(body) => write!(f, "(λ. {})", body),
            Term::App(t1, t2) => write!(f, "({} {})", t1, t2),
        }
    }
}

impl Term {
    fn shift(&mut self, up: bool, cutoff: Index) {
        match self {
            Term::Var(index) => {
                if *index >= cutoff {
                    if up {
                        *index += 1;
                    } else {
                        *index -= 1;
                    }
                }
            }
            Term::Abs(body) => {
                body.shift(up, cutoff + 1);
            }
            Term::App(t1, t2) => {
                t1.shift(up, cutoff);
                t2.shift(up, cutoff);
            }
        }
    }

    fn replace(&mut self, index: Index, subs: &mut Term) {
        match self {
            Term::Var(index2) => {
                if index == *index2 {
                    *self = subs.clone();
                }
            }
            Term::Abs(body) => {
                subs.shift(true, 0);
                body.replace(index + 1, subs);
            }
            Term::App(t1, t2) => {
                t1.replace(index, subs);
                t2.replace(index, subs);
            }
        }
    }

    pub fn reduce(&mut self) -> bool {
        match self {
            Term::App(t1, t2) => {
                t1.reduce()
                    || t2.reduce()
                    || match &mut **t1 {
                        Term::Abs(body) => {
                            t2.shift(true, 0);
                            body.replace(0, t2);
                            body.shift(false, 0);
                            *self = *body.clone();
                            true
                        }
                        _ => false,
                    }
            }
            Term::Abs(term) => term.reduce(),
            _ => false,
        }
    }
}
