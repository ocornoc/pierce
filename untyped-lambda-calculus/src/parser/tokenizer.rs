use super::result::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    loc: usize,
    kind: TokenKind,
}

impl Token {
    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn into_unexpected(self) -> ParseError {
        ParseError::new(self.loc, ParseErrorKind::UnexpectedToken(self.kind))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    LBracket,
    RBracket,
    Lambda,
    Char(u8),
    Dot,
    Space,
    EOF,
}

impl TokenKind {
    fn into_token(self, loc: usize) -> Token {
        Token { loc, kind: self }
    }
}

pub struct Tokenizer<'a> {
    input: &'a [u8],
    loc: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Tokenizer { input, loc: 0 }
    }

    fn tokenize(&self, byte: u8) -> ParseResult<Token> {
        use TokenKind::*;

        let kind = match byte {
            b'(' => LBracket,
            b')' => RBracket,
            b'\\' => Lambda,
            b'.' => Dot,
            b' ' => Space,
            byte if byte.is_ascii_lowercase() => Char(byte),
            _ => return Err(ParseError::new(self.loc, ParseErrorKind::InvalidByte(byte))),
        };
        Ok(kind.into_token(self.loc))
    }

    pub fn next_token(&mut self) -> ParseResult<Token> {
        if let Some(&byte) = self.input.get(self.loc) {
            let token = self.tokenize(byte)?;
            self.loc += 1;
            Ok(token)
        } else {
            Ok(TokenKind::EOF.into_token(self.loc))
        }
    }
}
