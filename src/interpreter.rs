use crate::{ast::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr, Visitor}, scanner::{LiteralValue, TokenType}};

struct Interpreter {}

impl Visitor<LiteralValue> for Interpreter {
    fn visit_binary(&self, binary: &BinaryExpr) -> LiteralValue {
        todo!()
    }

    fn visit_grouping(&self, grouping: &GroupingExpr) -> LiteralValue {
        self.evaluate(&grouping.expression)
    }

    fn visit_literal(&self, literal: &LiteralExpr) -> LiteralValue {
        literal.value.clone()
    }

    fn visit_unary(&self, unary: &UnaryExpr) -> LiteralValue {
        let right = self.evaluate(&unary.right);

        match unary.operator.r#type {
            TokenType::Minus => {
                if let LiteralValue::Number(num) = right {
                    LiteralValue::Number(-num)
                } else {
                    // unreachable - Parser returns an error when "-" is used with a non-numeric value
                    LiteralValue::Nil
                }
            },
            TokenType::Bang => {
                if is_truthy(right) {
                    LiteralValue::False
                } else {
                    LiteralValue::True
                }
            },
            _ => {
                // unreachable
                // UnaryExpr supports two operands ! and -
                // Parser returns an error when any other operand is used, so the code below is unreachable
                LiteralValue::Nil
            }
        }
    }
}

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> LiteralValue {
        expr.accept(self)
    }
}

fn is_truthy(val: LiteralValue) -> bool {
    val != LiteralValue::False && val != LiteralValue::Nil
}

#[cfg(test)]
mod test_interpreter {
    use crate::scanner::{Token, TokenType};

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

    #[test]
    fn test_visit_grouping_with_literal_expr() {
        let grouping = GroupingExpr {
            expression: Box::new(Expr::Literal(
                LiteralExpr {
                    value: LiteralValue::Number(1.2),
                }
            )),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_grouping(&grouping);

        assert_eq!(LiteralValue::Number(1.2), val);
    }

    #[test]
    fn test_visit_unary_with_minus() {
        let unary = UnaryExpr {
            operator: Token { r#type: TokenType::Minus, lexeme: "-".to_string(), literal: None, line: 1 },
            right: Box::new(
                Expr::Literal(LiteralExpr {
                    value: LiteralValue::Number(2.1)
                })
            ),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_unary(&unary);

        assert_eq!(LiteralValue::Number(-2.1), val);
    }

    #[test]
    fn test_visit_unary_with_bang_true() {
        let unary = UnaryExpr {
            operator: Token { r#type: TokenType::Bang, lexeme: "!".to_string(), literal: None, line: 1 },
            right: Box::new(
                Expr::Literal(LiteralExpr {
                    value: LiteralValue::True
                })
            ),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_unary(&unary);

        assert_eq!(LiteralValue::False, val);
    }

    #[test]
    fn test_visit_unary_with_bang_false() {
        let unary = UnaryExpr {
            operator: Token { r#type: TokenType::Bang, lexeme: "!".to_string(), literal: None, line: 1 },
            right: Box::new(
                Expr::Literal(LiteralExpr {
                    value: LiteralValue::False
                })
            ),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_unary(&unary);

        assert_eq!(LiteralValue::True, val);
    }

    #[test]
    fn test_visit_unary_with_bang_nil() {
        let unary = UnaryExpr {
            operator: Token { r#type: TokenType::Bang, lexeme: "!".to_string(), literal: None, line: 1 },
            right: Box::new(
                Expr::Literal(LiteralExpr {
                    value: LiteralValue::Nil
                })
            ),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_unary(&unary);

        assert_eq!(LiteralValue::True, val);
    }

    #[test]
    fn test_visit_unary_with_bang_number() {
        let unary = UnaryExpr {
            operator: Token { r#type: TokenType::Bang, lexeme: "!".to_string(), literal: None, line: 1 },
            right: Box::new(
                Expr::Literal(LiteralExpr {
                    value: LiteralValue::Number(1.1)
                })
            ),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_unary(&unary);

        assert_eq!(LiteralValue::False, val);
    }
 
    #[test]
    fn test_visit_unary_with_bang_string() {
        let unary = UnaryExpr {
            operator: Token { r#type: TokenType::Bang, lexeme: "!".to_string(), literal: None, line: 1 },
            right: Box::new(
                Expr::Literal(LiteralExpr {
                    value: LiteralValue::String("foo".to_string())
                })
            ),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_unary(&unary);

        assert_eq!(LiteralValue::False, val);
    }
}