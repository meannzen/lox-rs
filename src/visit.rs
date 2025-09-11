use crate::Expression;

pub trait Visitor<T, E: std::error::Error> {
    fn visit_expr(&mut self, expr: &Expression) -> Result<T, E>;
}
