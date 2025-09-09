use crate::{Expression, Literal, TokenKind};

pub trait Visitor<T, E: std::error::Error> {
    fn visit_expr(&mut self, expr: &Expression) -> Result<T, E>;
    fn visit_literal_expr(&mut self, literal: &Literal) -> Result<T, E>;
    fn visit_unary_expr(&mut self, expr: &Expression, op: &TokenKind) -> Result<T, E>;
    fn visit_binary_expr(&mut self, expr: &Expression) -> Result<T, E>;
}
