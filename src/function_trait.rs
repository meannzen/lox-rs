use crate::{Interpreter, InterpreterError, Value};

pub trait Callable: std::fmt::Debug {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Value>,
    ) -> Result<Value, InterpreterError>;

    fn arity(&self) -> usize;

    fn name(&self) -> String;
}
