pub mod result;
pub mod tokenizer;

use std::fmt;

use crate::ty::{Binding, Ty};

use self::result::*;
use self::tokenizer::*;

pub type Name = char;

#[derive(Clone)]
pub enum NamedTerm {
    Unit,
    Var(Name),
    Lam(Binding, Box<NamedTerm>),
    App(Box<NamedTerm>, Box<NamedTerm>),
    Let(Name, Box<NamedTerm>, Box<NamedTerm>),
}

impl fmt::Display for NamedTerm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NamedTerm::Unit => write!(f, "unit"),
            NamedTerm::Var(var) => write!(f, "{}", var),
            NamedTerm::Lam(Binding { name, ty }, term) => {
                write!(f, "(λ{}:{}. {})", name, ty, term)
            }
            NamedTerm::App(t1, t2) => write!(f, "({} {})", t1, t2),
            NamedTerm::Let(name, t1, t2) => write!(f, "(let {} = {} in {})", name, t1, t2),
        }
    }
}

pub fn parse(input: String) -> Option<NamedTerm> {
    let tokens = Tokenizer::new(input.clone());
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

struct Parser {
    lookahead: Token,
    tokens: Tokenizer,
}

impl Parser {
    fn run(mut tokens: Tokenizer) -> ParseResult<NamedTerm> {
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

    fn expect_word(&mut self, expected_word: &str) -> ParseResult<()> {
        let token = self.consume_lookahead()?;
        match token.kind() {
            TokenKind::Word(word) if word == expected_word => Ok(()),
            _ => Err(token.into_unexpected())
        }
    }

    fn parse_name(&mut self) -> ParseResult<Name> {
        let token = self.consume_lookahead()?;
        if let TokenKind::AlphaChar(byte) = token.kind() {
            Ok(*byte)
        } else {
            Err(token.into_unexpected())
        }
    }

    fn parse_term(&mut self) -> ParseResult<NamedTerm> {
        let token = self.consume_lookahead()?;
        match token.kind() {
            TokenKind::Word(chars) if chars == "unit" => Ok(NamedTerm::Unit),
            TokenKind::AlphaChar(c) => Ok(NamedTerm::Var(*c)),
            TokenKind::LBracket => {
                let term = match self.lookahead.kind() {
                    TokenKind::Lambda => self.parse_lam()?,
                    TokenKind::Word(chars) if chars == "let" => self.parse_let()?,
                    _ => self.parse_app()?,
                };
                self.expect_token(TokenKind::RBracket)?;
                Ok(term)
            }
            _ => Err(token.into_unexpected()),
        }
    }

    fn parse_lam(&mut self) -> ParseResult<NamedTerm> {
        self.expect_token(TokenKind::Lambda)?;

        let name = self.parse_name()?;
        self.expect_token(TokenKind::Colon)?;
        let ty = self.parse_ty()?;
        self.expect_token(TokenKind::Dot)?;
        self.expect_token(TokenKind::Space)?;
        let body = self.parse_term()?;

        Ok(NamedTerm::Lam(Binding { name, ty }, Box::new(body)))
    }

    fn parse_app(&mut self) -> ParseResult<NamedTerm> {
        let t1 = self.parse_term()?;
        self.expect_token(TokenKind::Space)?;
        let t2 = self.parse_term()?;

        Ok(NamedTerm::App(Box::new(t1), Box::new(t2)))
    }

    fn parse_let(&mut self) -> ParseResult<NamedTerm> {
        self.expect_word("let")?;

        self.expect_token(TokenKind::Space)?;
        let name = self.parse_name()?;
        self.expect_token(TokenKind::Space)?;
        self.expect_token(TokenKind::Equal)?;
        self.expect_token(TokenKind::Space)?;
        let t1 = self.parse_term()?;
        self.expect_token(TokenKind::Space)?;
        self.expect_word("in")?;
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
            TokenKind::Word(chars) if chars == "Unit" => Ok(Ty::Unit),
            _ => Err(token.into_unexpected()),
        }
    }
}
