use crate::error::LoxResult;
use crate::stmt::*;

use super::Resolver;

impl StmtVisitor<()> for Resolver {
    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<(), LoxResult> {
        self.begin_scope();
        self.resolve(&stmt.statements);
        self.end_scope();
        Ok(())
    }

    fn visit_break_stmt(&self, stmt: &BreakStmt) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_function_stmt(&self, stmt: &FunctionStmt) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_if_stmt(&self, stmt: &IfStmt) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_return_stmt(&self, stmt: &ReturnStmt) -> Result<(), LoxResult> {
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

    fn visit_while_stmt(&self, stmt: &WhileStmt) -> Result<(), LoxResult> {
        Ok(())
    }
}
