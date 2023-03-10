use crate::core::*;
use crate::error::*;
use crate::token::*;

use std::rc::Rc;

#[derive(Debug)]
pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Logical(LogicalExpr),
    Unary(UnaryExpr),
    Call(CallExpr),
    Variable(VariableExpr),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxResult> {
        match self {
            Expr::Assign(b) => b.accept(visitor),
            Expr::Binary(b) => b.accept(visitor),
            Expr::Grouping(b) => b.accept(visitor),
            Expr::Literal(b) => b.accept(visitor),
            Expr::Logical(b) => b.accept(visitor),
            Expr::Unary(b) => b.accept(visitor),
            Expr::Call(b) => b.accept(visitor),
            Expr::Variable(b) => b.accept(visitor),
        }
    }
}

#[derive(Debug)]
pub struct AssignExpr {
    pub name: Token,
    pub value: Rc<Expr>,
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Rc<Expr>,
    pub operator: Token,
    pub right: Rc<Expr>,
}

#[derive(Debug)]
pub struct GroupingExpr {
    pub expression: Rc<Expr>,
}

#[derive(Debug)]
pub struct LiteralExpr {
    pub value: Object,
}

#[derive(Debug)]
pub struct LogicalExpr {
    pub left: Rc<Expr>,
    pub operator: Token,
    pub right: Rc<Expr>,
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Rc<Expr>,
}

#[derive(Debug)]
pub struct CallExpr {
    pub callee: Rc<Expr>,
    pub paren: Token,
    pub arguments: Vec<Rc<Expr>>,
}

#[derive(Debug)]
pub struct VariableExpr {
    pub name: Token,
}

impl AssignExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_assign_expr(self)
    }
}

impl BinaryExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_binary_expr(self)
    }
}

impl GroupingExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_grouping_expr(self)
    }
}

impl LiteralExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_literal_expr(self)
    }
}

impl LogicalExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_logical_expr(self)
    }
}

impl UnaryExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_unary_expr(self)
    }
}

impl CallExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_call_expr(self)
    }
}

impl VariableExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_variable_expr(self)
    }
}

pub trait ExprVisitor<T> {
    fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<T, LoxResult>;
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, LoxResult>;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T, LoxResult>;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T, LoxResult>;
    fn visit_logical_expr(&self, expr: &LogicalExpr) -> Result<T, LoxResult>;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T, LoxResult>;
    fn visit_call_expr(&self, expr: &CallExpr) -> Result<T, LoxResult>;
    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<T, LoxResult>;
}
