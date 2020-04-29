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
}

pub struct Tokenizer {
    input: String,
    loc: usize,
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        Tokenizer { input, loc: 0 }
    }

    pub fn next_token(&mut self) -> ParseResult<Token> {
        use TokenKind::*;

        let mut chars = self.input.get(self.loc..).unwrap_or("").chars().peekable();
        let mut offset = 0;
        if let Some(c) = chars.peek() {
            offset += c.len_utf8();
            let kind = match c {
                '(' => LBracket,
                ')' => RBracket,
                '\\' | 'λ' => Lambda,
                '.' => Dot,
                ' ' => Space,
                ':' => Colon,
                '=' => Equal,
                '-' => {
                    chars.next().unwrap();
                    match chars.peek() {
                        Some('>') => {
                            offset += '>'.len_utf8();
                            Arrow
                        },
                        Some(&c) => {
                            return Err(ParseError::new(self.loc, ParseErrorKind::InvalidChar(c)))
                        }
                        None => {
                            return Err(ParseError::new(
                                self.loc,
                                ParseErrorKind::UnexpectedToken(TokenKind::EOF),
                            ))
                        }
                    }
                }
                &c if c.is_alphabetic() => {
                    let mut word = String::new();
                    word.push(chars.next().unwrap());
                    while let Some(&c) = chars.peek() {
                        if !c.is_alphabetic() {
                            break;
                        }
                        offset += c.len_utf8();
                        word.push(chars.next().unwrap());
                    }
                    if offset > 1 {
                        Word(word)
                    } else {
                        AlphaChar(c)
                    }
                }
                &c => return Err(ParseError::new(self.loc, ParseErrorKind::InvalidChar(c))),
            };
            let token = kind.into_token(self.loc);
            self.loc += offset;
            Ok(token)
        } else {
            Ok(TokenKind::EOF.into_token(self.loc))
        }
    }
}
