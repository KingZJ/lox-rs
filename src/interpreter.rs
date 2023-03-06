use crate::error::LoxError;
use crate::expr::{BinaryExpr, Expr, ExprVisitor, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::token::Object;
use crate::token_type::TokenType;

pub struct Interpreter {}

impl ExprVisitor<Object> for Interpreter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, LoxError> {
        let left = self.evaluate(expr.left.as_ref())?;
        let right = self.evaluate(expr.right.as_ref())?;

        // match left {
        //     Object::Number(left_num) => match right {
        //         Object::Number(right_num) => self.number_binary_evaluate(left_num, right_num, expr.operator.tk_type),
        //         _ => Err(LoxError::error(0, "invalid number".to_string())),
        //     },
        //     _ => Err(LoxError::error(0, "invalid number".to_string())),
        // }

        match (left, right) {
            (Object::Number(left_num), Object::Number(right_num)) => {
                self.number_binary_evaluate(left_num, right_num, expr.operator.tk_type)
            }
            // (Object::Str(left), Object::Str(right)) => {},
            (Object::Nil, Object::Nil) => Ok(Object::True),

            _ => Err(LoxError::error(
                expr.operator.line,
                "interpreter error invalid number".to_string(),
            )),
        }
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, LoxError> {
        self.evaluate(expr.expression.as_ref())
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, LoxError> {
        Ok(expr.value.clone())
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, LoxError> {
        let right = self.evaluate(expr.right.as_ref())?;
        match expr.operator.tk_type {
            TokenType::Minus => match right {
                Object::Number(num) => Ok(Object::Number(-num)),
                _ => Err(LoxError::error(0, "invalid number".to_string())),
            },
            TokenType::Bang => match right {
                Object::Nil | Object::False => Ok(Object::True),
                _ => Ok(Object::False),
            },
            _ => Ok(Object::Nil),
        }
    }
}

impl Interpreter {
    pub fn interpreter(&self, expr: &Expr) -> Option<Object> {
        match self.evaluate(expr) {
            Ok(obj) => {
                println!("evaluate value: {} primitive: {0:?}", obj);
                Some(obj)
            }
            Err(e) => {
                e.report("Runtime error");
                None
            }
        }
    }
    fn evaluate(&self, expr: &Expr) -> Result<Object, LoxError> {
        expr.accept(self)
    }

    fn number_binary_evaluate(
        &self,
        left_num: f64,
        right_num: f64,
        tk_type: TokenType,
    ) -> Result<Object, LoxError> {
        // good idea rewrite std::ops::{ADD, SUB}  std::cmp::PartialOrd for Object
        match tk_type {
            TokenType::Minus => Ok(Object::Number(left_num - right_num)),
            TokenType::Plus => Ok(Object::Number(left_num + right_num)),
            TokenType::Star => Ok(Object::Number(left_num * right_num)),
            TokenType::Slash => Ok(Object::Number(left_num / right_num)),
            TokenType::BangEqual => {
                if left_num != right_num {
                    Ok(Object::True)
                } else {
                    Ok(Object::False)
                }
            }
            TokenType::EqualEqual => {
                if left_num == right_num {
                    Ok(Object::True)
                } else {
                    Ok(Object::False)
                }
            }
            TokenType::Greater => {
                if left_num > right_num {
                    Ok(Object::True)
                } else {
                    Ok(Object::False)
                }
            }
            TokenType::GreaterEqual => {
                if left_num >= right_num {
                    Ok(Object::True)
                } else {
                    Ok(Object::False)
                }
            }
            TokenType::Less => {
                if left_num < right_num {
                    Ok(Object::True)
                } else {
                    Ok(Object::False)
                }
            }
            TokenType::LessEqual => {
                if left_num <= right_num {
                    Ok(Object::True)
                } else {
                    Ok(Object::False)
                }
            }

            _ => Err(LoxError::error(
                0,
                "interpreter error invalid operator".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{expr::{Expr, BinaryExpr}, token::{Object, Token}, token_type::TokenType};

    use super::Interpreter;

    fn make_literal(value: Object) -> Box<Expr> {
        Box::new(Expr::Literal(crate::expr::LiteralExpr { value }))
    }

    #[test]
    fn test_binary_add() {
        let left = make_literal(Object::Number(15.0));
        let right = make_literal(Object::Number(15.0));
        let operator = Token::new(TokenType::Plus, "+".to_string(), None, 10);
        let expr = Expr::Binary(BinaryExpr{left, operator, right});

        let interpreter = Interpreter{};
        let res = interpreter.evaluate(&expr);
        assert!(res.is_ok());
        assert_eq!(res.ok(), Some(Object::Number(30.0)));
    }

    #[test]
    fn test_comparison() {
        let operator = Token::new(TokenType::EqualEqual, "==".to_string(), None, 10);
        let expected: Vec<Object> = vec![Object::False, Object::True, Object::False];
        // expected.push(make_literal(value))
        test_binary_num(20.0, &operator, expected);
    }

    fn test_binary_num(left: f64, operator: &Token, expected: Vec<Object>) {
        let num = vec![5.0, 20.0, 50.0];
        for (b, right) in expected.iter().zip(num) {
            let left = make_literal(Object::Number(left));
            let right = make_literal(Object::Number(right));
            
            let expr = Expr::Binary(BinaryExpr{left, operator: operator.clone(), right});

            let interpreter = Interpreter{};
            let res = interpreter.evaluate(&expr);
            assert!(res.is_ok());
            assert_eq!(res.ok(), Some(b.clone()));
        }
    }
}
