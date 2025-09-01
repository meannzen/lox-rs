use crate::TokenKind;

#[derive(Debug, Clone)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: TokenKind,
        right: Box<Expression>,
    },

    Literal(Literal),
    Unary {
        operator: TokenKind,
        expression: Box<Expression>,
    },
    Group(Box<Expression>),
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
            Literal::String(s) => write!(f, "{s}"),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(l) => write!(f, "{l}"),
            Expression::Group(expr) => write!(f, "(group {expr})"),
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let op_str = match operator {
                    TokenKind::Plus => "+",
                    TokenKind::Minus => "-",
                    TokenKind::Star => "*",
                    TokenKind::Slash => "/",
                    TokenKind::BangEqual => "!=",
                    TokenKind::EqualEqual => "==",
                    TokenKind::Greater => ">",
                    TokenKind::GreaterEqual => ">=",
                    TokenKind::Less => "<",
                    TokenKind::LessEqual => "<=",
                    _ => unimplemented!(),
                };
                let needs_parens = matches!(
                    operator,
                    TokenKind::BangEqual
                        | TokenKind::EqualEqual
                        | TokenKind::Greater
                        | TokenKind::GreaterEqual
                        | TokenKind::Less
                        | TokenKind::LessEqual
                        | TokenKind::Plus
                        | TokenKind::Minus
                );
                let left_needs_parens = needs_parens
                    && matches!(
                        left.as_ref(),
                        Expression::Binary {
                            operator: TokenKind::Star
                                | TokenKind::Slash
                                | TokenKind::Plus
                                | TokenKind::Minus
                                | TokenKind::Greater
                                | TokenKind::GreaterEqual
                                | TokenKind::Less
                                | TokenKind::LessEqual,
                            ..
                        }
                    );
                let right_needs_parens = needs_parens
                    && matches!(
                        right.as_ref(),
                        Expression::Binary {
                            operator: TokenKind::Star
                                | TokenKind::Slash
                                | TokenKind::Plus
                                | TokenKind::Minus
                                | TokenKind::Greater
                                | TokenKind::GreaterEqual
                                | TokenKind::Less
                                | TokenKind::LessEqual,
                            ..
                        }
                    );
                if left_needs_parens && right_needs_parens {
                    write!(f, "({}) {} ({})", left, op_str, right)
                } else if left_needs_parens {
                    write!(f, "({}) {} {}", left, op_str, right)
                } else if right_needs_parens {
                    write!(f, "{} {} ({})", left, op_str, right)
                } else {
                    write!(f, "{} {} {}", left, op_str, right)
                }
            }
            Expression::Unary {
                operator,
                expression,
            } => {
                let op = match operator {
                    TokenKind::Bang => "!",
                    TokenKind::Minus => "-",
                    _ => unimplemented!(),
                };

                write!(f, "({op} {expression})")
            }
        }
    }
}
