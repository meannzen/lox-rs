use std::iter::Peekable;

use crate::{ast::Expression, Lexer, Token, TokenKind};

pub struct Parser<'input> {
    tokens: Peekable<Lexer<'input>>,
}

impl<'input> Parser<'input> {
    pub fn new(input: &'input str) -> Self {
        Parser {
            tokens: Lexer::new(input).peekable(),
        }
    }

    pub fn parse(&mut self) -> Option<Expression> {
        self.expression()
    }

    fn expression(&mut self) -> Option<Expression> {
        self.equality()
    }

    fn equality(&mut self) -> Option<Expression> {
        let mut expr = self.comparison()?;
        while let Some(token) = self.peek() {
            match token.kind {
                TokenKind::EqualEqual | TokenKind::BangEqual => {
                    let operator = self.advance()?;
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
        Some(expr)
    }

    fn comparison(&mut self) -> Option<Expression> {
        let mut expr = self.term()?;
        while let Some(token) = self.peek() {
            match token.kind {
                TokenKind::Greater
                | TokenKind::GreaterEqual
                | TokenKind::Less
                | TokenKind::LessEqual => {
                    let operator = self.advance()?;
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
        Some(expr)
    }

    fn term(&mut self) -> Option<Expression> {
        let mut expr = self.factor()?;
        while let Some(token) = self.peek() {
            match token.kind {
                TokenKind::Plus | TokenKind::Minus => {
                    let operator = self.advance()?;
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
        Some(expr)
    }

    fn factor(&mut self) -> Option<Expression> {
        let mut expr = self.unary()?;
        while let Some(token) = self.peek() {
            match token.kind {
                TokenKind::Star | TokenKind::Slash => {
                    let operator = self.advance()?;
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
        Some(expr)
    }

    fn unary(&mut self) -> Option<Expression> {
        if let Some(token) = self.peek() {
            if matches!(token.kind, TokenKind::Bang | TokenKind::Minus) {
                let operator = self.advance()?;
                let expression = self.unary()?;
                return Some(Expression::Unary {
                    operator: operator.kind,
                    expression: Box::new(expression),
                });
            }
        }
        self.primary()
    }

    fn primary(&mut self) -> Option<Expression> {
        let token = self.advance()?;
        match token.kind {
            TokenKind::Number(n) => Some(Expression::Literal(crate::ast::Literal::Number(n))),
            TokenKind::String => Some(Expression::Literal(crate::ast::Literal::String(
                token.literal,
            ))),
            TokenKind::True => Some(Expression::Literal(crate::ast::Literal::Boolean(true))),
            TokenKind::False => Some(Expression::Literal(crate::ast::Literal::Boolean(false))),
            TokenKind::Nil => Some(Expression::Literal(crate::ast::Literal::Nil)),
            TokenKind::LeftParen => {
                let expression = self.expression()?;
                self.consume(TokenKind::RightParen)?;
                Some(Expression::Group(Box::new(expression)))
            }
            _ => None,
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn consume(&mut self, expected: TokenKind) -> Option<Token> {
        if let Some(token) = self.peek() {
            if token.kind == expected {
                return self.advance();
            }
        }
        None
    }

    fn advance(&mut self) -> Option<Token> {
        self.tokens.next()
    }
}
