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
    Word(Vec<u8>),
    AlphaChar(u8),
    Dot,
    Colon,
    Arrow,
    Space,
    Equal,
    EOF,
}

impl TokenKind {
    fn into_token(self, loc: usize) -> Token {
        Token { loc, kind: self }
    }

    fn step(&self) -> usize {
        use TokenKind::*;

        match self {
            Arrow => 2,
            EOF => 0,
            Word(bytes) => bytes.len(),
            _ => 1,
        }
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

    pub fn next_token(&mut self) -> ParseResult<Token> {
        use TokenKind::*;

        if let Some(&byte) = self.input.get(self.loc) {
            let kind = match byte {
                b'(' => LBracket,
                b')' => RBracket,
                b'\\' => Lambda,
                b'.' => Dot,
                b' ' => Space,
                b':' => Colon,
                b'=' => Equal,
                b'-' => {
                    if self.input.get(self.loc + 1) == Some(&b'>') {
                        Arrow
                    } else {
                        return Err(ParseError::new(self.loc, ParseErrorKind::InvalidByte(byte)));
                    }
                }
                byte if byte.is_ascii_alphabetic() => {
                    let mut word = vec![byte];
                    let mut offset = 1;
                    while let Some(&byte) = self.input.get(self.loc + offset) {
                        if !byte.is_ascii_alphabetic() {
                            break;
                        }
                        word.push(byte);
                        offset += 1;
                    }
                    if offset > 1 {
                        Word(word)
                    } else {
                        AlphaChar(word[0])
                    }
                }
                _ => return Err(ParseError::new(self.loc, ParseErrorKind::InvalidByte(byte))),
            };
            let token = kind.into_token(self.loc);
            self.loc += token.kind.step();
            Ok(token)
        } else {
            Ok(TokenKind::EOF.into_token(self.loc))
        }
    }
}
