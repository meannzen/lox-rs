use crate::Callable;

#[derive(Debug)]
pub struct LoxClass {
    name: String,
}

impl Callable for LoxClass {
    fn call(
        &self,
        interpreter: &mut crate::Interpreter,
        args: Vec<crate::Value>,
    ) -> Result<crate::Value, crate::InterpreterError> {
        todo!()
    }

    fn arity(&self) -> usize {
        return 0;
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

