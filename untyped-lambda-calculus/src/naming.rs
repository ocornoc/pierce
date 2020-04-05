use crate::eval::{Index, Term};
use crate::parser::NamedTerm;

pub fn remove_names(term: NamedTerm) -> Term {
     Context::default().remove_names(term)
}

pub fn add_names(term: Term) -> NamedTerm {
     Context::default().add_names(term)
}

#[derive(Default)]
struct Context {
    inner: Vec<u8>,
}

impl Context {
    fn add_names(&mut self, term: Term) -> NamedTerm {
        let len =  self.inner.len() as u8;
        match term {
            Term::Var(index) => {
                let name = self.inner.get((len - index - 1) as usize).unwrap();

                NamedTerm::Var(*name)
            }
            Term::Abs(body) => {
                let arg = b'a' + len;
                self.inner.push(arg);
                let body = self.add_names(*body);
                self.inner.pop().unwrap();
                NamedTerm::Abs(arg, Box::new(body))
            }
            Term::App(t1, t2) => {
                let t1 = self.add_names(*t1);
                let t2 = self.add_names(*t2);
                NamedTerm::App(Box::new(t1), Box::new(t2))
            }
        }
    }

    fn remove_names(&mut self, term: NamedTerm) -> Term {
        match term {
            NamedTerm::Var(name) => {
                let index = self
                    .inner
                    .iter()
                    .rev()
                    .enumerate()
                    .find(|(_, name2)| name == **name2)
                    .map(|(index, _)| index)
                    .unwrap() as Index;

                Term::Var(index)
            }
            NamedTerm::Abs(arg, body) => {
                self.inner.push(arg);
                let body = self.remove_names(*body);
                self.inner.pop().unwrap();
                Term::Abs(Box::new(body))
            }
            NamedTerm::App(t1, t2) => {
                let t1 = self.remove_names(*t1);
                let t2 = self.remove_names(*t2);
                Term::App(Box::new(t1), Box::new(t2))
            }
        }
    }
}
