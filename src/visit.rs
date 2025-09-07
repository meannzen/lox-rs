use crate::{Expression, Literal};

pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expression) -> T;
    fn visit_literal_expr(&mut self, literal: &Literal) -> T;
}
