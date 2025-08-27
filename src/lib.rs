pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

mod lexer;
mod token;

pub use lexer::Lexer;
pub use token::*;
