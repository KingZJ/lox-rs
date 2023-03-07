// use std::rc::Rc;

use crate::environment::Environment;
use crate::error::LoxError;
use crate::stmt::*;
use crate::token::Object;

use super::Interpreter;

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<(), LoxError> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<(), LoxError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_var_stmt(&self, stmt: &VarStmt) -> Result<(), LoxError> {
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

    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<(), LoxError> {
        // println!("{:?}", self.environment);
        // Rc::clone(self.environment.as_ref())
        let e = self.environment.borrow().clone();
        self.execute_block(&stmt.statements, Environment::new_enclosing(e))?;
        Ok(())
    }

    fn visit_if_stmt(&self, stmt: &IfStmt) -> Result<(), LoxError> {
        if self.is_truthy(self.evaluate(&stmt.condition)?) {
            self.execute(&stmt.then_branch)
        } else if let Some(ref else_branch) = stmt.else_branch {
            self.execute(else_branch)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {}
