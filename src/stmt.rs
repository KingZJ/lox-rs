use crate::error::*;
use crate::expr::*;
use crate::token::*;

use std::rc::Rc;

#[derive(Debug)]
pub enum Stmt {
    Block(BlockStmt),
    Expression(ExpressionStmt),
    Function(FunctionStmt),
    If(IfStmt),
    Print(PrintStmt),
    Var(VarStmt),
    Return(ReturnStmt),
    Break(BreakStmt),
    While(WhileStmt),
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
        match self {
            Stmt::Block(b) => b.accept(visitor),
            Stmt::Expression(b) => b.accept(visitor),
            Stmt::Function(b) => b.accept(visitor),
            Stmt::If(b) => b.accept(visitor),
            Stmt::Print(b) => b.accept(visitor),
            Stmt::Var(b) => b.accept(visitor),
            Stmt::Return(b) => b.accept(visitor),
            Stmt::Break(b) => b.accept(visitor),
            Stmt::While(b) => b.accept(visitor),
        }
    }
}

#[derive(Debug)]
pub struct BlockStmt {
    pub statements: Rc<Vec<Rc<Stmt>>>,
}

#[derive(Debug)]
pub struct ExpressionStmt {
    pub expression: Rc<Expr>,
}

#[derive(Debug)]
pub struct FunctionStmt {
    pub name: Token,
    pub params: Rc<Vec<Token>>,
    pub body: Rc<Vec<Rc<Stmt>>>,
}

#[derive(Debug)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Rc<Stmt>,
    pub else_branch: Option<Rc<Stmt>>,
}

#[derive(Debug)]
pub struct PrintStmt {
    pub expression: Rc<Expr>,
}

#[derive(Debug)]
pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Rc<Expr>>,
}

#[derive(Debug)]
pub struct ReturnStmt {
    pub name: Token,
    pub value: Option<Rc<Expr>>,
}

#[derive(Debug)]
pub struct BreakStmt {
    pub u: usize,
}

#[derive(Debug)]
pub struct WhileStmt {
    pub condition: Rc<Expr>,
    pub body: Rc<Stmt>,
}

impl BlockStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_block_stmt(self)
    }
}

impl ExpressionStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_expression_stmt(self)
    }
}

impl FunctionStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_function_stmt(self)
    }
}

impl IfStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_if_stmt(self)
    }
}

impl PrintStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_print_stmt(self)
    }
}

impl VarStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_var_stmt(self)
    }
}

impl ReturnStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_return_stmt(self)
    }
}

impl BreakStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_break_stmt(self)
    }
}

impl WhileStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
        visitor.visit_while_stmt(self)
    }
}

pub trait StmtVisitor<T> {
    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<T, LoxResult>;
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<T, LoxResult>;
    fn visit_function_stmt(&self, stmt: &FunctionStmt) -> Result<T, LoxResult>;
    fn visit_if_stmt(&self, stmt: &IfStmt) -> Result<T, LoxResult>;
    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<T, LoxResult>;
    fn visit_var_stmt(&self, stmt: &VarStmt) -> Result<T, LoxResult>;
    fn visit_return_stmt(&self, stmt: &ReturnStmt) -> Result<T, LoxResult>;
    fn visit_break_stmt(&self, stmt: &BreakStmt) -> Result<T, LoxResult>;
    fn visit_while_stmt(&self, stmt: &WhileStmt) -> Result<T, LoxResult>;
}
