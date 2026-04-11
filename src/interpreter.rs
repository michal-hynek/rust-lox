use anyhow::Result;

use crate::{ast::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr, Visitor}, scanner::{LiteralValue, TokenType}};

pub struct Interpreter {}

impl Visitor<Result<LiteralValue>> for Interpreter {
    fn visit_binary(&self, binary: &BinaryExpr) -> Result<LiteralValue> {
        let left = self.evaluate(&binary.left)?;
        let right = self.evaluate(&binary.right)?;

        match binary.operator.r#type {
            TokenType::Minus => {
                let left = Self::get_num_val(left)?;
                let right = Self::get_num_val(right)?;

                Ok(LiteralValue::Number(left-right))
            },
            TokenType::Slash =>  {
                let left = Self::get_num_val(left)?;
                let right = Self::get_num_val(right)?;

                Ok(LiteralValue::Number(left/right))
            },
            TokenType::Star => {
                let left = Self::get_num_val(left)?;
                let right = Self::get_num_val(right)?;

                Ok(LiteralValue::Number(left*right))
            },
            TokenType::Plus => {
                let (addition, concatenation) = (
                    left.as_num().zip(right.as_num()).map(|(x, y)| x + y),
                    left.as_string().zip(right.as_string()).map(|(x, y)| x + &y),
                );

                match (addition, concatenation) {
                    (Some(result), None) => Ok(LiteralValue::Number(result)),
                    (None, Some(result)) => Ok(LiteralValue::String(result)),
                    _ => Err(anyhow::anyhow!("Expected number or string operads for '+' operation")),
                }
            },
            TokenType::Greater => {
                let left = Self::get_num_val(left)?;
                let right = Self::get_num_val(right)?;

                Ok(LiteralValue::from_bool(left > right))
            },
            TokenType::GreaterEqual => {
                let left = Self::get_num_val(left)?;
                let right = Self::get_num_val(right)?;

                Ok(LiteralValue::from_bool(left >= right))
            },
            TokenType::Less => {
                let left = Self::get_num_val(left)?;
                let right = Self::get_num_val(right)?;

                Ok(LiteralValue::from_bool(left < right))
            },
            TokenType::LessEqual => {
                let left = Self::get_num_val(left)?;
                let right = Self::get_num_val(right)?;

                Ok(LiteralValue::from_bool(left <= right))
            },
            TokenType::BangEqual => {
                if left != right {
                    Ok(LiteralValue::True)
                } else {
                    Ok(LiteralValue::False)
                }
            },
            TokenType::EqualEqual => {
                if left == right {
                    Ok(LiteralValue::True)
                } else {
                    Ok(LiteralValue::False)
                }
            },
            _ => Ok(LiteralValue::Nil),
        }
    }

    fn visit_grouping(&self, grouping: &GroupingExpr) -> Result<LiteralValue> {
        self.evaluate(&grouping.expression)
    }

    fn visit_literal(&self, literal: &LiteralExpr) -> Result<LiteralValue> {
        Ok(literal.value.clone())
    }

    fn visit_unary(&self, unary: &UnaryExpr) -> Result<LiteralValue> {
        let right = self.evaluate(&unary.right)?;

        match unary.operator.r#type {
            TokenType::Minus => {
                if let LiteralValue::Number(num) = right {
                    Ok(LiteralValue::Number(-num))
                } else {
                    // unreachable - Parser returns an error when "-" is used with a non-numeric value
                    Ok(LiteralValue::Nil)
                }
            },
            TokenType::Bang => {
                if is_truthy(right) {
                    Ok(LiteralValue::False)
                } else {
                    Ok(LiteralValue::True)
                }
            },
            _ => {
                // unreachable
                // UnaryExpr supports two operands ! and -
                // Parser returns an error when any other operand is used, so the code below is unreachable
                Ok(LiteralValue::Nil)
            }
        }
    }
}

impl Interpreter {
    pub fn interpret(&self, expr: &Expr) -> Result<()> {
        let result = self.evaluate(expr)?;
        println!("{result}");

        Ok(())
    }

    fn evaluate(&self, expr: &Expr) -> Result<LiteralValue> {
        expr.accept(self)
    }

    fn get_num_val(literal: LiteralValue) -> Result<f64> {
        if let LiteralValue::Number(val) = literal {
            Ok(val)
        } else {
            Err(anyhow::anyhow!("Expected a number"))
        }
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
    fn test_visit_literal_string() -> Result<()> {
        let literal_expr = LiteralExpr {
            value: LiteralValue::String("foo".to_string()),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_literal(&literal_expr)?;

        assert_eq!(LiteralValue::String("foo".to_string()), val);

        Ok(())
    }

    #[test]
    fn test_visit_literal_number() -> Result<()> {
        let literal_expr = LiteralExpr {
            value: LiteralValue::Number(123f64),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_literal(&literal_expr)?;

        assert_eq!(LiteralValue::Number(123f64), val);

        Ok(())
    }

    #[test]
    fn test_visit_literal_true() -> Result<()> {
        let literal_expr = LiteralExpr {
            value: LiteralValue::True,
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_literal(&literal_expr)?;

        assert_eq!(LiteralValue::True, val);

        Ok(())
    }

    #[test]
    fn test_visit_literal_false() -> Result<()> {
        let literal_expr = LiteralExpr {
            value: LiteralValue::False,
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_literal(&literal_expr)?;

        assert_eq!(LiteralValue::False, val);

        Ok(())
    }

    #[test]
    fn test_visit_literal_nil() -> Result<()> {
        let literal_expr = LiteralExpr {
            value: LiteralValue::Nil,
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_literal(&literal_expr)?;

        assert_eq!(LiteralValue::Nil, val);

        Ok(())
    }

    #[test]
    fn test_visit_grouping_with_literal_expr() -> Result<()> {
        let grouping = GroupingExpr {
            expression: Box::new(Expr::Literal(
                LiteralExpr {
                    value: LiteralValue::Number(1.2),
                }
            )),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_grouping(&grouping)?;

        assert_eq!(LiteralValue::Number(1.2), val);

        Ok(())
    }

    #[test]
    fn test_visit_unary_with_minus() -> Result<()> {
        let unary = UnaryExpr {
            operator: Token { r#type: TokenType::Minus, lexeme: "-".to_string(), literal: None, line: 1 },
            right: Box::new(
                Expr::Literal(LiteralExpr {
                    value: LiteralValue::Number(2.1)
                })
            ),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_unary(&unary)?;

        assert_eq!(LiteralValue::Number(-2.1), val);

        Ok(())
    }

    #[test]
    fn test_visit_unary_with_bang_true() -> Result<()> {
        let unary = UnaryExpr {
            operator: Token { r#type: TokenType::Bang, lexeme: "!".to_string(), literal: None, line: 1 },
            right: Box::new(
                Expr::Literal(LiteralExpr {
                    value: LiteralValue::True
                })
            ),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_unary(&unary)?;

        assert_eq!(LiteralValue::False, val);

        Ok(())
    }

    #[test]
    fn test_visit_unary_with_bang_false() -> Result<()> {
        let unary = UnaryExpr {
            operator: Token { r#type: TokenType::Bang, lexeme: "!".to_string(), literal: None, line: 1 },
            right: Box::new(
                Expr::Literal(LiteralExpr {
                    value: LiteralValue::False
                })
            ),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_unary(&unary)?;

        assert_eq!(LiteralValue::True, val);

        Ok(())
    }

    #[test]
    fn test_visit_unary_with_bang_nil() -> Result<()> {
        let unary = UnaryExpr {
            operator: Token { r#type: TokenType::Bang, lexeme: "!".to_string(), literal: None, line: 1 },
            right: Box::new(
                Expr::Literal(LiteralExpr {
                    value: LiteralValue::Nil
                })
            ),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_unary(&unary)?;

        assert_eq!(LiteralValue::True, val);

        Ok(())
    }

    #[test]
    fn test_visit_unary_with_bang_number() -> Result<()> {
        let unary = UnaryExpr {
            operator: Token { r#type: TokenType::Bang, lexeme: "!".to_string(), literal: None, line: 1 },
            right: Box::new(
                Expr::Literal(LiteralExpr {
                    value: LiteralValue::Number(1.1)
                })
            ),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_unary(&unary)?;

        assert_eq!(LiteralValue::False, val);

        Ok(())
    }
 
    #[test]
    fn test_visit_unary_with_bang_string() -> Result<()> {
        let unary = UnaryExpr {
            operator: Token { r#type: TokenType::Bang, lexeme: "!".to_string(), literal: None, line: 1 },
            right: Box::new(
                Expr::Literal(LiteralExpr {
                    value: LiteralValue::String("foo".to_string())
                })
            ),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_unary(&unary)?;

        assert_eq!(LiteralValue::False, val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_subtraction() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(3.0) } )),
            operator: Token { r#type: TokenType::Minus, lexeme: "-".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(2.0) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::Number(1.0), val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_division() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(4.0) } )),
            operator: Token { r#type: TokenType::Slash, lexeme: "/".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(2.0) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::Number(2.0), val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_multiplication() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(5.0) } )),
            operator: Token { r#type: TokenType::Star, lexeme: "*".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(3.0) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::Number(15.0), val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_addition() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(4.0) } )),
            operator: Token { r#type: TokenType::Plus, lexeme: "+".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(3.0) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::Number(7.0), val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_concatenation() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::String("Hello".to_string()) } )),
            operator: Token { r#type: TokenType::Plus, lexeme: "+".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::String("World".to_string()) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::String("HelloWorld".to_string()), val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_greater_comparison() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(1.0) } )),
            operator: Token { r#type: TokenType::Greater, lexeme: ">".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(2.0) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::False, val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_greater_comparison2() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(3.0) } )),
            operator: Token { r#type: TokenType::Greater, lexeme: ">".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(2.0) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::True, val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_greater_comparison3() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(2.0) } )),
            operator: Token { r#type: TokenType::Greater, lexeme: ">".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(2.0) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::False, val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_greater_or_equal_comparison() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(1.0) } )),
            operator: Token { r#type: TokenType::GreaterEqual, lexeme: ">=".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(2.0) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::False, val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_greater_or_equal_comparison2() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(3.0) } )),
            operator: Token { r#type: TokenType::GreaterEqual, lexeme: ">=".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(2.0) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::True, val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_greater_or_equal_comparison3() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(2.0) } )),
            operator: Token { r#type: TokenType::GreaterEqual, lexeme: ">=".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(2.0) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::True, val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_less_comparison() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(1.0) } )),
            operator: Token { r#type: TokenType::Less, lexeme: "<".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(2.0) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::True, val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_less_comparison2() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(3.0) } )),
            operator: Token { r#type: TokenType::Less, lexeme: "<".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(2.0) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::False, val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_less_comparison3() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(2.0) } )),
            operator: Token { r#type: TokenType::Less, lexeme: "<".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(2.0) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::False, val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_less_or_equal_comparison() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(1.0) } )),
            operator: Token { r#type: TokenType::LessEqual, lexeme: "<=".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(2.0) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::True, val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_less_or_equal_comparison2() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(3.0) } )),
            operator: Token { r#type: TokenType::LessEqual, lexeme: "<=".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(2.0) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::False, val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_less_or_equal_comparison3() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(2.0) } )),
            operator: Token { r#type: TokenType::LessEqual, lexeme: "<=".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(2.0) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::True, val);
        
        Ok(())
    }

    #[test]
    fn test_visit_binary_not_equal_strings() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::String("hello".to_string()) } )),
            operator: Token { r#type: TokenType::BangEqual, lexeme: "!=".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::String("hello".to_string()) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::False, val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_not_equal_strings2() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::String("hello".to_string()) } )),
            operator: Token { r#type: TokenType::BangEqual, lexeme: "!=".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::String("world".to_string()) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::True, val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_not_equal_numbers() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(1.1) } )),
            operator: Token { r#type: TokenType::BangEqual, lexeme: "!=".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(1.1) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::False, val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_not_equal_numbers2() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(1.1) } )),
            operator: Token { r#type: TokenType::BangEqual, lexeme: "!=".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(1.2) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::True, val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_equal_strings() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::String("hello".to_string()) } )),
            operator: Token { r#type: TokenType::EqualEqual, lexeme: "==".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::String("hello".to_string()) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::True, val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_equal_strings2() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::String("hello".to_string()) } )),
            operator: Token { r#type: TokenType::EqualEqual, lexeme: "==".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::String("world".to_string()) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::False, val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_equal_numbers() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(1.1) } )),
            operator: Token { r#type: TokenType::EqualEqual, lexeme: "==".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(1.1) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::True, val);

        Ok(())
    }

    #[test]
    fn test_visit_binary_equal_numbers2() -> Result<()> {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(1.1) } )),
            operator: Token { r#type: TokenType::EqualEqual, lexeme: "==".to_string(), literal: None, line: 1 },
            right: Box::new(Expr::Literal(LiteralExpr { value: LiteralValue::Number(1.2) })),
        };
        let interpreter = Interpreter {};

        let val = interpreter.visit_binary(&binary_expr)?;

        assert_eq!(LiteralValue::False, val);

        Ok(())
    }

}