use crate::error::*;
use crate::expr::*;
use crate::token::*;

#[derive(Debug)]
pub enum Stmt {
    Block(BlockStmt),
    Expression(ExpressionStmt),
    If(IfStmt),
    Print(PrintStmt),
    Var(VarStmt),
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        match self {
            Stmt::Block(b) => b.accept(visitor),
            Stmt::Expression(b) => b.accept(visitor),
            Stmt::If(b) => b.accept(visitor),
            Stmt::Print(b) => b.accept(visitor),
            Stmt::Var(b) => b.accept(visitor),
        }
    }
}

#[derive(Debug)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

#[derive(Debug)]
pub struct ExpressionStmt {
    pub expression: Expr,
}

#[derive(Debug)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Debug)]
pub struct PrintStmt {
    pub expression: Expr,
}

#[derive(Debug)]
pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}

impl BlockStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_block_stmt(self)
    }
}

impl ExpressionStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_expression_stmt(self)
    }
}

impl IfStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_if_stmt(self)
    }
}

impl PrintStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_print_stmt(self)
    }
}

impl VarStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_var_stmt(self)
    }
}

pub trait StmtVisitor<T> {
    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<T, LoxError>;
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<T, LoxError>;
    fn visit_if_stmt(&self, stmt: &IfStmt) -> Result<T, LoxError>;
    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<T, LoxError>;
    fn visit_var_stmt(&self, stmt: &VarStmt) -> Result<T, LoxError>;
}
