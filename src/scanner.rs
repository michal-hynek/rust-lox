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
}

#[derive(Debug, Display)]
#[display("type: {}, lexeme: {}, literal: {:?}, line: {}", r#type, lexeme, literal, line)]
pub struct Token {
    r#type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: &str, literal: Option<&str>, line: usize) -> Self {
        Self {
            r#type,
            lexeme: lexeme.to_string(),
            literal: literal.map(|literal| literal.to_string()),
            line,
        }
    }
}

pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
        }
    }

    pub fn scan_tokens(&self) -> Result<Vec<Token>> {
        Ok(vec![])
    }
}