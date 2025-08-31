use crate::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },

    Literal(Literal),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{:.1}", n)
                } else {
                    write!(f, "{n}")
                }
            }
            Literal::Boolean(value) => write!(f, "{value}"),
            Literal::String(s) => write!(f, "\"{}\"", s),
            Literal::Nil => write!(f, "nil"),
        }
    }
}
