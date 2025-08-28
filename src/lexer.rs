use std::{iter::Peekable, str::Chars};

use crate::token::{Token, TokenKind};

pub struct Lexer<'c> {
    input: Peekable<Chars<'c>>,
}

impl<'c> Lexer<'c> {
    pub fn new(input: &'c str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let ch = self.input.next()?;

        let kind = match ch {
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            _ => unimplemented!(),
        };

        Some(Token { kind })
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c.is_ascii_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Lexer, Token, TokenKind};

    #[test]
    fn empty() {
        let mut lexer = Lexer::new("");
        let token = lexer.next();
        assert_eq!(token, None);
    }

    #[test]
    fn scanning_parentheses() {
        let input = " ( ( ) ) ";
        let lexer = Lexer::new(input);

        let expected_tokens = vec![
            Token {
                kind: TokenKind::LeftParen,
            },
            Token {
                kind: TokenKind::LeftParen,
            },
            Token {
                kind: TokenKind::RightParen,
            },
            Token {
                kind: TokenKind::RightParen,
            },
        ];

        let actual_tokens: Vec<Token> = lexer.collect();

        assert_eq!(actual_tokens, expected_tokens);
    }
}
