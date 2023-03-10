use crate::error::LoxResult;
use crate::expr::*;

use super::Resolver;

impl ExprVisitor<()> for Resolver {
    fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_call_expr(&self, expr: &CallExpr) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_logical_expr(&self, expr: &LogicalExpr) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<(), LoxResult> {
        Ok(())
    }
}
