use std::{collections::HashMap, sync::LazyLock};

use anyhow::Result;
use derive_more::Display;

static KEYWORDS: LazyLock<HashMap<String, TokenType>> = LazyLock::new(|| HashMap::from([
    ("and".to_string(), TokenType::And),
    ("class".to_string(), TokenType::Class),
    ("else".to_string(), TokenType::Else),
    ("false".to_string(), TokenType::False),
    ("for".to_string(), TokenType::For),
    ("fun".to_string(), TokenType::Fun),
    ("if".to_string(), TokenType::If),
    ("nil".to_string(), TokenType::Nil),
    ("or".to_string(), TokenType::Or),
    ("print".to_string(), TokenType::Print),
    ("return".to_string(), TokenType::Return),
    ("super".to_string(), TokenType::Super),
    ("this".to_string(), TokenType::This),
    ("true".to_string(), TokenType::True),
    ("var".to_string(), TokenType::Var),
    ("while".to_string(), TokenType::While),
]));

fn is_alpha(c: char) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_uppercase() || c == '_'
}

#[derive(Debug, Display, Clone, Copy)]
pub enum TokenType {
    // single character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // one or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // literals
    Identifier, String, Number,

    // keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof,
}

#[derive(Debug, Display)]
#[display("type: {}, lexeme: {}, literal: {:?}, line: {}", r#type, lexeme, literal, line)]
pub struct Token {
    r#type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

pub struct Scanner {
    source: String,
    current: usize,
    start: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            current: 0,
            start: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;

            match self.scan_token() {
                Ok(token) => {
                    if let Some(token) = token {
                        tokens.push(token);
                    }
                },
                Err(e) => errors.push(e),
            };
        }

        if !errors.is_empty() {
            let error_messages = errors.iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join("\n");

            return Err(anyhow::anyhow!("syntax errors:\n{}", error_messages));
        }

        tokens.push(Token { r#type: TokenType::Eof, lexeme: "".to_string(), literal: None, line: self.line });

        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Option<Token>> {
        let c = self.advance();

        let token = match c {
            // single character tokens
            '(' => Token { r#type: TokenType::LeftParen, lexeme: c.to_string(), literal: None, line: self.line },
            ')' => Token { r#type: TokenType::RightParen, lexeme: c.to_string(), literal: None, line: self.line },
            '{' => Token { r#type: TokenType::LeftBrace, lexeme: c.to_string(), literal: None, line: self.line },
            '}' => Token { r#type: TokenType::RightBrace, lexeme: c.to_string(), literal: None, line: self.line },
            ',' => Token { r#type: TokenType::Comma, lexeme: c.to_string(), literal: None, line: self.line },
            '.' => Token { r#type: TokenType::Dot, lexeme: c.to_string(), literal: None, line: self.line },
            '-' => Token { r#type: TokenType::Minus, lexeme: c.to_string(), literal: None, line: self.line },
            '+' => Token { r#type: TokenType::Plus, lexeme: c.to_string(), literal: None, line: self.line },
            ';' => Token { r#type: TokenType::Semicolon, lexeme: c.to_string(), literal: None, line: self.line },
            '*' => Token { r#type: TokenType::Star, lexeme: c.to_string(), literal: None, line: self.line },

            // operators
            '!' => {
                let token_type = if self.match_next('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };

                Token { r#type: token_type, lexeme: c.to_string(), literal: None, line: self.line }
            }
            '=' => {
                let token_type = if self.match_next('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };

                Token { r#type: token_type, lexeme: c.to_string(), literal: None, line: self.line }
            },
            '<' => {
                let token_type = if self.match_next('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };

                Token { r#type: token_type, lexeme: c.to_string(), literal: None, line: self.line }
            },
            '>' => {
                let token_type = if self.match_next('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };

                Token { r#type: token_type, lexeme: c.to_string(), literal: None, line: self.line }
            },

            // comments
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }

                    return Ok(None);
                } else {
                    Token { r#type: TokenType::Slash, lexeme: c.to_string(), literal: None, line: self.line }
                }
            },

            // whitespace
            ' ' | '\r' | '\t' => {
                return Ok(None);
            }
            '\n' => {
                self.line += 1;
                return Ok(None);
            }

            // string literals
            '"' => {
                self.string()?
            },

            c => {
                if c.is_ascii_digit() {
                    self.number()?
                } else if is_alpha(c) {
                    self.identifier()?
                } else {
                    return Err(anyhow::anyhow!("Unexpected character: {c} on line {}", self.line));
                }
            },
        };

        Ok(Some(token))
    }

    fn advance(&mut self) -> char {
        let c = self.source.as_bytes()[self.current];
        self.current += 1;

        c as char
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.as_bytes()[self.current+1] as char != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.as_bytes()[self.current+1] as char
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.as_bytes()[self.current+1] as char
        }
    }

    fn string(&mut self) -> Result<Token> {
        let line = self.line;

        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err(anyhow::anyhow!("Unterminated string."));
        }

        // closing "
        self.advance();

        let value = String::from_utf8(self.source.as_bytes()[self.start+1..self.current-1].to_vec())?;
        let lexeme = String::from_utf8(self.source.as_bytes()[self.start..self.current].to_vec())?;
        let token = Token { r#type: TokenType::String, lexeme, literal: Some(value), line };

        Ok(token)
    }

    fn number(&mut self) -> Result<Token> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
        }

        while self.peek().is_ascii_digit() {
            self.advance();
        }

        let value = String::from_utf8(self.source.as_bytes()[self.start..self.current].to_vec())?;
        let token = Token { r#type: TokenType::Number, lexeme: value.clone(), literal: Some(value), line: self.line };

        Ok(token)
    }

    fn identifier(&mut self) -> Result<Token> {
        while is_alpha(self.peek()) || self.peek().is_ascii_digit() {
            self.advance();
        }

        let value = String::from_utf8(self.source.as_bytes()[self.start..self.current].to_vec())?;
        let token_type = if let Some(token_type) = KEYWORDS.get(&value) {
            *token_type
        } else {
            TokenType::Identifier
        };

        let token = Token { r#type: token_type, lexeme: value.clone(), literal: Some(value), line: self.line };

        Ok(token)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}