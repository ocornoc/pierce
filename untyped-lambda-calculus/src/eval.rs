use std::fmt;

#[derive(Clone)]
pub enum Term {
    Var(u8),
    Abs(u8, Box<Term>),
    App(Box<Term>, Box<Term>),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(var) => write!(f, "{}", *var as char),
            Term::Abs(var, term) => write!(f, "(λ{}. {})", *var as char, term),
            Term::App(t1, t2) => write!(f, "({} {})", t1, t2),
        }
    }
}

impl Term {
    fn is_free(&self, var: u8) -> bool {
        match self {
            Term::Var(var2) => var == *var2,
            Term::Abs(arg, body) => (var != *arg) && body.is_free(var),
            Term::App(t1, t2) => t1.is_free(var) || t2.is_free(var),
        }
    }

    fn replace(&mut self, var: u8, subs: &Term) -> bool {
        match self {
            Term::Var(var2) => {
                if var == *var2 {
                    *self = subs.clone();
                }
                true
            }
            Term::Abs(arg, body) => {
                if var == *arg {
                    true
                } else if subs.is_free(*arg) {
                    false
                } else {
                    body.replace(var, subs)
                }
            }
            Term::App(t1, t2) => t1.replace(var, subs) && t2.replace(var, subs),
        }
    }

    pub fn reduce(&mut self) {
        match self {
            Term::App(t1, t2) => match &mut **t1 {
                Term::Abs(arg, body) => {
                    if body.replace(*arg, t2) {
                        *self = *body.clone();
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}
