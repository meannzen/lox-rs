use crate::{Expression, Statement, TokenKind};

pub trait Visitor<T, E: std::error::Error> {
    fn visit_expr(&mut self, expr: &Expression) -> Result<T, E>;
    fn visit_stmt(&mut self, stms: &Statement) -> Result<(), E>;
    fn visit_block(&mut self, list: &[Statement]) -> Result<(), E>;
    fn visit_if_stms(
        &mut self,
        condition: &Expression,
        then_branch: &Statement,
        else_branch: &Option<Box<Statement>>,
    ) -> Result<(), E>;
    fn visit_logical(
        &mut self,
        left: &Expression,
        operator: &TokenKind,
        right: &Expression,
    ) -> Result<T, E>;
}
