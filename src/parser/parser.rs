use std::iter::Peekable;

use crate::{ast::Expression, Lexer, ParserError, Statement, Token, TokenKind};

pub struct Parser<'input> {
    tokens: Peekable<Lexer<'input>>,
}

impl<'input> Parser<'input> {
    pub fn new(input: &'input str) -> Self {
        Parser {
            tokens: Lexer::new(input).peekable(),
        }
    }

    pub fn parse_statements(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut statements = Vec::new();
        while self.peek().is_some() {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Statement, ParserError> {
        if let Some(token) = self.peek() {
            match token.kind {
                TokenKind::Print => self.statement_expr(),
                _ => unimplemented!("I'm not implement yet, please don't panic"),
            }
        } else {
            Err(ParserError::UnexpectedEof { line: 1 })
        }
    }

    fn statement_expr(&mut self) -> Result<Statement, ParserError> {
        self.advance().unwrap();
        let expr = self.expression()?;
        self.consume(TokenKind::Semi)?;
        Ok(Statement::Expr(expr))
    }

    pub fn parse(&mut self) -> Result<Expression, ParserError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expression, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.comparison()?;
        while let Some(token) = self.peek() {
            match token.kind {
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
        while let Some(token) = self.peek() {
            match token.kind {
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
        while let Some(token) = self.peek() {
            match token.kind {
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
        while let Some(token) = self.peek() {
            match token.kind {
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
        if let Some(token) = self.peek() {
            if matches!(token.kind, TokenKind::Bang | TokenKind::Minus) {
                let operator = self.advance().unwrap();
                let expression = self.unary()?;
                return Ok(Expression::Unary {
                    operator: operator.kind,
                    expression: Box::new(expression),
                });
            }
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expression, ParserError> {
        let token = match self.advance() {
            Some(token) => token,
            _ => return Err(ParserError::UnexpectedEof { line: 1 }),
        };
        match token.kind {
            TokenKind::Number(n) => Ok(Expression::Literal(crate::ast::Literal::Number(n))),
            TokenKind::String => Ok(Expression::Literal(crate::ast::Literal::String(
                token.literal,
            ))),
            TokenKind::True => Ok(Expression::Literal(crate::ast::Literal::Boolean(true))),
            TokenKind::False => Ok(Expression::Literal(crate::ast::Literal::Boolean(false))),
            TokenKind::Nil => Ok(Expression::Literal(crate::ast::Literal::Nil)),
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

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn consume(&mut self, expected: TokenKind) -> Result<Token, ParserError> {
        let mut line: usize = 1;
        if let Some(token) = self.peek() {
            line = token.line;
            if token.kind == expected {
                return Ok(self.advance().unwrap());
            }
        }
        Err(ParserError::UnexpectedEof { line })
    }

    fn advance(&mut self) -> Option<Token> {
        self.tokens.next()
    }
}
