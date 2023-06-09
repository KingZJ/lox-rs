// use std::rc::Rc;

use std::ops::Deref;
use std::rc::Rc;

use crate::core::{LoxCallable, LoxFunction, Object};
use crate::environment::Environment;
use crate::error::LoxResult;
use crate::stmt::*;

use super::Interpreter;

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<(), LoxResult> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<(), LoxResult> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_var_stmt(&self, stmt: &VarStmt) -> Result<(), LoxResult> {
        let value = if let Some(ref initializer) = stmt.initializer {
            self.evaluate(initializer)?
        } else {
            Object::Nil
        };
        self.environment
            .borrow()
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<(), LoxResult> {
        // println!("{:?}", self.environment);
        // Rc::clone(self.environment.as_ref())
        let e = self.environment.borrow().clone();
        self.execute_block(&stmt.statements, Environment::new_enclosing(e))?;
        Ok(())
    }

    fn visit_if_stmt(&self, stmt: &IfStmt) -> Result<(), LoxResult> {
        if self.is_truthy(&self.evaluate(&stmt.condition)?) {
            self.execute(&stmt.then_branch)
        } else if let Some(ref else_branch) = stmt.else_branch {
            self.execute(else_branch)
        } else {
            Ok(())
        }
    }

    fn visit_while_stmt(&self, stmt: &WhileStmt) -> Result<(), LoxResult> {
        while self.is_truthy(&self.evaluate(&stmt.condition)?) {
            match self.execute(&stmt.body) {
                Err(LoxResult::Break) => break,
                Err(e) => return Err(e),
                Ok(_) => (),
            }
        }

        Ok(())
    }

    fn visit_break_stmt(&self, _stmt: &BreakStmt) -> Result<(), LoxResult> {
        Err(LoxResult::Break)
    }

    fn visit_function_stmt(&self, stmt: &FunctionStmt) -> Result<(), LoxResult> {
        let function = LoxFunction::new(stmt, self.environment.borrow().deref());
        self.environment.borrow().borrow_mut().define(
            stmt.name.as_string(),
            Object::Func(LoxCallable {
                func: Rc::new(function),
            }),
        );

        Ok(())
    }

    fn visit_return_stmt(&self, stmt: &ReturnStmt) -> Result<(), LoxResult> {
        let value = if let Some(ref expr) = stmt.value {
            self.evaluate(expr)?
        } else {
            Object::Nil
        };

        Err(LoxResult::return_error(value))
    }
}

#[cfg(test)]
mod test {}
