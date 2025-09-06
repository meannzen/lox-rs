#[derive(Debug)]
pub enum ParserError {
    UnexpectedEof { line: usize },
    UnexpectedToken { line: usize, token: String },
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedEof { line } => {
                write!(f, "[line {}] Error: Unexpected EOF ", line)
            }
            ParserError::UnexpectedToken { line, token } => {
                write!(f, "[line {line}] Error at '{}': Expect expression.", token)
            }
        }
    }
}

impl std::error::Error for ParserError {}
