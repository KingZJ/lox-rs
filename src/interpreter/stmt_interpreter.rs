use crate::error::LoxError;
use crate::stmt::{ExpressionStmt, PrintStmt, StmtVisitor, VarStmt};
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
        self.environment.borrow_mut().define(stmt.name.lexeme.clone(), value);
        Ok(())
    }
}

#[cfg(test)]
mod test {}
