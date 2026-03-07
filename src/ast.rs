use crate::scanner::Token;
use crate::scanner::LiteralValue;

pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

pub struct LiteralExpr {
    pub value: LiteralValue,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

pub trait Visitor<T> {
    fn visit_binary(&self, binary: &BinaryExpr) -> T;
    fn visit_grouping(&self, grouping: &GroupingExpr) -> T;
    fn visit_literal(&self, literal: &LiteralExpr) -> T;
    fn visit_unary(&self, unary: &UnaryExpr) -> T;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        match self {
            Expr::Binary(expr) => visitor.visit_binary(expr),
            Expr::Grouping(expr) => visitor.visit_grouping(expr),
            Expr::Literal(expr) => visitor.visit_literal(expr),
            Expr::Unary(expr) => visitor.visit_unary(expr),
        }
    }
}