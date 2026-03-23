use crate::{ast::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr, Visitor}, scanner::LiteralValue};

struct Interpreter {}

impl Visitor<LiteralValue> for Interpreter {
    fn visit_binary(&self, binary: &BinaryExpr) -> LiteralValue {
        todo!()
    }

    fn visit_grouping(&self, grouping: &GroupingExpr) -> LiteralValue {
        todo!()
    }

    fn visit_literal(&self, literal: &LiteralExpr) -> LiteralValue {
        literal.value.clone()
    }

    fn visit_unary(&self, unary: &UnaryExpr) -> LiteralValue {
        todo!()
    }
}

#[cfg(test)]
mod test_interpreter {
    use super::*;

    #[test]
    fn test_visit_literal_string() {
        let literal_expr = LiteralExpr {
            value: LiteralValue::String("foo".to_string()),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_literal(&literal_expr);

        assert_eq!(LiteralValue::String("foo".to_string()), val);
    }

    #[test]
    fn test_visit_literal_number() {
        let literal_expr = LiteralExpr {
            value: LiteralValue::Number(123f64),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_literal(&literal_expr);

        assert_eq!(LiteralValue::Number(123f64), val);
    }

    #[test]
    fn test_visit_literal_true() {
        let literal_expr = LiteralExpr {
            value: LiteralValue::True,
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_literal(&literal_expr);

        assert_eq!(LiteralValue::True, val);
    }

    #[test]
    fn test_visit_literal_false() {
        let literal_expr = LiteralExpr {
            value: LiteralValue::False,
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_literal(&literal_expr);

        assert_eq!(LiteralValue::False, val);
    }

    #[test]
    fn test_visit_literal_nil() {
        let literal_expr = LiteralExpr {
            value: LiteralValue::Nil,
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_literal(&literal_expr);

        assert_eq!(LiteralValue::Nil, val);
    }
}