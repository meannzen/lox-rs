#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
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
    // add more token
    Unknown,
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Token { kind }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            TokenKind::LeftParen => write!(f, "LEFT_PAREN ( null"),
            TokenKind::RightParen => write!(f, "RIGHT_PAREN ) null"),
            TokenKind::LeftBrace => write!(f, "LEFT_BRACE {} null", "{"),
            TokenKind::RightBrace => write!(f, "RIGHT_BRACE {} null", "}"),
            TokenKind::Star => write!(f, "STAR * null"),
            TokenKind::Dot => write!(f, "DOT . null"),
            TokenKind::Comma => write!(f, "COMMA , null"),
            TokenKind::Plus => write!(f, "PLUS + null"),
            TokenKind::Minus => write!(f, "MINUS - null"),
            TokenKind::Semi => write!(f, "SEMICOLON ; null"),
            TokenKind::Slash => write!(f, "SLASH / null"),
            _ => unimplemented!(),
        }
    }
}
