use anyhow::Result;

use crate::{ast::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr}, scanner::{LiteralValue, Token, TokenType}};

mod ast_printer;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr> {
        todo!()
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        while self.r#match(vec![TokenType::Bang, TokenType::BangEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;

            expr = Expr::Binary(BinaryExpr{
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            })
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;

        while self.r#match(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term()?;

            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;

        while self.r#match(vec![TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;

            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;

        while self.r#match(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.r#match(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr> {
        if self.r#match(vec![TokenType::Number, TokenType::String]) {
            let value = self.previous().literal.as_ref().unwrap();
            return Ok(Expr::Literal(LiteralExpr { value: value.clone() }));
        }

        if self.r#match(vec![TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr { value: LiteralValue::True }));
        }

        if self.r#match(vec![TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr { value: LiteralValue::False }));
        }

        if self.r#match(vec![TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr { value: LiteralValue::Nil }));
        }

        if self.r#match(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(
                TokenType::RightParen,
                format!("Expected ')' after expression on line {}", self.peek().line)
            )?;

            return Ok(Expr::Grouping(GroupingExpr { expression: Box::new(expr) }));
        }

        Err(anyhow::anyhow!(format!("Expected expression on line {}", self.peek().line)))
    }

    fn r#match(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.tokens[self.current].r#type == token_type
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn consume(&mut self, expected_type: TokenType, error_message: String) -> Result<&Token> {
        if self.check(expected_type) {
            return Ok(self.advance());
        }

        Err(anyhow::anyhow!(error_message))
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current-1]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current-1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().r#type == TokenType::Eof
    }
}