use crate::Token;

#[derive(Debug)]
pub enum Error {
    UnexpectedToken(Token),
    UnexpectedEof,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnexpectedToken(token) => write!(f, "Unexpected token {:?}", token),
            Error::UnexpectedEof => write!(f, "Unexpected end of file"),
        }
    }
}

impl std::error::Error for Error {}
