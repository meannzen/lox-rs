use crate::{Expression, Literal, TokenKind, Visitor};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Nil,
    String(String),
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
            Expression::Unary {
                operator,
                expression,
            } => self.visit_unary_expr(expression, operator),
            Expression::Group(expr) => self.visit_expr(expr),
            _ => unimplemented!(),
        }
    }
    fn visit_literal_expr(&mut self, literal: &crate::Literal) -> Value {
        match literal {
            Literal::Number(v) => Value::Number(*v),
            Literal::Boolean(v) => Value::Boolean(*v),
            Literal::Nil => Value::Nil,
            Literal::String(v) => Value::String(v.clone()),
        }
    }

    fn visit_unary_expr(&mut self, expr: &Expression, _op: &TokenKind) -> Value {
        self.visit_expr(expr)
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
            Value::String(v) => write!(f, "{v}"),
        }
    }
}
