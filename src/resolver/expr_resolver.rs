use crate::error::LoxResult;
use crate::expr::*;

use super::Resolver;

impl ExprVisitor<()> for Resolver {
    fn visit_assign_expr(&self, _expr: &AssignExpr) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_binary_expr(&self, _expr: &BinaryExpr) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_call_expr(&self, _expr: &CallExpr) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_grouping_expr(&self, _expr: &GroupingExpr) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_literal_expr(&self, _expr: &LiteralExpr) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_logical_expr(&self, _expr: &LogicalExpr) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_unary_expr(&self, _expr: &UnaryExpr) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_variable_expr(&self, _expr: &VariableExpr) -> Result<(), LoxResult> {
        Ok(())
    }
}
