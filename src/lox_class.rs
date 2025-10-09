use std::rc::Rc;

use crate::Callable;

#[derive(Debug, Clone)]
pub struct LoxClass {
    pub name: String,
}

impl Callable for LoxClass {
    fn call(
        &self,
        _interpreter: &mut crate::Interpreter,
        _args: Vec<crate::Value>,
    ) -> Result<crate::Value, crate::InterpreterError> {
        let class_rc = Rc::new(self.clone());
        Ok(crate::Value::Instance(crate::LoxInstance::new(class_rc)))
    }

    fn arity(&self) -> usize {
        0
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
