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
    Word(String),
    AlphaChar(char),
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

pub struct Tokenizer {
    input: Vec<char>,
    loc: usize
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        Tokenizer { input: input.chars().collect(), loc: 0 }
    }

    pub fn next_token(&mut self) -> ParseResult<Token> {
        use TokenKind::*;

        if let Some(&c) = self.input.get(self.loc) {
            let kind = match c {
                '(' => LBracket,
                ')' => RBracket,
                '\\' | 'λ' => Lambda,
                '.' => Dot,
                ' ' => Space,
                ':' => Colon,
                '=' => Equal,
                '-' => {
                    if self.input.get(self.loc + 1) == Some(&'>') {
                        Arrow
                    } else {
                        return Err(ParseError::new(self.loc, ParseErrorKind::InvalidChar(c)));
                    }
                }
                c if c.is_alphabetic() => {
                    let mut word = c.to_string();
                    let mut offset = 1;
                    while let Some(&c) = self.input.get(self.loc + offset) {
                        if !c.is_alphabetic() {
                            break;
                        }
                        word.push(c);
                        offset += 1;
                    }
                    if offset > 1 {
                        Word(word)
                    } else {
                        AlphaChar(c)
                    }
                }
                _ => return Err(ParseError::new(self.loc, ParseErrorKind::InvalidChar(c))),
            };
            let token = kind.into_token(self.loc);
            self.loc += token.kind.step();
            Ok(token)
        } else {
            Ok(TokenKind::EOF.into_token(self.loc))
        }
    }
}
