use std::fmt;

use crate::eval::{Index, Term};
use crate::parser::{Name, NamedTerm};

pub fn remove_names(term: NamedTerm) -> Option<Term> {
     match Context::default().remove_names(term) {
        Ok(term) => Some(term),
        Err(error) => {
            eprintln!("{}", error);
            None
        }
     }
}

pub fn restore_names(term: Term) -> Option<NamedTerm> {
    match Context::default().restore_names(term) {
        Ok(term) => Some(term),
        Err(error) => {
            eprintln!("{}", error);
            None
        }
    }
}

type NamingResult<T> = Result<T, NamingError>;

enum NamingError {
    MissingIndex(Name),
    MissingName(Index),
}

impl fmt::Display for NamingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use NamingError::*;
        match *self {
            MissingIndex(name) => write!(f, "Error during name removal: Variable {:?} is not binded by any abstraction.", name as char),
            MissingName(index) => write!(f, "Error during name restoring: Missign variable name for index {}.", index),
        }
    }
}

#[derive(Default)]
struct Context {
    inner: Vec<Name>,
}

impl Context {
    fn restore_names(&mut self, term: Term) -> NamingResult<NamedTerm> {
        match term {
            Term::Var(index) => {
                let name = self.inner.get(self.inner.len() - index as usize - 1).ok_or_else(|| NamingError::MissingName(index))?;
                Ok(NamedTerm::Var(*name))
            }
            Term::Abs(arg, body) => {
                self.inner.push(arg);
                let body = self.restore_names(*body)?;
                self.inner.pop().unwrap();
                Ok(NamedTerm::Abs(arg, Box::new(body)))
            }
            Term::App(t1, t2) => {
                let t1 = self.restore_names(*t1)?;
                let t2 = self.restore_names(*t2)?;
                Ok(NamedTerm::App(Box::new(t1), Box::new(t2)))
            }
        }
    }

    fn remove_names(&mut self, term: NamedTerm) -> NamingResult<Term> {
        match term {
            NamedTerm::Var(name) => {
                let index = self
                    .inner
                    .iter()
                    .enumerate()
                    .find(|(_, name2)| name == **name2)
                    .map(|(index, _)| self.inner.len() - index - 1)
                    .ok_or_else(|| NamingError::MissingIndex(name))? as Index;

                Ok(Term::Var(index))
            }
            NamedTerm::Abs(arg, body) => {
                self.inner.push(arg);
                let body = self.remove_names(*body)?;
                self.inner.pop().unwrap();
                Ok(Term::Abs(arg, Box::new(body)))
            }
            NamedTerm::App(t1, t2) => {
                let t1 = self.remove_names(*t1)?;
                let t2 = self.remove_names(*t2)?;
                Ok(Term::App(Box::new(t1), Box::new(t2)))
            }
        }
    }
}
