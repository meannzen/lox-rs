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

    fn visit_while(&mut self, condition: &Expression, body: &Statement) -> Result<(), E>;
    fn visit_for(
        &mut self,
        initialize: &Option<Statement>,
        condition: &Option<Expression>,
        increment: &Option<Expression>,
        body: &Statement,
    ) -> Result<(), E>;

    fn visit_call_expr(&mut self, callee: &Expression, args: &[Expression]) -> Result<T, E>;

    fn visit_function_stms(&mut self, name: &str, params: &[String], body: &[Statement]);

    fn visit_return_stms(&mut self, stms: &Option<Expression>) -> Result<(), E>;

    fn visit_class(&mut self, stmt: &Statement) -> Result<(), E>;
}
