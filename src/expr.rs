use crate::token::*;
use crate::error::*;

#[derive(Debug)]
pub enum Expr {
	Assign(AssignExpr),
	Binary(BinaryExpr),
	Grouping(GroupingExpr),
	Literal(LiteralExpr),
	Unary(UnaryExpr),
	Variable(VariableExpr),
}

impl Expr {
	pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
		match self {
			Expr::Assign(b) => b.accept(visitor),
			Expr::Binary(b) => b.accept(visitor),
			Expr::Grouping(b) => b.accept(visitor),
			Expr::Literal(b) => b.accept(visitor),
			Expr::Unary(b) => b.accept(visitor),
			Expr::Variable(b) => b.accept(visitor),
		}
	}
}

#[derive(Debug)]
pub struct AssignExpr {
	pub name: Token,
	pub value: Box<Expr>,
}

#[derive(Debug)]
pub struct BinaryExpr {
	pub left: Box<Expr>,
	pub operator: Token,
	pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct GroupingExpr {
	pub expression: Box<Expr>,
}

#[derive(Debug)]
pub struct LiteralExpr {
	pub value: Object,
}

#[derive(Debug)]
pub struct UnaryExpr {
	pub operator: Token,
	pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct VariableExpr {
	pub name: Token,
}

impl AssignExpr {
	pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
		visitor.visit_assign_expr(self)
	}
}

impl BinaryExpr {
	pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
		visitor.visit_binary_expr(self)
	}
}

impl GroupingExpr {
	pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
		visitor.visit_grouping_expr(self)
	}
}

impl LiteralExpr {
	pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
		visitor.visit_literal_expr(self)
	}
}

impl UnaryExpr {
	pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
		visitor.visit_unary_expr(self)
	}
}

impl VariableExpr {
	pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
		visitor.visit_variable_expr(self)
	}
}

pub trait ExprVisitor<T> {
	fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<T, LoxError>;
	fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, LoxError>;
	fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T, LoxError>;
	fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T, LoxError>;
	fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T, LoxError>;
	fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<T, LoxError>;
}
