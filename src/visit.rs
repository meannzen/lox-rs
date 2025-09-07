use crate::{Expression, Literal, TokenKind};

pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expression) -> T;
    fn visit_literal_expr(&mut self, literal: &Literal) -> T;
    fn visit_unary_expr(&mut self, expr: &Expression, op: &TokenKind) -> T;
    fn visit_binary_expr(&mut self, expr: &Expression) -> T;
}
