use std::{cell::RefCell, collections::HashMap};

use crate::{
    error::LoxResult,
    expr::Expr,
    interpreter::Interpreter,
    stmt::Stmt, token::Token,
};

mod expr_resolver;
mod stmt_resolver;

pub struct Resolver {
    interpreter: Interpreter,
    scopes: RefCell<Vec<RefCell<HashMap<String, bool>>>>,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        Self {
            interpreter,
            scopes: RefCell::new(Vec::new()),
        }
    }

    pub fn resolve(&self, statements: &[Stmt]) -> Result<(), LoxResult>{
        for stmt in statements {
            self.resolve_stmt(stmt)?;
        }

        Ok(())
    }

    pub fn resolve_stmt(&self, stmt: &Stmt) -> Result<(), LoxResult> {
        stmt.accept(self)
    }

    pub fn resolve_expr(&self, expr: &Expr) -> Result<(), LoxResult> {
        expr.accept(self)
    }

    fn begin_scope(&self) {
        self.scopes.borrow_mut().push(RefCell::new(HashMap::new()));
    }

    fn end_scope(&self) {
        self.scopes.borrow_mut().pop();
    }

    fn declare(&self, token: &Token) {
        if !self.scopes.borrow().is_empty() {
            self.scopes.borrow().last().unwrap().borrow_mut().insert(token.as_string(), false);
        }
    }

    fn define(&self, token: &Token) {
        if !self.scopes.borrow().is_empty() {
            self.scopes.borrow().last().unwrap().borrow_mut().insert(token.as_string(), true);
        }
    }
}
