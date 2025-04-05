use miette::{miette, LabeledSpan, Report};

#[derive(Debug, Copy, Clone)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Question,
    Colon,

    String,
    Ident,
    EOF,
}

#[derive(Debug, Copy, Clone)]
pub struct Token<'a> {
    token_kind: TokenKind,
    position: usize,
    literal: &'a str
}

pub struct Lexer<'a>{
    source: &'a str,
    tokens: Vec<Result<Token<'a>, Report>>,
    position: usize,
    start: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer{source, tokens: vec![], position: 0, start: 0}
    }
    pub fn lex(&mut self) -> &Vec<Result<Token<'a>, Report>> {
        while self.position < self.source.len() {
            self.start = self.position;
            let c = self.source[self.position..].chars().next().unwrap();

            self.position += c.len_utf8();

            let token = match c {
                '(' => Ok(self.create_token(TokenKind::LeftParen)),
                ')' => Ok(self.create_token(TokenKind::RightParen)),
                '{' => Ok(self.create_token(TokenKind::LeftBrace)),
                '}' => Ok(self.create_token(TokenKind::RightBrace)),
                ',' => Ok(self.create_token(TokenKind::Comma)),
                '.' => Ok(self.create_token(TokenKind::Dot)),
                '-' => Ok(self.create_token(TokenKind::Minus)),
                '+' => Ok(self.create_token(TokenKind::Plus)),
                ';' => Ok(self.create_token(TokenKind::Semicolon)),
                '/' => {
                    if self.match_char('/') {
                        while self.position < self.source.len() &&!self.match_char('\n') {
                            if let Some(c) = self.peek() {
                                self.position += c.len_utf8();
                            }
                        }
                        continue;
                    } else {
                        Ok(self.create_token(TokenKind::Slash))
                    }
                },
                '*' => Ok(self.create_token(TokenKind::Star)),
                '!' => {
                    if self.match_char('=') {
                        Ok(self.create_token(TokenKind::BangEqual))
                    } else {
                        Ok(self.create_token(TokenKind::Bang))
                    }
                }
                '=' => {
                    if self.match_char('=') {
                        Ok(self.create_token(TokenKind::EqualEqual))
                    } else {
                        Ok(self.create_token(TokenKind::Equal))
                    }
                }
                '<' => {
                    if self.match_char('=') {
                        Ok(self.create_token(TokenKind::LessEqual))
                    } else {
                        Ok(self.create_token(TokenKind::Less))
                    }
                }
                '>' => {
                    if self.match_char('=') {
                        Ok(self.create_token(TokenKind::GreaterEqual))
                    } else {
                        Ok(self.create_token(TokenKind::Greater))
                    }
                }
                '"' => {
                    let rest = &self.source[self.start..];
                    let token = match rest[1..].find('"') {
                        Some(pos) => {
                            let end_offset = pos + 1;
                            self.position = self.start + end_offset + 1;
                            Ok(self.create_token(TokenKind::String))
                        },
                        None => {
                            Err(miette!(
                                help = "unterminated string",
                                labels = vec![LabeledSpan::at(self.start..self.source.len(), "string starts here but never closes")],
                                "unterminated string"
                            ).with_source_code(self.source.to_string()))
                        }
                    };
                    token
                }
                'a'..'z' | 'A'..'Z' | '_' => {
                    let rest = &self.source[self.start..];
                    let end_offset = rest.find(|c: char| {
                        !c.is_alphanumeric() && c != '_'
                    }).unwrap_or(rest.len());

                    self.position = self.start + end_offset;

                    Ok(self.create_token(TokenKind::Ident))
                }

                ' ' | '\r' | '\t' | '\n' => continue,
                _ => {
                    Err(miette!(
                    help = "unexpected character",
                    labels = vec![LabeledSpan::at(self.start, format!("unexpected '{}'", c))],
                    "unexpected character"
                    ).with_source_code(self.source.to_string()))

                }
            };
                self.tokens.push(token);
        }
        let eof_token = Token{token_kind: TokenKind::EOF, position: self.source.len(), literal: ""};
        self.tokens.push(Ok(eof_token));

        &self.tokens
    }

    fn create_token(&self, token_kind: TokenKind) -> Token<'a> {
        let literal = &self.source[self.start..self.position];
        Token {token_kind, position: self.start, literal}
    }

    fn peek(&self) -> Option<char> {
        self.source[self.position..].chars().next()
    }

    fn match_char(&mut self, expected: char) -> bool {
        let next = match self.peek() {
            None => return false,
            Some(c) => c,
        };

        if next == expected {
            self.position += next.len_utf8();
            true
        } else {
            false
        }

    }
}