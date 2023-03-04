use crate::token::{Token, Object};
use crate::error::LoxError;

pub enum Expr {
	Binary(BinaryExpr),
	Grouping(GroupingExpr),
	Literal(LiteralExpr),
	Unary(UnaryExpr),
}

impl Expr {
	pub fn accept<T>(&self, visitor: Box<&dyn ExprVisitor<T>>) -> Result<T, LoxError> {
		match self {
			Expr::Binary(b) => b.accept(visitor),
			Expr::Grouping(b) => b.accept(visitor),
			Expr::Literal(b) => b.accept(visitor),
			Expr::Unary(b) => b.accept(visitor),
		}
	}
}

pub struct BinaryExpr {
	left: Box<Expr>,
	operator: Token,
	right: Box<Expr>,
}

pub struct GroupingExpr {
	expression: Box<Expr>,
}

pub struct LiteralExpr {
	value: Object,
}

pub struct UnaryExpr {
	operator: Token,
	right: Box<Expr>,
}

impl BinaryExpr {
	pub fn accept<T>(&self, visitor: Box<&dyn ExprVisitor<T>>) -> Result<T, LoxError> {
		visitor.visit_binary_expr(self)
	}
}

impl GroupingExpr {
	pub fn accept<T>(&self, visitor: Box<&dyn ExprVisitor<T>>) -> Result<T, LoxError> {
		visitor.visit_grouping_expr(self)
	}
}

impl LiteralExpr {
	pub fn accept<T>(&self, visitor: Box<&dyn ExprVisitor<T>>) -> Result<T, LoxError> {
		visitor.visit_literal_expr(self)
	}
}

impl UnaryExpr {
	pub fn accept<T>(&self, visitor: Box<&dyn ExprVisitor<T>>) -> Result<T, LoxError> {
		visitor.visit_unary_expr(self)
	}
}

pub trait ExprVisitor<T> {
	pub fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, LoxError>;
	pub fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T, LoxError>;
	pub fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T, LoxError>;
	pub fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T, LoxError>;
}
