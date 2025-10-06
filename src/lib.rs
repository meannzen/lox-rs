pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

mod ast;
mod function;
mod function_trait;
mod interpreter;
mod lox_class;
mod parser;
mod resolver;
mod tokenizer;
mod visit;

pub use ast::*;
pub use function::*;
pub use function_trait::*;
pub use interpreter::*;
pub use lox_class::*;
pub use parser::*;
pub use resolver::*;
pub use tokenizer::*;
pub use visit::*;
