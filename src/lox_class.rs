use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{Callable, LoxFunction};

#[derive(Debug, Clone)]
pub struct LoxClass {
    pub name: String,
    pub methods: Rc<RefCell<HashMap<String, LoxFunction>>>,
}

impl LoxClass {
    pub fn new(name: String) -> Self {
        LoxClass {
            name,
            methods: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn create_method(&self, name: String, method: LoxFunction) {
        self.methods.borrow_mut().insert(name, method);
    }
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
