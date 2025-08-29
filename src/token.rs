#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
    Semi,
    Slash,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Illegal,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            TokenKind::LeftParen => write!(f, "LEFT_PAREN ( null"),
            TokenKind::RightParen => write!(f, "RIGHT_PAREN ) null"),
            TokenKind::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            TokenKind::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            TokenKind::Star => write!(f, "STAR * null"),
            TokenKind::Dot => write!(f, "DOT . null"),
            TokenKind::Comma => write!(f, "COMMA , null"),
            TokenKind::Plus => write!(f, "PLUS + null"),
            TokenKind::Minus => write!(f, "MINUS - null"),
            TokenKind::Semi => write!(f, "SEMICOLON ; null"),
            TokenKind::Slash => write!(f, "SLASH / null"),
            TokenKind::Equal => write!(f, "EQUAL = null"),
            TokenKind::EqualEqual => write!(f, "EQUAL_EQUAL == null"),
            TokenKind::Bang => write!(f, "BANG ! null"),
            TokenKind::BangEqual => write!(f, "BANG_EQUAL != null"),
            TokenKind::Less => write!(f, "LESS < null"),
            TokenKind::LessEqual => write!(f, "LESS_EQUAL <= null"),
            TokenKind::Greater => write!(f, "GREATER > null"),
            TokenKind::GreaterEqual => write!(f, "GREATER_EQUAL >= null"),
            TokenKind::Illegal => write!(
                f,
                "[line {}] Error: Unexpected character: {}",
                self.line, self.literal
            ),
        }
    }
}
