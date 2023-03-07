use std::{cell::{RefCell, Ref}};

use crate::{error::LoxError, expr::Expr, stmt::Stmt, token::Object, environment::Environment};

mod expr_interpreter;
mod stmt_interpreter;

pub struct Interpreter {
    pub environment: RefCell<Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { environment: RefCell::new(Environment::new()) }
    }
    pub fn interpreter(&self, statements: &Vec<Stmt>) {
        for stmt in statements {
            match self.execute(stmt) {
                Ok(_) => (),
                Err(e) => e.report(""),
            }
        }
    }

    pub fn print_environment(&self){
        println!("{:?}", self.environment);
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
