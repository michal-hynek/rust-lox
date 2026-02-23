use anyhow::Result;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum TokenType {
    // single character tokens
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // one or two character tokens
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,

    // literals
    IDENTIFIER, STRING, NUMBER,

    // keywords
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF,
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
                Ok(token) => tokens.push(token),
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

        tokens.push(Token { r#type: TokenType::EOF, lexeme: "".to_string(), literal: None, line: self.line });

        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Token> {
        let c = self.advance();

        let token = match c {
            // single character tokens
            '(' => Token { r#type: TokenType::LEFT_PAREN, lexeme: c.to_string(), literal: None, line: self.line },
            ')' => Token { r#type: TokenType::RIGHT_PAREN, lexeme: c.to_string(), literal: None, line: self.line },
            '{' => Token { r#type: TokenType::LEFT_BRACE, lexeme: c.to_string(), literal: None, line: self.line },
            '}' => Token { r#type: TokenType::RIGHT_BRACE, lexeme: c.to_string(), literal: None, line: self.line },
            ',' => Token { r#type: TokenType::COMMA, lexeme: c.to_string(), literal: None, line: self.line },
            '.' => Token { r#type: TokenType::DOT, lexeme: c.to_string(), literal: None, line: self.line },
            '-' => Token { r#type: TokenType::MINUS, lexeme: c.to_string(), literal: None, line: self.line },
            '+' => Token { r#type: TokenType::PLUS, lexeme: c.to_string(), literal: None, line: self.line },
            ';' => Token { r#type: TokenType::SEMICOLON, lexeme: c.to_string(), literal: None, line: self.line },
            '*' => Token { r#type: TokenType::STAR, lexeme: c.to_string(), literal: None, line: self.line },

            // operators
            '!' => {
                let token_type = if self.match_next('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };

                Token { r#type: token_type, lexeme: c.to_string(), literal: None, line: self.line }
            }
            '=' => {
                let token_type = if self.match_next('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };

                Token { r#type: token_type, lexeme: c.to_string(), literal: None, line: self.line }
            },
            '<' => {
                let token_type = if self.match_next('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };

                Token { r#type: token_type, lexeme: c.to_string(), literal: None, line: self.line }
            },
            '>' => {
                let token_type = if self.match_next('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };

                Token { r#type: token_type, lexeme: c.to_string(), literal: None, line: self.line }
            },

            _ => return Err(anyhow::anyhow!("Unexpected character: {c} on line {}", self.line)),
        };

        Ok(token)
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

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}