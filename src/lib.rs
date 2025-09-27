pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

mod ast;
mod function;
mod function_trait;
mod interpreter;
mod parser;
mod tokenizer;
mod visit;

pub use ast::*;
pub use function::*;
pub use function_trait::*;
pub use interpreter::*;
pub use parser::*;
pub use tokenizer::*;
pub use visit::*;
