use crate::{error::LoxError, expr::Expr, stmt::Stmt, token::Object};

mod expr_interpreter;
mod stmt_interpreter;

pub struct Interpreter {}

impl Interpreter {
    pub fn interpreter(&self, statements: &Vec<Stmt>) {
        for stmt in statements {
            match self.execute(stmt) {
                Ok(_) => (),
                Err(e) => e.report(""),
            }
        }
    }

    // 语句执行器
    fn execute(&self, stmt: &Stmt) -> Result<(), LoxError> {
        stmt.accept(self)
    }

    // 表达式解释器
    fn evaluate(&self, expr: &Expr) -> Result<Object, LoxError> {
        expr.accept(self)
    }
}
