use crate::{Expression, Literal, Statement, TokenKind, Visitor};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Nil,
    String(String),
}

#[derive(Debug)]
pub enum InterpreterError {
    Unary,
    Binary,
}

pub struct Interpreter;

impl Interpreter {
    pub fn run(stmt: Vec<Statement>) -> Result<(), InterpreterError> {
        for st in stmt.into_iter() {
            match st {
                Statement::Print { expr } => {
                    let eval = Interpreter::evaluate(expr)?;
                    println!("{eval}");
                }
                Statement::Expr(expr) => {
                    let eval = Interpreter::evaluate(expr)?;
                    println!("{eval}");
                }
            }
        }
        Ok(())
    }

    pub fn evaluate(expr: Expression) -> Result<Value, InterpreterError> {
        let mut interpreter = Interpreter;
        let value = interpreter.visit_expr(&expr)?;
        Ok(value)
    }
}

impl Visitor<Value, InterpreterError> for Interpreter {
    fn visit_expr(&mut self, expr: &crate::Expression) -> Result<Value, InterpreterError> {
        let value = match expr {
            Expression::Literal(literal) => self.visit_literal_expr(literal)?,
            Expression::Unary {
                operator,
                expression,
            } => self.visit_unary_expr(expression, operator)?,
            Expression::Group(expr) => self.visit_expr(expr)?,
            _ => self.visit_binary_expr(expr)?,
        };

        Ok(value)
    }
    fn visit_literal_expr(&mut self, literal: &crate::Literal) -> Result<Value, InterpreterError> {
        let value = match literal {
            Literal::Number(v) => Value::Number(*v),
            Literal::Boolean(v) => Value::Boolean(*v),
            Literal::Nil => Value::Nil,
            Literal::String(v) => Value::String(v.clone()),
        };

        Ok(value)
    }

    fn visit_unary_expr(
        &mut self,
        expr: &Expression,
        op: &TokenKind,
    ) -> Result<Value, InterpreterError> {
        let value = self.visit_expr(expr)?;
        match (op, value.clone()) {
            (TokenKind::Minus, val) => match val {
                Value::Number(v) => Ok(Value::Number(-v)),
                _ => Err(InterpreterError::Unary),
            },
            (TokenKind::Bang, val) => match val {
                Value::Boolean(v) => Ok(Value::Boolean(!v)),
                Value::Nil => Ok(Value::Boolean(true)),
                _ => Ok(Value::Boolean(false)),
            },
            _ => Ok(value),
        }
    }

    fn visit_binary_expr(&mut self, expr: &Expression) -> Result<Value, InterpreterError> {
        match expr {
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.visit_expr(left)?;
                let right = self.visit_expr(right)?;
                match (left, operator, right) {
                    (Value::Number(n), TokenKind::Plus, Value::Number(n1)) => {
                        Ok(Value::Number(n + n1))
                    }
                    (Value::String(s), TokenKind::Plus, Value::String(s1)) => {
                        let s = format!("{s}{s1}");
                        Ok(Value::String(s))
                    }
                    (Value::Number(n), TokenKind::Minus, Value::Number(n1)) => {
                        Ok(Value::Number(n - n1))
                    }
                    (Value::Number(n), TokenKind::Star, Value::Number(n1)) => {
                        Ok(Value::Number(n * n1))
                    }
                    (Value::Number(n), TokenKind::Slash, Value::Number(n1)) => {
                        Ok(Value::Number(n / n1))
                    }
                    (Value::Number(n), TokenKind::Greater, Value::Number(n1)) => {
                        Ok(Value::Boolean(n > n1))
                    }
                    (Value::Number(n), TokenKind::Less, Value::Number(n1)) => {
                        Ok(Value::Boolean(n < n1))
                    }
                    (Value::Number(n), TokenKind::GreaterEqual, Value::Number(n1)) => {
                        Ok(Value::Boolean(n >= n1))
                    }
                    (Value::Number(n), TokenKind::LessEqual, Value::Number(n1)) => {
                        Ok(Value::Boolean(n <= n1))
                    }
                    (Value::Number(n), TokenKind::EqualEqual, Value::Number(n1)) => {
                        Ok(Value::Boolean(n == n1))
                    }
                    (Value::Number(n), TokenKind::BangEqual, Value::Number(n1)) => {
                        Ok(Value::Boolean(n != n1))
                    }
                    (Value::String(s), TokenKind::EqualEqual, Value::String(s2)) => {
                        Ok(Value::Boolean(s == s2))
                    }
                    (Value::String(s), TokenKind::BangEqual, Value::String(s2)) => {
                        Ok(Value::Boolean(s != s2))
                    }
                    (Value::Number(_), TokenKind::EqualEqual, Value::String(_)) => {
                        Ok(Value::Boolean(false))
                    }
                    (Value::Number(_), TokenKind::BangEqual, Value::String(_)) => {
                        Ok(Value::Boolean(true))
                    }
                    (Value::String(_), TokenKind::EqualEqual, Value::Number(_)) => {
                        Ok(Value::Boolean(false))
                    }
                    (Value::String(_), TokenKind::BangEqual, Value::Number(_)) => {
                        Ok(Value::Boolean(true))
                    }
                    _ => Err(InterpreterError::Binary),
                }
            }
            _ => self.visit_expr(expr),
        }
    }
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operand must be a number.")
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
