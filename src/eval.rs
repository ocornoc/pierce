use std::fmt;

use crate::ty::Binding;

pub type Index = u8;

#[derive(Clone)]
pub enum Term {
    Unit,
    Var(Index),
    Abs(Binding, Box<Term>),
    App(Box<Term>, Box<Term>),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Unit => write!(f, "unit"),
            Term::Var(index) => write!(f, "{}", *index),
            Term::Abs(_, body) => write!(f, "(λ. {})", body),
            Term::App(t1, t2) => write!(f, "({} {})", t1, t2),
        }
    }
}

impl Term {
    fn shift(&mut self, up: bool, cutoff: Index) {
        match self {
            Term::Unit => (),
            Term::Var(index) => {
                if *index >= cutoff {
                    if up {
                        *index += 1;
                    } else {
                        *index -= 1;
                    }
                }
            }
            Term::Abs(_, body) => {
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
            Term::Unit => (),
            Term::Var(index2) => {
                if index == *index2 {
                    *self = subs.clone();
                }
            }
            Term::Abs(_, body) => {
                subs.shift(true, 0);
                body.replace(index + 1, subs);
                subs.shift(false, 0);
            }
            Term::App(t1, t2) => {
                t1.replace(index, subs);
                t2.replace(index, subs);
            }
        }
    }

    fn reduce(&mut self) -> bool {
        match self {
            Term::App(t1, t2) => match t1.as_mut() {
                Term::Abs(_, body) => {
                    t2.shift(true, 0);
                    body.replace(0, t2);
                    body.shift(false, 0);
                    *self = *body.clone();
                    true
                }
                _ => t1.reduce() || t2.reduce(),
            },
            Term::Abs(_, term) => term.reduce(),
            _ => false,
        }
    }

    pub fn evaluate(&mut self) {
        while self.reduce() {}
    }
}
