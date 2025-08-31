#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq, Clone)]
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
    String,
    Number(f64),
    Identifier,
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Illegal(IlligalType),
}

#[derive(Debug, PartialEq, Clone)]
pub enum IlligalType {
    Unexpected,
    UnterminatedString,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
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
            TokenKind::String => write!(f, "STRING \"{}\" {}", self.literal, self.literal),
            TokenKind::Number(num) => {
                if num.fract() == 0.0 {
                    write!(f, "NUMBER {} {:.1}", self.literal, num)
                } else {
                    write!(f, "NUMBER {} {num}", self.literal)
                }
            }
            TokenKind::Identifier => write!(f, "IDENTIFIER {} null", self.literal),
            TokenKind::And => write!(f, "AND {} null", self.literal),
            TokenKind::Class => write!(f, "CLASS {} null", self.literal),
            TokenKind::Else => write!(f, "ELSE {} null", self.literal),
            TokenKind::For => write!(f, "FOR {} null", self.literal),
            TokenKind::Fun => write!(f, "FUN {} null", self.literal),
            TokenKind::If => write!(f, "IF {} null", self.literal),
            TokenKind::False => write!(f, "FALSE {} null", self.literal),
            TokenKind::Nil => write!(f, "NIL {} null", self.literal),
            TokenKind::Or => write!(f, "OR {} null", self.literal),
            TokenKind::Print => write!(f, "PRINT {} null", self.literal),
            TokenKind::Return => write!(f, "RETURN {} null", self.literal),
            TokenKind::This => write!(f, "THIS {} null", self.literal),
            TokenKind::True => write!(f, "TRUE {} null", self.literal),
            TokenKind::Var => write!(f, "VAR {} null", self.literal),
            TokenKind::While => write!(f, "WHILE {} null", self.literal),
            TokenKind::Super => write!(f, "SUPER {} null", self.literal),
            TokenKind::Illegal(ty) => {
                let word = match ty {
                    IlligalType::UnterminatedString => "Unterminated string .".to_string(),
                    IlligalType::Unexpected => format!("Unexpected character: {}", self.literal),
                };

                write!(f, "[line {}] Error: {}", self.line, word)
            }
        }
    }
}
