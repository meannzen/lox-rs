use crate::TokenKind;

#[derive(Debug, Clone)]
pub enum Statement {
    Expr(Expression),
    Block(Vec<Statement>),
    Class {
        name: String,
        methods: Vec<Statement>,
    },
    Print(Expression),
    Var {
        name: String,
        initializer: Option<Expression>,
    },

    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    While {
        condition: Box<Expression>,
        body: Box<Statement>,
    },

    For {
        initialize: Option<Box<Statement>>,
        condition: Option<Expression>,
        increment: Option<Expression>,
        body: Box<Statement>,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },

    Return {
        value: Option<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Assign {
        name: String,
        value: Box<Expression>,
        resolved: Option<usize>,
    },
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
    Variable {
        name: String,
        resolved: Option<usize>,
    },
    Logical {
        left: Box<Expression>,
        operator: TokenKind,
        right: Box<Expression>,
    },

    Call {
        callee: Box<Expression>,
        args: Vec<Expression>,
    },

    Set {
        object: Box<Expression>,
        property: String,
        value: Box<Expression>,
    },
    Get {
        object: Box<Expression>,
        name: String,
    },
    This {
        resolved: Option<usize>,
    },
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Expr(expr) => write!(f, "{expr}"),
            Statement::Print(expr) => write!(f, "{expr}"),
            Statement::Var { name, initializer } => write!(f, "{name}: {:?}", initializer),
            Statement::Block(list) => write!(f, "{list:?}"),
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => write!(
                f,
                "condition:{}, then: {}, else {:?}",
                condition, then_branch, else_branch
            ),
            Statement::While { condition, body } => {
                write!(f, "condition {}, body {}", condition, body)
            }

            Statement::For {
                initialize,
                condition,
                increment,
                body,
            } => write!(f, "init :{initialize:?} condition:{condition:?} increment: {increment:?} body {body:?}"),
            Statement::Function { name, params, body } => {write!(f, "function {name}({params:?}){body:?}")},
            Statement::Return { value }=> write!(f, "{value:?}"),
            Statement::Class { name, methods: _ } => write!(f, "{name}")
        }
    }
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
                write!(f, "({} {} {})", op_str, left, right)
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
                write!(f, "({} {})", op, expression)
            }
            Expression::Variable { name, resolved: _ } => write!(f, "{name:?}"),
            Expression::Assign {
                name,
                value,
                resolved: _,
            } => write!(f, "{name}={value}"),
            Expression::Logical {
                left: lelf,
                operator,
                right,
            } => write!(f, "{} {:?} {}", lelf, operator, right),

            Expression::Call { callee, args } => write!(f, "{}, {:?}", callee, args),

            Expression::Set {
                object,
                property,
                value,
            } => write!(f, "{object:?} {property} {value:?}"),
            Expression::Get { object, name } => write!(f, "{object:?}.{name}"),
            Expression::This { resolved } => write!(f, "{resolved:?}"),
        }
    }
}
