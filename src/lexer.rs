use std::{iter::Peekable, str::Chars};

use crate::token::{Token, TokenKind};

pub struct Lexer<'c> {
    input: Peekable<Chars<'c>>,
    line: usize,
    column: usize,
}

impl<'c> Lexer<'c> {
    pub fn new(input: &'c str) -> Self {
        Lexer {
            input: input.chars().peekable(),
            line: 1,
            column: 1,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.input.next()?;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let start_line = self.line;
        let start_column = self.column;

        let ch = self.advance()?;

        let mut literal: String = ch.to_string();

        let kind = match ch {
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            '*' => TokenKind::Star,
            '.' => TokenKind::Dot,
            ',' => TokenKind::Comma,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            ';' => TokenKind::Semi,
            '/' => {
                if let Some(_) = self.input.next_if_eq(&'/') {
                    self.next_line();
                    return self.next_token();
                } else {
                    TokenKind::Slash
                }
            }
            '=' => {
                if let Some(next_ch) = self.input.next_if_eq(&'=') {
                    literal.push(next_ch);
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            }
            '!' => {
                if let Some(next_ch) = self.input.next_if_eq(&'=') {
                    literal.push(next_ch);
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                }
            }
            '<' => {
                if let Some(next_ch) = self.input.next_if_eq(&'=') {
                    literal.push(next_ch);
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }
            '>' => {
                if let Some(next_ch) = self.input.next_if_eq(&'=') {
                    literal.push(next_ch);
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }
            _ => TokenKind::Illegal,
        };

        Some(Token {
            kind,
            literal,
            line: start_line,
            column: start_column,
        })
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c.is_ascii_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn next_line(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c != '\n' {
                self.advance();
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
                literal: "(".to_string(),
                line: 1,
                column: 2,
            },
            Token {
                kind: TokenKind::LeftParen,
                literal: "(".to_string(),
                line: 1,
                column: 4,
            },
            Token {
                kind: TokenKind::RightParen,
                literal: ")".to_string(),
                line: 1,
                column: 6,
            },
            Token {
                kind: TokenKind::RightParen,
                literal: ")".to_string(),
                line: 1,
                column: 8,
            },
        ];

        let actual_tokens: Vec<Token> = lexer.collect();

        assert_eq!(actual_tokens, expected_tokens);
    }

    #[test]
    fn scanning_brace() {
        let input = " {{ }} ";
        let lexer = Lexer::new(input);

        let expected_tokens = vec![
            Token {
                kind: TokenKind::LeftBrace,
                literal: "{".to_string(),
                line: 1,
                column: 2,
            },
            Token {
                kind: TokenKind::LeftBrace,
                literal: "{".to_string(),
                line: 1,
                column: 3,
            },
            Token {
                kind: TokenKind::RightBrace,
                literal: "}".to_string(),
                line: 1,
                column: 5,
            },
            Token {
                kind: TokenKind::RightBrace,
                literal: "}".to_string(),
                line: 1,
                column: 6,
            },
        ];

        let actual_tokens: Vec<Token> = lexer.collect();

        assert_eq!(actual_tokens, expected_tokens);
    }

    #[test]
    fn scanning_symbols() {
        let input = "{*.,+*-/;})";
        let lexer = Lexer::new(input);

        let expected_tokens = vec![
            Token {
                kind: TokenKind::LeftBrace,
                literal: "{".to_string(),
                line: 1,
                column: 1,
            },
            Token {
                kind: TokenKind::Star,
                literal: "*".to_string(),
                line: 1,
                column: 2,
            },
            Token {
                kind: TokenKind::Dot,
                literal: ".".to_string(),
                line: 1,
                column: 3,
            },
            Token {
                kind: TokenKind::Comma,
                literal: ",".to_string(),
                line: 1,
                column: 4,
            },
            Token {
                kind: TokenKind::Plus,
                literal: "+".to_string(),
                line: 1,
                column: 5,
            },
            Token {
                kind: TokenKind::Star,
                literal: "*".to_string(),
                line: 1,
                column: 6,
            },
            Token {
                kind: TokenKind::Minus,
                literal: "-".to_string(),
                line: 1,
                column: 7,
            },
            Token {
                kind: TokenKind::Slash,
                literal: "/".to_string(),
                line: 1,
                column: 8,
            },
            Token {
                kind: TokenKind::Semi,
                literal: ";".to_string(),
                line: 1,
                column: 9,
            },
            Token {
                kind: TokenKind::RightBrace,
                literal: "}".to_string(),
                line: 1,
                column: 10,
            },
            Token {
                kind: TokenKind::RightParen,
                literal: ")".to_string(),
                line: 1,
                column: 11,
            },
        ];

        let actual_tokens: Vec<Token> = lexer.collect();

        assert_eq!(actual_tokens, expected_tokens);
    }

    #[test]
    fn scanning_with_newline() {
        let input = "{\n}";
        let lexer = Lexer::new(input);

        let expected_tokens = vec![
            Token {
                kind: TokenKind::LeftBrace,
                literal: "{".to_string(),
                line: 1,
                column: 1,
            },
            Token {
                kind: TokenKind::RightBrace,
                literal: "}".to_string(),
                line: 2,
                column: 1,
            },
        ];

        let actual_tokens: Vec<Token> = lexer.collect();

        assert_eq!(actual_tokens, expected_tokens);
    }

    #[test]
    fn scanning_equal_bang() {
        let input = "===!=!<<=>>=";
        let lexer = Lexer::new(input);

        let expected_tokens = vec![
            Token {
                kind: TokenKind::EqualEqual,
                literal: "==".to_string(),
                line: 1,
                column: 1,
            },
            Token {
                kind: TokenKind::Equal,
                literal: "=".to_string(),
                line: 1,
                column: 2,
            },
            Token {
                kind: TokenKind::BangEqual,
                literal: "!=".to_string(),
                line: 1,
                column: 3,
            },
            Token {
                kind: TokenKind::Bang,
                literal: "!".to_string(),
                line: 1,
                column: 4,
            },
            Token {
                kind: TokenKind::Less,
                literal: "<".to_string(),
                line: 1,
                column: 5,
            },
            Token {
                kind: TokenKind::LessEqual,
                literal: "<=".to_string(),
                line: 1,
                column: 6,
            },
            Token {
                kind: TokenKind::Greater,
                literal: ">".to_string(),
                line: 1,
                column: 7,
            },
            Token {
                kind: TokenKind::GreaterEqual,
                literal: ">=".to_string(),
                line: 1,
                column: 8,
            },
        ];

        let actual_tokens: Vec<Token> = lexer.collect();

        assert_eq!(actual_tokens, expected_tokens);
    }
}
