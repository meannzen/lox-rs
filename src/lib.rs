pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

mod ast;
mod interpreter;
mod lexer;
mod parser;
mod token;
mod visit;

pub use ast::*;
pub use interpreter::*;
pub use lexer::Lexer;
pub use parser::Parser;
pub use parser::ParserError;
pub use token::*;
pub use visit::*;
