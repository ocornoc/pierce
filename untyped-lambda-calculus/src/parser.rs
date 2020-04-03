pub mod result;
pub mod tokenizer;

use crate::eval::Term;

use self::result::*;
use self::tokenizer::*;

pub fn parse(input: &str) -> Option<Term> {
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
    fn run(mut tokens: Tokenizer<'a>) -> ParseResult<Term> {
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

    fn parse_term(&mut self) -> ParseResult<Term> {
        let token = self.consume_lookahead()?;
        match *token.kind() {
            TokenKind::Char(byte) => Ok(Term::Var(byte)),
            TokenKind::LBracket => {
                let term = if let TokenKind::Lambda = self.lookahead.kind() {
                    self.parse_abs()?
                } else {
                    self.parse_app()?
                };
                self.expect_token(TokenKind::RBracket)?;
                Ok(term)
            }
            _ => Err(token.into_unexpected()),
        }
    }

    fn parse_abs(&mut self) -> ParseResult<Term> {
        self.expect_token(TokenKind::Lambda)?;

        let token = self.consume_lookahead()?;
        let arg = if let TokenKind::Char(byte) = *token.kind() {
            byte
        } else {
            return Err(token.into_unexpected());
        };

        self.expect_token(TokenKind::Dot)?;
        self.expect_token(TokenKind::Space)?;
        let body = self.parse_term()?;

        Ok(Term::Abs(arg, Box::new(body)))
    }

    fn parse_app(&mut self) -> ParseResult<Term> {
        let t1 = self.parse_term()?;
        self.expect_token(TokenKind::Space)?;
        let t2 = self.parse_term()?;
        Ok(Term::App(Box::new(t1), Box::new(t2)))
    }
}
