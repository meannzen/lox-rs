use crate::{Expression, Literal, Visitor};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Nil,
}

#[derive(Debug)]
pub enum InterpreterError {}

pub struct Interpreter;

impl Interpreter {
    pub fn run(expr: Expression) -> Result<(), InterpreterError> {
        let mut interpreter = Interpreter;
        let value = interpreter.visit_expr(&expr);
        interpreter.evaluate(value);
        Ok(())
    }

    fn evaluate(&self, value: Value) {
        println!("{value}");
    }
}

impl Visitor<Value> for Interpreter {
    fn visit_expr(&mut self, expr: &crate::Expression) -> Value {
        match expr {
            Expression::Literal(literal) => self.visit_literal_expr(literal),
            _ => unimplemented!(),
        }
    }
    fn visit_literal_expr(&mut self, literal: &crate::Literal) -> Value {
        match *literal {
            Literal::Number(v) => Value::Number(v),
            Literal::Boolean(v) => Value::Boolean(v),
            Literal::Nil => Value::Nil,
            _ => unimplemented!(),
        }
    }
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WTF, errror")
    }
}

impl std::error::Error for InterpreterError {}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(v) => write!(f, "{v}"),
            Value::Boolean(v) => write!(f, "{v}"),
            Value::Nil => write!(f, "nil"),
        }
    }
}
