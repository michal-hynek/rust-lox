use std::{collections::HashMap, fmt, sync::LazyLock};

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

#[derive(Debug, Display, Clone, Copy, PartialEq)]
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

#[derive(Debug, Display, Clone)]
#[display("type: {}, lexeme: {}, literal: {:?}, line: {}", r#type, lexeme, literal, line)]
pub struct Token {
    pub r#type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub line: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

impl LiteralValue {
    pub fn as_num(&self) -> Option<f64> {
        match self {
            LiteralValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            LiteralValue::String(str) => Some(str.to_string()),
            _ => None,
        }
    }

    pub fn from_bool(val: bool) -> Self {
        if val {
            LiteralValue::True
        } else {
            LiteralValue::False
        }
    }
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralValue::Number(num) => write!(f, "{}", num),
            LiteralValue::String(s) => write!(f, "\"{}\"", s),
            LiteralValue::True => write!(f, "true"),
            LiteralValue::False => write!(f, "false"),
            LiteralValue::Nil => write!(f, "nil"),
        }
    }
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

        if self.source.as_bytes()[self.current] as char != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.as_bytes()[self.current] as char
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
        let token = Token { r#type: TokenType::String, lexeme, literal: Some(LiteralValue::String(value)), line };

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
        let num = value.parse().map_err(|_| anyhow::anyhow!(format!("line {}: '{}' is not a valid number", self.line, value)))?;
        let token = Token { r#type: TokenType::Number, lexeme: value.clone(), literal: Some(LiteralValue::Number(num)), line: self.line };

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

        let token = Token { r#type: token_type, lexeme: value.clone(), literal: Some(LiteralValue::String(value)), line: self.line };

        Ok(token)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

#[cfg(test)]
mod test_scanner {
    use super::*;

    #[test]
    fn test_scan_tokens_recognizes_single_lexemes() -> Result<()> {
        let mut scanner = Scanner::new("() {} , . - + ; *");
        let tokens = scanner.scan_tokens()?;

        assert_eq!(11, tokens.len());
        assert_eq!(TokenType::LeftParen, tokens[0].r#type);
        assert_eq!(TokenType::RightParen, tokens[1].r#type);
        assert_eq!(TokenType::LeftBrace, tokens[2].r#type);
        assert_eq!(TokenType::RightBrace, tokens[3].r#type);
        assert_eq!(TokenType::Comma, tokens[4].r#type);
        assert_eq!(TokenType::Dot, tokens[5].r#type);
        assert_eq!(TokenType::Minus, tokens[6].r#type);
        assert_eq!(TokenType::Plus, tokens[7].r#type);
        assert_eq!(TokenType::Semicolon, tokens[8].r#type);
        assert_eq!(TokenType::Star, tokens[9].r#type);
        assert_eq!(TokenType::Eof, tokens[10].r#type);

        Ok(())
    }

    #[test]
    fn test_scan_tokens_recognizes_comparions() -> Result<()> {
        let mut scanner = Scanner::new("! != = == > >= < <=");
        let tokens = scanner.scan_tokens()?;

        assert_eq!(9, tokens.len());
        assert_eq!(TokenType::Bang, tokens[0].r#type);
        assert_eq!(TokenType::BangEqual, tokens[1].r#type);
        assert_eq!(TokenType::Equal, tokens[2].r#type);
        assert_eq!(TokenType::EqualEqual, tokens[3].r#type);
        assert_eq!(TokenType::Greater, tokens[4].r#type);
        assert_eq!(TokenType::GreaterEqual, tokens[5].r#type);
        assert_eq!(TokenType::Less, tokens[6].r#type);
        assert_eq!(TokenType::LessEqual, tokens[7].r#type);
        assert_eq!(TokenType::Eof, tokens[8].r#type);

        Ok(())
    }

    #[test]
    fn test_scan_tokens_recognizes_literals() -> Result<()> {
        let mut scanner = Scanner::new("foo \"bar\" 123.45");
        let tokens = scanner.scan_tokens()?;

        assert_eq!(4, tokens.len());
        assert_eq!(TokenType::Identifier, tokens[0].r#type);
        assert_eq!(LiteralValue::String("foo".to_string()), *tokens[0].literal.as_ref().unwrap());
        assert_eq!(TokenType::String, tokens[1].r#type);
        assert_eq!(LiteralValue::String("bar".to_string()), *tokens[1].literal.as_ref().unwrap());
        assert_eq!(TokenType::Number, tokens[2].r#type);
        assert_eq!(LiteralValue::Number(123.45), *tokens[2].literal.as_ref().unwrap());
        assert_eq!(TokenType::Eof, tokens[3].r#type);

        Ok(())
    }

    #[test]
    fn test_scan_tokens_recognizes_tokens() -> Result<()> {
        let mut scanner = Scanner::new("and class else false fun for if nil or print return super this true var while");
        let tokens = scanner.scan_tokens()?;

        assert_eq!(17, tokens.len());
        assert_eq!(TokenType::And, tokens[0].r#type);
        assert_eq!(LiteralValue::String("and".to_string()), *tokens[0].literal.as_ref().unwrap());
        assert_eq!(TokenType::Class, tokens[1].r#type);
        assert_eq!(LiteralValue::String("class".to_string()), *tokens[1].literal.as_ref().unwrap());
        assert_eq!(TokenType::Else, tokens[2].r#type);
        assert_eq!(LiteralValue::String("else".to_string()), *tokens[2].literal.as_ref().unwrap());
        assert_eq!(TokenType::False, tokens[3].r#type);
        assert_eq!(LiteralValue::String("false".to_string()), *tokens[3].literal.as_ref().unwrap());
        assert_eq!(TokenType::Fun, tokens[4].r#type);
        assert_eq!(LiteralValue::String("fun".to_string()), *tokens[4].literal.as_ref().unwrap());
        assert_eq!(TokenType::For, tokens[5].r#type);
        assert_eq!(LiteralValue::String("for".to_string()), *tokens[5].literal.as_ref().unwrap());
        assert_eq!(TokenType::If, tokens[6].r#type);
        assert_eq!(LiteralValue::String("if".to_string()), *tokens[6].literal.as_ref().unwrap());
        assert_eq!(TokenType::Nil, tokens[7].r#type);
        assert_eq!(LiteralValue::String("nil".to_string()), *tokens[7].literal.as_ref().unwrap());
        assert_eq!(TokenType::Or, tokens[8].r#type);
        assert_eq!(LiteralValue::String("or".to_string()), *tokens[8].literal.as_ref().unwrap());
        assert_eq!(TokenType::Print, tokens[9].r#type);
        assert_eq!(LiteralValue::String("print".to_string()), *tokens[9].literal.as_ref().unwrap());
        assert_eq!(TokenType::Return, tokens[10].r#type);
        assert_eq!(LiteralValue::String("return".to_string()), *tokens[10].literal.as_ref().unwrap());
        assert_eq!(TokenType::Super, tokens[11].r#type);
        assert_eq!(LiteralValue::String("super".to_string()), *tokens[11].literal.as_ref().unwrap());
        assert_eq!(TokenType::This, tokens[12].r#type);
        assert_eq!(LiteralValue::String("this".to_string()), *tokens[12].literal.as_ref().unwrap());
        assert_eq!(TokenType::True, tokens[13].r#type);
        assert_eq!(LiteralValue::String("true".to_string()), *tokens[13].literal.as_ref().unwrap());
        assert_eq!(TokenType::Var, tokens[14].r#type);
        assert_eq!(LiteralValue::String("var".to_string()), *tokens[14].literal.as_ref().unwrap());
        assert_eq!(TokenType::While, tokens[15].r#type);
        assert_eq!(LiteralValue::String("while".to_string()), *tokens[15].literal.as_ref().unwrap());
        assert_eq!(TokenType::Eof, tokens[16].r#type);

        Ok(())
    }

    #[test]
    fn test_scan_tokens_ignores_comments() -> Result<()> {
        let mut scanner = Scanner::new("// hello world \n for");
        let tokens = scanner.scan_tokens()?;

        assert_eq!(2, tokens.len());
        assert_eq!(TokenType::For, tokens[0].r#type);
        assert_eq!(TokenType::Eof, tokens[1].r#type);

        Ok(())
    }
}