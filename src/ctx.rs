use std::fmt;

use crate::eval::{Index, Term};
use crate::parser::{Name, NamedTerm};
use crate::ty::{Binding, Ty};

pub fn desugar(term: NamedTerm) -> Option<(Term, Ty)> {
    match Context::default().desugar(term) {
        Ok(term) => Some(term),
        Err(error) => {
            eprintln!("{}", error);
            None
        }
    }
}

pub fn restore(term: Term) -> Option<NamedTerm> {
    match Context::default().restore(term) {
        Ok(term) => Some(term),
        Err(error) => {
            eprintln!("{}", error);
            None
        }
    }
}

type CtxResult<T> = Result<T, CtxError>;

enum CtxError {
    MissingBinding(Name),
    UnexpectedType(Term, Ty),
}

impl fmt::Display for CtxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CtxError::*;
        match self {
            MissingBinding(name) => write!(f, "Context error: Variable {:?} is not bound.", name),
            UnexpectedType(term, ty) => write!(
                f,
                "Context error: Unexpected type {} for term {}.",
                ty, term
            ),
        }
    }
}
#[derive(Default)]
struct Context {
    inner: Vec<Binding>,
}

impl Context {
    fn restore(&mut self, term: Term) -> CtxResult<NamedTerm> {
        match term {
            Term::Unit => Ok(NamedTerm::Unit),
            Term::Var(index) => {
                let bind = self
                    .inner
                    .get(self.inner.len() - index as usize - 1)
                    .unwrap();

                Ok(NamedTerm::Var(bind.name))
            }
            Term::Abs(bind, body) => {
                self.inner.push(bind);
                let body = self.restore(*body)?;
                let bind = self.inner.pop().unwrap();
                Ok(NamedTerm::Abs(bind, Box::new(body)))
            }
            Term::App(t1, t2) => {
                let t1 = self.restore(*t1)?;
                let t2 = self.restore(*t2)?;
                Ok(NamedTerm::App(Box::new(t1), Box::new(t2)))
            }
        }
    }

    fn desugar(&mut self, term: NamedTerm) -> CtxResult<(Term, Ty)> {
        match term {
            NamedTerm::Unit => Ok((Term::Unit, Ty::Unit)),
            NamedTerm::Var(name) => {
                let (index, bind) = self
                    .inner
                    .iter()
                    .rev()
                    .enumerate()
                    .find(|(_, bind)| name == bind.name)
                    .ok_or_else(|| CtxError::MissingBinding(name))?;
                Ok((Term::Var(index as Index), bind.ty.clone()))
            }
            NamedTerm::Abs(bind, body) => {
                self.inner.push(bind);
                let (body, body_ty) = self.desugar(*body)?;
                let bind = self.inner.pop().unwrap();
                Ok((
                    Term::Abs(bind.clone(), Box::new(body)),
                    Ty::Arrow(Box::new(bind.ty), Box::new(body_ty)),
                ))
            }
            NamedTerm::App(t1, t2) => {
                let (t1, ty1) = self.desugar(*t1)?;
                let (t2, ty2) = self.desugar(*t2)?;
                match ty1 {
                    Ty::Arrow(ty11, ty) if *ty11 == ty2 => {
                        Ok((Term::App(Box::new(t1), Box::new(t2)), *ty))
                    }
                    _ => Err(CtxError::UnexpectedType(t2, ty2)),
                }
            }
            NamedTerm::Let(name, t1, t2) => {
                let (t1, ty1) = self.desugar(*t1)?;
                self.inner.push(Binding { name, ty: ty1 });
                let (t2, ty2) = self.desugar(*t2)?;
                let bind = self.inner.pop().unwrap();
                Ok((
                    Term::App(Box::new(Term::Abs(bind, Box::new(t2))), Box::new(t1)),
                    ty2,
                ))
            }
        }
    }
}
