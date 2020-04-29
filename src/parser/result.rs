use super::tokenizer::TokenKind;
use std::fmt;

pub type ParseResult<T> = Result<T, ParseError>;

pub struct ParseError {
    pub loc: usize,
    pub kind: ParseErrorKind,
}

impl ParseError {
    pub fn new(loc: usize, kind: ParseErrorKind) -> Self {
        ParseError { loc, kind }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parsing error at location {}", self.loc)
    }
}

pub enum ParseErrorKind {
    InvalidChar(char),
    UnexpectedToken(TokenKind),
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ParseErrorKind::*;
        match self {
            &InvalidChar(c) => write!(f, "Invalid char {}", c),
            UnexpectedToken(token) => write!(f, "Unexpected token '{:?}'", token),
        }
    }
}
