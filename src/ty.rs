use std::fmt;

use crate::parser::Name;

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
