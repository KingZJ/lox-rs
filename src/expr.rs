use crate::token::{Token, Object};
use crate::error::LoxError;
// use std::io::BufRead;
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

impl Expr {
    fn accept<T>(&self, visitor: Box<&dyn ExprVisitor<T>>) -> Result<T, LoxError> {
        match self {
            Expr::Binary(b) => b.accept(visitor),    
            Expr::Grouping(b) => b.accept(visitor),    
            Expr::Literal(b) => b.accept(visitor),    
            Expr::Unary(b) => b.accept(visitor),    
        }
    }
}

// type LoxError = String;

pub struct BinaryExpr {
    left: Box<Expr>,
    operator: Token,  // Token
    right: Box<Expr>,
}

impl BinaryExpr {
    fn accept<T>(&self, visitor: Box<&dyn ExprVisitor<T>>) -> Result<T, LoxError> {
        visitor.visit_binary_expr(self)
    }
}

pub struct GroupingExpr {
    expression: Box<Expr>,
}

impl GroupingExpr {
    fn accept<T>(&self, visitor: Box<&dyn ExprVisitor<T>>) -> Result<T, LoxError> {
        visitor.visit_grouping_expr(self)
    }
}


pub struct LiteralExpr {
    value: Object,
}

impl LiteralExpr {
    fn accept<T>(&self, visitor: Box<&dyn ExprVisitor<T>>) -> Result<T, LoxError> {
        visitor.visit_literal_expr(self)
    }
}

pub struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
}

impl UnaryExpr {
    fn accept<T>(&self, visitor: Box<&dyn ExprVisitor<T>>) -> Result<T, LoxError> {
        visitor.visit_unary_expr(self)
    }
}

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, LoxError>;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T, LoxError>;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T, LoxError>;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T, LoxError>;
}