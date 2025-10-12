use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{BoundMethod, InterpreterError, LoxClass, LoxFunction, Value};

#[derive(Debug, Clone)]
pub struct LoxInstance {
    class: Rc<LoxClass>,
    fields: Rc<RefCell<HashMap<String, Value>>>,
}

impl LoxInstance {
    pub fn new(class: Rc<LoxClass>) -> Self {
        Self {
            class,
            fields: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn get(&self, name: &str) -> Result<Value, InterpreterError> {
        if let Some(value) = self.fields.borrow().get(name).cloned() {
            return Ok(value);
        }

        if let Some(method) = self.find_method(name) {
            return Ok(Value::Function(Rc::new(BoundMethod {
                function: Rc::new(method),
                instance: Rc::new(self.clone()),
            })));
        }

        Err(InterpreterError::Message(
            format!("Undefined property '{}'.", name),
            crate::ExitCode::RunTimeError,
        ))
    }

    pub fn set(&self, name: &str, value: Value) {
        self.fields.borrow_mut().insert(name.to_string(), value);
    }

    pub fn name(&self) -> String {
        format!("{} instance", self.class.name.clone())
    }

    fn find_method(&self, name: &str) -> Option<LoxFunction> {
        self.class.methods.borrow().get(name).cloned()
    }
}
