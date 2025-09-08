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
            _ => self.visit_binary_expr(expr),
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

    fn visit_unary_expr(&mut self, expr: &Expression, op: &TokenKind) -> Value {
        let value = self.visit_expr(expr);
        match (op, value.clone()) {
            (TokenKind::Minus, val) => match val {
                Value::Number(v) => Value::Number(-v),
                _ => panic!("Error wtf"),
            },
            (TokenKind::Bang, val) => match val {
                Value::Boolean(v) => Value::Boolean(!v),
                Value::Nil => Value::Boolean(true),
                _ => Value::Boolean(false),
            },
            _ => value,
        }
    }

    fn visit_binary_expr(&mut self, expr: &Expression) -> Value {
        match expr {
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.visit_expr(left);
                let right = self.visit_expr(right);
                match (left, operator, right) {
                    (Value::Number(n), TokenKind::Plus, Value::Number(n1)) => Value::Number(n + n1),
                    (Value::String(s), TokenKind::Plus, Value::String(s1)) => {
                        let s = format!("{s}{s1}");
                        Value::String(s)
                    }
                    (Value::Number(n), TokenKind::Minus, Value::Number(n1)) => {
                        Value::Number(n - n1)
                    }
                    (Value::Number(n), TokenKind::Star, Value::Number(n1)) => Value::Number(n * n1),
                    (Value::Number(n), TokenKind::Slash, Value::Number(n1)) => {
                        Value::Number(n / n1)
                    }
                    (Value::Number(n), TokenKind::Greater, Value::Number(n1)) => {
                        Value::Boolean(n > n1)
                    }
                    (Value::Number(n), TokenKind::Less, Value::Number(n1)) => {
                        Value::Boolean(n < n1)
                    }

                    (Value::Number(n), TokenKind::GreaterEqual, Value::Number(n1)) => {
                        Value::Boolean(n >= n1)
                    }

                    (Value::Number(n), TokenKind::LessEqual, Value::Number(n1)) => {
                        Value::Boolean(n <= n1)
                    }
                    (Value::Number(n), TokenKind::EqualEqual, Value::Number(n1)) => {
                        Value::Boolean(n == n1)
                    }
                    (Value::Number(n), TokenKind::BangEqual, Value::Number(n1)) => {
                        Value::Boolean(n != n1)
                    }
                    (Value::String(s), op, Value::String(s2)) => match op {
                        TokenKind::EqualEqual => Value::Boolean(s == s2),
                        TokenKind::BangEqual => Value::Boolean(s != s2),
                        _ => panic!("I'm not Implement yet , Please contact to FBI"),
                    },
                    (Value::Number(_), op, Value::String(_)) => match op {
                        TokenKind::EqualEqual => Value::Boolean(false),
                        TokenKind::BangEqual => Value::Boolean(true),
                        _ => panic!("I'm not Implement yet , Please contact to FBI"),
                    },
                    (Value::String(_), op, Value::Number(_)) => match op {
                        TokenKind::EqualEqual => Value::Boolean(false),
                        TokenKind::BangEqual => Value::Boolean(true),
                        _ => panic!("I'm not Implement yet , Please contact to FBI"),
                    },
                    op => {
                        println!("{:?}", op);
                        panic!("No support fucking op")
                    }
                }
            }
            _ => self.visit_expr(expr),
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
            Value::String(v) => write!(f, "{v}"),
        }
    }
}
