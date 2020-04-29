use std::fmt;

use crate::ty::Binding;

pub type Index = u8;

#[derive(Clone)]
pub enum Term {
    Unit,
    Var(Index),
    Lam(Binding, Box<Term>),
    Mu(Binding, Box<Term>),
    App(Box<Term>, Box<Term>),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Unit => write!(f, "unit"),
            Term::Var(index) => write!(f, "{}", *index),
            Term::Lam(_, body) => write!(f, "(λ. {})", body),
            Term::Mu(_, body) => write!(f, "(μ. {})", body),
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
            Term::Lam(_, body) | Term::Mu(_, body) => {
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
            Term::Lam(_, body) | Term::Mu(_, body) => {
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

    fn structural_reduction(&mut self, index: Index, arg: &mut Term) {
        match self {
            Term::Unit | Term::Var(_) => (),
            Term::Lam(_, body) | Term::Mu(_, body) => {
                arg.shift(true, 0);
                body.structural_reduction(index + 1, arg);
                arg.shift(false, 0);
            }
            Term::App(t1, t2) => {
                match t1.as_ref() {
                    Term::Var(i) if *i == index => {
                        **t2 = Term::App(t2.clone(), Box::new(arg.clone()))
                    }
                    _ => {
                        t1.structural_reduction(index, arg);
                        t2.structural_reduction(index, arg);
                    }
                }
            }
        }
    }

    fn reduce(&mut self) -> bool {
        match self {
            Term::App(t1, t2) => match t1.as_mut() {
                Term::Lam(_, body) => {
                    t2.shift(true, 0);
                    body.replace(0, t2);
                    body.shift(false, 0);
                    *self = *body.clone();
                    true
                }
                Term::Mu(_, body) => {
                    t2.shift(true, 0);
                    body.structural_reduction(0, t2);
                    *self = *body.clone();
                    true
                }
                _ => t1.reduce() || t2.reduce(),
            },
            Term::Lam(_, term) | Term::Mu(_, term) => term.reduce(),
            _ => false,
        }
    }

    pub fn evaluate(&mut self) {
        while self.reduce() {}
    }
}
