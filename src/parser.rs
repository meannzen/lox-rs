use std::{collections::HashMap, iter::Peekable};

use crate::{ast::Expression, Lexer, Statement, Token, TokenKind};

#[derive(Debug)]
pub enum ParserError {
    Message(String),
    UnexpectedEof { line: usize },
    UnexpectedToken { line: usize, token: String },
    InvalidAssignmentTarget { line: usize, token: String },
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedEof { line } => {
                write!(f, "[line {}] Error: Unexpected EOF", line)
            }
            ParserError::UnexpectedToken { line, token } => {
                write!(f, "[line {line}] Error at '{token}': Expect expression.")
            }
            ParserError::InvalidAssignmentTarget { line, token } => {
                write!(
                    f,
                    "[line {line}] Error at '{token}': Invalid assignment target."
                )
            }

            ParserError::Message(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for ParserError {}

pub struct Parser<'input> {
    tokens: Peekable<Lexer<'input>>,
    had_error: bool,
    function_names: HashMap<String, usize>, // this fuckup [name function , total_argument]
}

impl<'input> Parser<'input> {
    pub fn new(input: &'input str) -> Self {
        Parser {
            tokens: Lexer::new(input).peekable(),
            had_error: false,
            function_names: HashMap::new(),
        }
    }

    pub fn parse_statements(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut statements = Vec::new();
        let mut errors = Vec::new();

        while self.peek().is_some() {
            match self.statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    errors.push(e);
                    self.had_error = true;
                    self.synchronize();
                }
            }
        }

        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(errors.into_iter().next().unwrap())
        }
    }

    fn statement(&mut self) -> Result<Statement, ParserError> {
        if let Some(token) = self.peek() {
            match token.kind {
                TokenKind::Print => self.print_statement(),
                TokenKind::Var => self.declaration(),
                TokenKind::LeftBrace => self.block(),
                TokenKind::If => self.if_statement(),
                TokenKind::While => self.while_statement(),
                TokenKind::For => self.for_statement(),
                TokenKind::Fun => self.function(),
                TokenKind::Return => self.return_statement(),
                _ => self.expr_statement(),
            }
        } else {
            Err(ParserError::UnexpectedEof { line: 1 })
        }
    }

    fn declaration(&mut self) -> Result<Statement, ParserError> {
        self.advance().unwrap(); // Consume 'var'
        let variable = self.consume(TokenKind::Identifier)?;
        let mut initializer: Option<Expression> = None;

        if self.peek().map(|t| t.kind) == Some(TokenKind::Equal) {
            self.advance().unwrap(); // Consume '='
            initializer = Some(self.expression()?);
        }

        self.consume(TokenKind::Semi)?;

        Ok(Statement::Var {
            name: variable.literal,
            initializer,
        })
    }

    fn return_statement(&mut self) -> Result<Statement, ParserError> {
        self.advance().unwrap(); // Consome 'return'
        let mut value = None;
        if self.peek().map(|t| t.kind) != Some(TokenKind::Semi) {
            value = Some(self.expression()?);
        }

        self.consume(TokenKind::Semi)?;

        Ok(Statement::Return { value })
    }

    fn function(&mut self) -> Result<Statement, ParserError> {
        self.advance().unwrap(); // Consume 'var'
        let function_name = self.peek().unwrap().literal.clone();
        self.consume(TokenKind::Identifier)?;
        self.consume(TokenKind::LeftParen)?;
        let mut params = vec![];
        if self.peek().map(|t| t.kind) != Some(TokenKind::RightParen) {
            loop {
                if params.len() >= 255 {
                    return Err(ParserError::Message(
                        "Cannot have more than 255 parameters.".to_string(),
                    ));
                }
                let param = self.consume(TokenKind::Identifier)?;
                params.push(param.literal);

                if self.peek().map(|t| t.kind) != Some(TokenKind::Comma) {
                    break;
                }
                self.advance().unwrap(); // Consume ','
            }
        }
        self.consume(TokenKind::RightParen)?;

        let body = match self.block()? {
            Statement::Block(v) => v,
            _ => unreachable!(),
        };

        self.function_names
            .insert(function_name.clone(), params.len());

        Ok(Statement::Function {
            name: function_name,
            params,
            body,
        })
    }

    fn print_statement(&mut self) -> Result<Statement, ParserError> {
        self.advance().unwrap(); // Consume 'print'
        let expr = self.expression()?;
        self.consume(TokenKind::Semi)?;
        Ok(Statement::Print(expr))
    }

    fn expr_statement(&mut self) -> Result<Statement, ParserError> {
        let expr = self.expression()?;
        self.consume(TokenKind::Semi)?;
        Ok(Statement::Expr(expr))
    }

    fn block(&mut self) -> Result<Statement, ParserError> {
        self.advance().unwrap(); // Consume '{'
        let mut blocks = Vec::new();

        while let Some(token) = self.peek() {
            if token.kind == TokenKind::RightBrace {
                break;
            }
            blocks.push(self.statement()?);
        }

        self.consume(TokenKind::RightBrace)?;
        Ok(Statement::Block(blocks))
    }

    fn if_statement(&mut self) -> Result<Statement, ParserError> {
        self.advance().unwrap(); // Consume 'if'
        self.consume(TokenKind::LeftParen)?;
        let condition = self.expression()?;
        self.consume(TokenKind::RightParen)?;

        let then_branch = Box::new(self.statement()?);
        let mut else_branch = None;

        if self.peek().map(|t| t.kind) == Some(TokenKind::Else) {
            self.advance().unwrap(); // Consume 'else'
            else_branch = if self.peek().map(|t| t.kind) == Some(TokenKind::Var) {
                self.advance().unwrap();
                let variable = self.consume(TokenKind::Identifier)?;
                self.consume(TokenKind::Equal)?;
                let initial = Some(self.expression()?);
                self.consume(TokenKind::Semi)?;
                Some(Box::new(Statement::Var {
                    name: variable.literal,
                    initializer: initial,
                }))
            } else {
                Some(Box::new(self.statement()?))
            };
        }

        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Result<Statement, ParserError> {
        self.advance().unwrap(); // Consume 'while'
        self.consume(TokenKind::LeftParen)?;
        let condition = self.expression()?;
        self.consume(TokenKind::RightParen)?;
        let body = Box::new(self.statement()?);

        Ok(Statement::While {
            condition: Box::new(condition),
            body,
        })
    }

    fn for_statement(&mut self) -> Result<Statement, ParserError> {
        self.advance().unwrap(); // Consume 'for'
        self.consume(TokenKind::LeftParen)?;

        let initialize = if self.peek().map(|t| t.kind) != Some(TokenKind::Semi) {
            Some(Box::new(
                if self.peek().map(|t| t.kind) == Some(TokenKind::Var) {
                    self.declaration()?
                } else {
                    self.expr_statement()?
                },
            ))
        } else {
            self.consume(TokenKind::Semi)?;
            None
        };

        let condition = if self.peek().map(|t| t.kind) != Some(TokenKind::Semi) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenKind::Semi)?;

        let increment = if self.peek().map(|t| t.kind) != Some(TokenKind::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenKind::RightParen)?;
        let body = if self.peek().map(|t| t.kind) == Some(TokenKind::Var) {
            self.advance().unwrap();
            let variable = self.consume(TokenKind::Identifier)?;
            self.consume(TokenKind::Equal)?;
            let initial = Some(self.expression()?);
            self.consume(TokenKind::Semi)?;
            Statement::Var {
                name: variable.literal,
                initializer: initial,
            }
        } else {
            self.statement()?
        };

        Ok(Statement::For {
            initialize,
            condition,
            increment,
            body: Box::new(body),
        })
    }

    pub fn parse(&mut self) -> Result<Expression, ParserError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expression, ParserError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression, ParserError> {
        let expr = self.or_expression()?;

        if self.peek().map(|t| t.kind) == Some(TokenKind::Equal) {
            let token = self.advance().unwrap();
            let value = self.assignment()?;

            if let Expression::Variable { name, resolved: _ } = expr {
                return Ok(Expression::Assign {
                    name,
                    value: Box::new(value),
                    resolved: None,
                });
            }

            return Err(ParserError::InvalidAssignmentTarget {
                line: token.line,
                token: token.literal,
            });
        }

        Ok(expr)
    }

    fn or_expression(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.and_expression()?;

        while self.peek().map(|t| t.kind) == Some(TokenKind::Or) {
            let operator = self.advance().unwrap();
            let right = self.and_expression()?;
            expr = Expression::Logical {
                left: Box::new(expr),
                operator: operator.kind,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and_expression(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.equality()?;

        while self.peek().map(|t| t.kind) == Some(TokenKind::And) {
            let operator = self.advance().unwrap();
            let right = self.equality()?;
            expr = Expression::Logical {
                left: Box::new(expr),
                operator: operator.kind,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.comparison()?;

        while let Some(kind) = self.peek().map(|t| t.kind) {
            match kind {
                TokenKind::EqualEqual | TokenKind::BangEqual => {
                    let operator = self.advance().unwrap();
                    let right = self.comparison()?;
                    expr = Expression::Binary {
                        left: Box::new(expr),
                        operator: operator.kind,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.term()?;

        while let Some(kind) = self.peek().map(|t| t.kind) {
            match kind {
                TokenKind::Greater
                | TokenKind::GreaterEqual
                | TokenKind::Less
                | TokenKind::LessEqual => {
                    let operator = self.advance().unwrap();
                    let right = self.term()?;
                    expr = Expression::Binary {
                        left: Box::new(expr),
                        operator: operator.kind,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.factor()?;

        while let Some(kind) = self.peek().map(|t| t.kind) {
            match kind {
                TokenKind::Plus | TokenKind::Minus => {
                    let operator = self.advance().unwrap();
                    let right = self.factor()?;
                    expr = Expression::Binary {
                        left: Box::new(expr),
                        operator: operator.kind,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.unary()?;

        while let Some(kind) = self.peek().map(|t| t.kind) {
            match kind {
                TokenKind::Star | TokenKind::Slash => {
                    let operator = self.advance().unwrap();
                    let right = self.unary()?;
                    expr = Expression::Binary {
                        left: Box::new(expr),
                        operator: operator.kind,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression, ParserError> {
        if let Some(kind) = self.peek().map(|t| t.kind) {
            if matches!(kind, TokenKind::Bang | TokenKind::Minus) {
                let operator = self.advance().unwrap();
                let expression = self.unary()?;
                return Ok(Expression::Unary {
                    operator: operator.kind,
                    expression: Box::new(expression),
                });
            }
        }
        self.call()
    }

    fn call(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.primary()?;

        loop {
            if self.peek().map(|t| t.kind) == Some(TokenKind::LeftParen) {
                self.advance().unwrap(); // Consume '('
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression, ParserError> {
        let mut args = Vec::new();
        let call_fn = match &callee {
            Expression::Variable { name, resolved: _ } => Some(name.clone()),
            _ => None,
        };
        if self.peek().map(|t| t.kind) != Some(TokenKind::RightParen) {
            loop {
                if args.len() >= 255 {
                    return Err(ParserError::Message(
                        "Cannot have more than 255 arguments.".to_string(),
                    ));
                }
                args.push(self.expression()?);
                if self.peek().map(|t| t.kind) != Some(TokenKind::Comma) {
                    break;
                }
                self.advance().unwrap(); // Consume ','
            }
        }

        self.consume(TokenKind::RightParen)?;

        if let Some(fn_name) = call_fn {
            if let Some(fun_args) = self.function_names.get(&fn_name) {
                if *fun_args > 0 && args.is_empty() {
                    return Err(ParserError::Message(format!(
                        "Expected {} arguments but got {}.",
                        args.len(),
                        *fun_args
                    )));
                }
            }
        }

        Ok(Expression::Call {
            callee: Box::new(callee),
            args,
        })
    }

    fn primary(&mut self) -> Result<Expression, ParserError> {
        let token = match self.advance() {
            Some(token) => token,
            None => return Err(ParserError::UnexpectedEof { line: 1 }),
        };

        match token.kind {
            TokenKind::Number(n) => Ok(Expression::Literal(crate::ast::Literal::Number(n))),
            TokenKind::String => Ok(Expression::Literal(crate::ast::Literal::String(
                token.literal,
            ))),
            TokenKind::True => Ok(Expression::Literal(crate::ast::Literal::Boolean(true))),
            TokenKind::False => Ok(Expression::Literal(crate::ast::Literal::Boolean(false))),
            TokenKind::Nil => Ok(Expression::Literal(crate::ast::Literal::Nil)),
            TokenKind::Identifier => Ok(Expression::Variable {
                name: token.literal,
                resolved: None,
            }),
            TokenKind::LeftParen => {
                let expression = self.expression()?;
                self.consume(TokenKind::RightParen)?;
                Ok(Expression::Group(Box::new(expression)))
            }
            _ => Err(ParserError::UnexpectedToken {
                line: token.line,
                token: token.literal,
            }),
        }
    }

    fn synchronize(&mut self) {
        while let Some(token) = self.peek() {
            if token.kind == TokenKind::Semi {
                self.advance();
                return;
            }

            if matches!(
                token.kind,
                TokenKind::Print
                    | TokenKind::Var
                    | TokenKind::LeftBrace
                    | TokenKind::If
                    | TokenKind::While
                    | TokenKind::For
                    | TokenKind::Return
                    | TokenKind::Fun
                    | TokenKind::Class
            ) {
                return;
            }

            if token.kind == TokenKind::RightBrace {
                return;
            }

            self.advance();
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn consume(&mut self, expected: TokenKind) -> Result<Token, ParserError> {
        match self.advance() {
            Some(token) if token.kind == expected => Ok(token),
            Some(token) => Err(ParserError::UnexpectedToken {
                line: token.line,
                token: token.literal,
            }),
            None => {
                let line = self.tokens.peek().map(|t| t.line).unwrap_or(1);
                Err(ParserError::UnexpectedEof { line })
            }
        }
    }

    fn advance(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    pub fn had_error(&self) -> bool {
        self.had_error
    }
}
