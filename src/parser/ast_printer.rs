use crate::{ast::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr, ExprVisitor}, scanner::LiteralValue};

pub struct AstPrinter {
    expr: Expr,
}

impl AstPrinter {
    pub fn new(expr: Expr) -> Self{
        Self {
            expr,
        }
    }

    pub fn print(&self) -> String {
        self.expr.accept::<String>(self)
    }

    fn parenthesize(&self, name: &str, expressions: Vec<&Expr>) -> String {
        let mut expr_string = String::new();

        expr_string.push_str(&format!("({}", name));
        for expr in expressions {
            expr_string.push_str(&format!(" {}", expr.accept::<String>(self)));
        }
        expr_string.push(')');

        expr_string
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary(&self, binary: &BinaryExpr) -> String {
        self.parenthesize(&binary.operator.lexeme, vec![&binary.left, &binary.right])
    }

    fn visit_grouping(&self, grouping: &GroupingExpr) -> String {
        self.parenthesize("group", vec![&grouping.expression])
    }

    fn visit_literal(&self, literal: &LiteralExpr) -> String {
        match &literal.value {
            LiteralValue::Number(v) => v.to_string(),
            LiteralValue::String(v) => v.clone(),
            LiteralValue::True => "true".to_string(),
            LiteralValue::False => "false".to_string(),
            LiteralValue::Nil => "nil".to_string(),
        }
    }

    fn visit_unary(&self, unary: &UnaryExpr) -> String {
        self.parenthesize(&unary.operator.lexeme, vec![&unary.right])
    }
}

#[cfg(test)]
mod test_printer {
    use crate::scanner::{Token, TokenType};

    use super::*;

    #[test]
    fn test_print_with_complex_expression() {
        let expr = BinaryExpr {
            left: Box::new(
                Expr::Unary(UnaryExpr {
                    operator: Token {
                        r#type: TokenType::Minus,
                        lexeme: "-".to_string(),
                        literal: None,
                        line: 1
                    },
                    right: Box::new(
                        Expr::Literal(LiteralExpr {
                            value: LiteralValue::Number(123f64)
                        })
                    ),
                }
            )),
            operator: Token { r#type: TokenType::Star, lexeme: "*".to_string(), literal: None, line: 1 },
            right: Box::new(
                Expr::Grouping(GroupingExpr {
                    expression: Box::new(
                        Expr::Literal(LiteralExpr {
                            value: LiteralValue::Number(45.67)
                        })
                    )
                })
            ),
        };
        let printer = AstPrinter { expr: Expr::Binary(expr) };

        let expr_string = printer.print();

        assert_eq!("(* (- 123) (group 45.67))", &expr_string);
    }
}