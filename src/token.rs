#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
}

#[derive(Debug)]
pub enum TokenKind {
    Eof,
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Token { kind }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            TokenKind::Eof => write!(f, "EOF  null"),
        }
    }
}
