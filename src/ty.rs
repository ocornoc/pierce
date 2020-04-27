use std::fmt;

use crate::parser::{Name, NamedTerm};

pub fn type_of(term: &NamedTerm) -> Option<Ty> {
    match Context::default().type_of(term) {
        Ok(ty) => Some(ty),
        Err(error) => {
            eprintln!("{}", error);
            None
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Ty {
    Unit,
    Arrow(Box<Ty>, Box<Ty>),
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Unit => write!(f, "Unit"),
            Ty::Arrow(t1, t2) => write!(f, "({} -> {})", t1, t2),
        }
    }
}

#[derive(Clone)]
pub struct Binding {
    pub name: Name,
    pub ty: Ty,
}

type TypingResult<T> = Result<T, TypingError>;

enum TypingError {
    MissingBinding(Name),
    UnexpectedType(NamedTerm, Ty),
}

impl fmt::Display for TypingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TypingError::*;
        match self {
            MissingBinding(name) => write!(
                f,
                "Typing error: Variable {:?} does not have a type.",
                *name as char
            ),
            UnexpectedType(term, ty) => {
                write!(f, "Typing error: Unexpected type {} for term {}.", ty, term,)
            }
        }
    }
}

#[derive(Default)]
struct Context {
    inner: Vec<Binding>,
}

impl Context {
    fn type_of(&mut self, term: &NamedTerm) -> TypingResult<Ty> {
        match term {
            NamedTerm::Unit => Ok(Ty::Unit),
            NamedTerm::Var(name) => {
                let bind = self
                    .inner
                    .iter()
                    .find(|bind| *name == bind.name)
                    .ok_or_else(|| TypingError::MissingBinding(*name))?;
                Ok(bind.ty.clone())
            }
            NamedTerm::Abs(bind, body) => {
                self.inner.push(bind.clone());
                let body_ty = self.type_of(body)?;
                let bind = self.inner.pop().unwrap();
                Ok(Ty::Arrow(Box::new(bind.ty), Box::new(body_ty)))
            }
            NamedTerm::App(t1, t2) => {
                let t1_ty = self.type_of(t1)?;
                let t2_ty = self.type_of(t2)?;
                if let Ty::Arrow(t11, r) = t1_ty {
                    if *t11 == t2_ty {
                        Ok(*r)
                    } else {
                        Err(TypingError::UnexpectedType(t2.as_ref().clone(), t2_ty))
                    }
                } else {
                    Err(TypingError::UnexpectedType(t1.as_ref().clone(), t1_ty))
                }
            }
        }
    }
}
