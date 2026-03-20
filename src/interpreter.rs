use crate::{ast::{BinaryExpr, GroupingExpr, LiteralExpr, UnaryExpr, Visitor}, scanner::LiteralValue};

struct Intepreter {}

impl Visitor<LiteralValue> for Intepreter {
    fn visit_binary(&self, binary: &BinaryExpr) -> LiteralValue {
        todo!()
    }

    fn visit_grouping(&self, grouping: &GroupingExpr) -> LiteralValue {
        todo!()
    }

    fn visit_literal(&self, literal: &LiteralExpr) -> LiteralValue {
        todo!()
    }

    fn visit_unary(&self, unary: &UnaryExpr) -> LiteralValue {
        todo!()
    }
}