use crate::error::LoxError;
use crate::stmt::{ExpressionStmt, PrintStmt, StmtVisitor};

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

    fn visit_var_stmt(&self, stmt: &crate::stmt::VarStmt) -> Result<(), LoxError> {
        Ok(())
    }
}

#[cfg(test)]
mod test {}
