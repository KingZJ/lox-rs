use std::{cell::RefCell, rc::Rc};

use crate::core::*;
use crate::{environment::Environment, error::LoxResult, expr::Expr, stmt::Stmt};

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

    fn is_truthy(&self, obj: &Object) -> bool {
        // match obj {
        //     Object::False | Object::Nil => false,
        //     _ => true,
        // }

        !matches!(obj, Object::False | Object::Nil)
    }

    // 语句执行器
    fn execute(&self, stmt: &Stmt) -> Result<(), LoxResult> {
        stmt.accept(self)
    }

    // 表达式解释器
    fn evaluate(&self, expr: &Expr) -> Result<Object, LoxResult> {
        expr.accept(self)
    }

    fn execute_block(
        &self,
        statements: &[Stmt],
        environment: Environment,
    ) -> Result<(), LoxResult> {
        let e = Rc::new(RefCell::new(environment));
        let previous = self.environment.replace(e);

        // try catch
        let result = statements.iter().try_for_each(|stmt| self.execute(stmt));

        self.environment.replace(previous);

        result
    }
}
