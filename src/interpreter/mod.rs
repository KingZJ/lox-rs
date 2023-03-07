use std::{cell::RefCell, rc::Rc};

use crate::{environment::Environment, error::LoxError, expr::Expr, stmt::Stmt, token::Object};

mod expr_interpreter;
mod stmt_interpreter;

#[derive(Default)]
pub struct Interpreter {
    pub environment: RefCell<Rc<RefCell<Environment>>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: RefCell::new(Rc::new(RefCell::new(Environment::new()))),
        }
    }
    pub fn interpreter(&self, statements: &Vec<Stmt>) {
        for stmt in statements {
            match self.execute(stmt) {
                Ok(_) => (),
                Err(e) => e.report(""),
            }
        }
    }

    pub fn print_environment(&self) {
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

    fn execute_block(
        &self,
        statements: &Vec<Stmt>,
        environment: Environment,
    ) -> Result<(), LoxError> {
        let e = Rc::new(RefCell::new(environment));
        let previous = self.environment.replace(e);

        // try catch
        let result = statements.iter().try_for_each(|stmt| self.execute(stmt));

        self.environment.replace(previous);

        result
    }
}
