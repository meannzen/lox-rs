use crate::{Callable, InterpreterError, Value};

#[derive(Debug, Clone)]
pub struct NativeFunction {
    pub arity: usize,
    pub name: String,
    pub function: fn(Vec<Value>) -> Result<Value, InterpreterError>,
}

impl Callable for NativeFunction {
    fn call(
        &self,
        _interpreter: &mut crate::Interpreter,
        args: Vec<Value>,
    ) -> Result<Value, InterpreterError> {
        (self.function)(args)
    }

    fn arity(&self) -> usize {
        self.arity
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
