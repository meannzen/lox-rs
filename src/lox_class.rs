use crate::Callable;

#[derive(Debug)]
pub struct LoxClass {
    pub name: String,
}

impl Callable for LoxClass {
    fn call(
        &self,
        _interpreter: &mut crate::Interpreter,
        _args: Vec<crate::Value>,
    ) -> Result<crate::Value, crate::InterpreterError> {
        Ok(crate::Value::Instance(crate::LoxInstance::new(
            self.name.clone(),
            1,
        )))
    }

    fn arity(&self) -> usize {
        0
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
