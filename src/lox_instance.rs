use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{InterpreterError, LoxClass, Value};

#[derive(Debug, Clone)]
pub struct LoxInstance {
    class: Rc<LoxClass>, // Assuming LoxClass is Rc-able
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
        self.fields.borrow().get(name).cloned().ok_or_else(|| {
            InterpreterError::Message(
                format!("Undefined property '{}'.", name),
                crate::ExitCode::RunTimeError,
            )
        })
    }

    pub fn set(&self, name: &str, value: Value) {
        self.fields.borrow_mut().insert(name.to_string(), value);
    }

    pub fn name(&self) -> String {
        format!("{} instance", self.class.name.clone())
    }
}

