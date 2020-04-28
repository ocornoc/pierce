pub mod result;
pub mod tokenizer;

use std::fmt;

use crate::ty::{Binding, Ty};

use self::result::*;
use self::tokenizer::*;

pub type Name = u8;

#[derive(Clone)]
pub enum NamedTerm {
    Unit,
    Var(Name),
    Abs(Binding, Box<NamedTerm>),
    App(Box<NamedTerm>, Box<NamedTerm>),
    Let(Name, Box<NamedTerm>, Box<NamedTerm>),
}

impl fmt::Display for NamedTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NamedTerm::Unit => write!(f, "unit"),
            NamedTerm::Var(var) => write!(f, "{}", *var as char),
            NamedTerm::Abs(Binding { name, ty }, term) => {
                write!(f, "(λ{}:{}. {})", *name as char, ty, term)
            }
            NamedTerm::App(t1, t2) => write!(f, "({} {})", t1, t2),
            NamedTerm::Let(name, t1, t2) => write!(f, "(let {} = {} in {})", *name as char, t1, t2),
        }
    }
}

pub fn parse(input: &str) -> Option<NamedTerm> {
    let tokens = Tokenizer::new(input.as_bytes());
    match Parser::run(tokens) {
        Ok(term) => Some(term),
        Err(error) => {
            eprintln!("{}", error);
            eprintln!("{}", input);
            eprintln!("{:offset$}{}", " ", "^", offset = error.loc);
            eprintln!("{:offset$}{}", " ", error.kind, offset = error.loc);
            None
        }
    }
}

struct Parser<'a> {
    lookahead: Token,
    tokens: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    fn run(mut tokens: Tokenizer<'a>) -> ParseResult<NamedTerm> {
        let mut parser = Parser {
            lookahead: tokens.next_token()?,
            tokens,
        };
        let term = parser.parse_term()?;
        parser.expect_token(TokenKind::EOF)?;
        Ok(term)
    }

    fn consume_lookahead(&mut self) -> ParseResult<Token> {
        let mut token = self.tokens.next_token()?;
        std::mem::swap(&mut token, &mut self.lookahead);
        Ok(token)
    }

    fn expect_token(&mut self, expected: TokenKind) -> ParseResult<()> {
        let token = self.consume_lookahead()?;
        if expected == *token.kind() {
            Ok(())
        } else {
            Err(token.into_unexpected())
        }
    }

    fn parse_term(&mut self) -> ParseResult<NamedTerm> {
        let token = self.consume_lookahead()?;
        match token.kind() {
            TokenKind::Word(bytes) if bytes == b"unit" => Ok(NamedTerm::Unit),
            TokenKind::AlphaChar(byte) => Ok(NamedTerm::Var(*byte)),
            TokenKind::LBracket => {
                let term = match self.lookahead.kind() {
                    TokenKind::Lambda => self.parse_abs()?,
                    TokenKind::Word(bytes) if bytes == b"let" => self.parse_let()?,
                    _ => self.parse_app()?,
                };
                self.expect_token(TokenKind::RBracket)?;
                Ok(term)
            }
            _ => Err(token.into_unexpected()),
        }
    }

    fn parse_abs(&mut self) -> ParseResult<NamedTerm> {
        self.expect_token(TokenKind::Lambda)?;

        let token = self.consume_lookahead()?;
        let name = if let TokenKind::AlphaChar(byte) = token.kind() {
            *byte
        } else {
            return Err(token.into_unexpected());
        };
        self.expect_token(TokenKind::Colon)?;
        let ty = self.parse_ty()?;
        self.expect_token(TokenKind::Dot)?;
        self.expect_token(TokenKind::Space)?;
        let body = self.parse_term()?;

        Ok(NamedTerm::Abs(Binding { name, ty }, Box::new(body)))
    }

    fn parse_app(&mut self) -> ParseResult<NamedTerm> {
        let t1 = self.parse_term()?;
        self.expect_token(TokenKind::Space)?;
        let t2 = self.parse_term()?;
        Ok(NamedTerm::App(Box::new(t1), Box::new(t2)))
    }

    fn parse_let(&mut self) -> ParseResult<NamedTerm> {
        self.expect_token(TokenKind::Word(b"let".to_vec()))?;
        self.expect_token(TokenKind::Space)?;

        let token = self.consume_lookahead()?;
        let name = if let TokenKind::AlphaChar(byte) = token.kind() {
            *byte
        } else {
            return Err(token.into_unexpected());
        };

        self.expect_token(TokenKind::Space)?;
        self.expect_token(TokenKind::Equal)?;
        self.expect_token(TokenKind::Space)?;
        let t1 = self.parse_term()?;
        self.expect_token(TokenKind::Space)?;
        self.expect_token(TokenKind::Word(b"in".to_vec()))?;
        self.expect_token(TokenKind::Space)?;
        let t2 = self.parse_term()?;
        Ok(NamedTerm::Let(name, Box::new(t1), Box::new(t2)))
    }

    fn parse_ty(&mut self) -> ParseResult<Ty> {
        let token = self.consume_lookahead()?;
        match token.kind() {
            TokenKind::LBracket => {
                let t1 = self.parse_ty()?;
                self.expect_token(TokenKind::Space)?;
                self.expect_token(TokenKind::Arrow)?;
                self.expect_token(TokenKind::Space)?;
                let t2 = self.parse_ty()?;
                self.expect_token(TokenKind::RBracket)?;
                Ok(Ty::Arrow(Box::new(t1), Box::new(t2)))
            }
            TokenKind::Word(bytes) if bytes == b"Unit" => Ok(Ty::Unit),
            _ => Err(token.into_unexpected()),
        }
    }
}
