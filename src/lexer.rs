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

    fn next_char(&mut self) -> Option<char> {
        todo!()
    }

    fn next_token(&mut self) -> Option<Token> {
        Some(Token {
            kind: TokenKind::Eof,
        })
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
