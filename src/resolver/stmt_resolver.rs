use crate::error::LoxResult;
use crate::stmt::*;

use super::Resolver;

impl StmtVisitor<()> for Resolver {
    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<(), LoxResult> {
        self.begin_scope();
        self.resolve(&stmt.statements)?;
        self.end_scope();
        Ok(())
    }

    fn visit_break_stmt(&self, _stmt: &BreakStmt) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_expression_stmt(&self, _stmt: &ExpressionStmt) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_function_stmt(&self, _stmt: &FunctionStmt) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_if_stmt(&self, _stmt: &IfStmt) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_print_stmt(&self, _stmt: &PrintStmt) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_return_stmt(&self, _stmt: &ReturnStmt) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_var_stmt(&self, stmt: &VarStmt) -> Result<(), LoxResult> {
        self.declare(&stmt.name);
        if let Some(ref init) = stmt.initializer {
            self.resolve_expr(init)?;
        }
        self.define(&stmt.name);
        Ok(())
    }

    fn visit_while_stmt(&self, _stmt: &WhileStmt) -> Result<(), LoxResult> {
        Ok(())
    }
}
