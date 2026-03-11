use crate::{ast::Expr, scanner::Token};

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

    pub fn parse(&mut self) -> Expr {
        todo!()
    }

    fn expression(&mut self) {
        self.equality();
    }

    fn equality(&mut self) {
        todo!();
    }
}