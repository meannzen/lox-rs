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

    pub fn expression(&mut self) -> Option<Expression> {
        self.primary()
    }

    fn peak(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn consume(&mut self, expected: TokenKind) -> Option<Token> {
        if let Some(token) = self.peak() {
            if token.kind == expected {
                return Some(self.advance().unwrap());
            }
        }
        None
    }

    fn advance(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn primary(&mut self) -> Option<Expression> {
        let token = self.advance()?;
        match token.kind {
            TokenKind::Number(n) => Some(Expression::Literal(crate::ast::Literal::Number(n))),
            TokenKind::Nil => Some(Expression::Literal(crate::ast::Literal::Nil)),
            TokenKind::False => Some(Expression::Literal(crate::ast::Literal::Boolean(false))),
            TokenKind::True => Some(Expression::Literal(crate::ast::Literal::Boolean(true))),
            TokenKind::String => Some(Expression::Literal(crate::ast::Literal::String(
                token.literal,
            ))),
            TokenKind::LeftParen => {
                let expression = self.expression()?;
                self.consume(TokenKind::RightParen)?;
                Some(Expression::Group(Box::new(expression)))
            }

            _ => None,
        }
    }
}
