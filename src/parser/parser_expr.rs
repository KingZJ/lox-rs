use crate::error::LoxResult;
use crate::expr::*;
use crate::token::Object;
use crate::token_type::TokenType;

use super::Parser;

impl Parser {
    // expression     → assignment ;
    pub fn expression(&mut self) -> Result<Expr, LoxResult> {
        self.assignment()
    }

    // assignment     → IDENTIFIER "=" assignment | equality ;
    // a + b = c;
    // fn assignment(&mut self) -> Result<Expr, LoxError> {
    //     if self.is_match(&vec![TokenType::Identifier]) {
    //         let name = self.previous().unwrap();
    //         if self.is_match(&vec![TokenType::Equal]) {
    //             let value = self.assignment()?;
    //             Ok(Expr::Assign(AssignExpr { name, value: Box::new(value) }))
    //         } else {
    //             Err(LoxError::error(name.line, "parser error invalid assign".to_string()))
    //         }

    //     } else {
    //         self.equality()
    //     }
    // }
    // assignment     → IDENTIFIER "=" assignment | logic_or ;
    fn assignment(&mut self) -> Result<Expr, LoxResult> {
        let expr = self.logic_or()?;
        if self.is_match(&vec![TokenType::Equal]) {
            let equals = self.previous().unwrap();
            if let Expr::Variable(e) = expr {
                let name = e.name;
                let value = self.assignment()?;

                return Ok(Expr::Assign(AssignExpr {
                    name,
                    value: Box::new(value),
                }));
            }

            return Err(LoxResult::parse_error(equals, "invalid assign".to_string()));
        }

        Ok(expr)
    }

    // logic_or       → logic_and ( "or" logic_and )*
    fn logic_or(&mut self) -> Result<Expr, LoxResult> {
        let mut left = self.logic_and()?;

        while self.is_match(&vec![TokenType::Or]) {
            let operator = self.previous().unwrap();
            let right = self.logic_and()?;
            left = Expr::Logical(LogicalExpr {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    // logic_and      → equality ( "and" equality )* ;
    fn logic_and(&mut self) -> Result<Expr, LoxResult> {
        let mut left = self.equality()?;

        while self.is_match(&vec![TokenType::And]) {
            let operator = self.previous().unwrap();
            let right = self.equality()?;
            left = Expr::Logical(LogicalExpr {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Expr, LoxResult> {
        let mut left = self.comparison()?;
        let equality_operators = vec![TokenType::BangEqual, TokenType::EqualEqual];
        while self.is_match(&equality_operators) {
            let operator = self.previous().unwrap();
            let right = self.comparison()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Expr, LoxResult> {
        let mut left = self.term()?;
        let comparison_operators = vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];
        while self.is_match(&comparison_operators) {
            let operator = self.previous().unwrap();
            let right = self.term()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    // term           → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Expr, LoxResult> {
        let mut left = self.factor()?;
        let term_operators = vec![TokenType::Plus, TokenType::Minus];
        while self.is_match(&term_operators) {
            let operator = self.previous().unwrap();
            let right = self.factor()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    // factor         → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Result<Expr, LoxResult> {
        let mut left = self.unary()?;
        let factor_operators = vec![TokenType::Star, TokenType::Slash];
        while self.is_match(&factor_operators) {
            let operator = self.previous().unwrap();
            let right = self.unary()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    // unary          → ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Result<Expr, LoxResult> {
        let unary_operators = vec![TokenType::Bang, TokenType::Minus];
        if self.is_match(&unary_operators) {
            let operator = self.previous().unwrap();
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }));
        }

        self.primary()
    }

    // primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr, LoxResult> {
        if self.is_match(&vec![TokenType::Number, TokenType::String]) {
            let value = self.previous().unwrap().literal.unwrap();
            Ok(Expr::Literal(LiteralExpr { value }))
        } else if self.is_match(&vec![TokenType::False]) {
            Ok(Expr::Literal(LiteralExpr {
                value: Object::False,
            }))
        } else if self.is_match(&vec![TokenType::True]) {
            Ok(Expr::Literal(LiteralExpr {
                value: Object::True,
            }))
        } else if self.is_match(&vec![TokenType::Nil]) {
            Ok(Expr::Literal(LiteralExpr { value: Object::Nil }))
        } else if self.is_match(&vec![TokenType::Identifier]) {
            Ok(Expr::Variable(VariableExpr {
                name: self.previous().unwrap(),
            }))
        } else if self.is_match(&vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "expect `)` after expression")?;
            Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }))
        } else {
            let token = self.peek().unwrap();
            let message = "failed primary parse".to_string();
            Err(LoxResult::parse_error(token, message))
        }
    }
}
