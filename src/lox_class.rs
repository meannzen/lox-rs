use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{BoundMethod, Callable, LoxFunction, LoxInstance, Value};

#[derive(Debug, Clone)]
pub struct LoxClass {
    pub name: String,
    pub superclass: Option<Rc<LoxClass>>,
    pub methods: Rc<RefCell<HashMap<String, LoxFunction>>>,
}

impl LoxClass {
    pub fn new(name: String, superclass: Option<Rc<LoxClass>>) -> Self {
        LoxClass {
            name,
            superclass,
            methods: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn create_method(&self, name: String, method: LoxFunction) {
        self.methods.borrow_mut().insert(name, method);
    }

    pub fn find_method(&self, name: &str) -> Option<LoxFunction> {
        if let Some(m) = self.methods.borrow().get(name) {
            return Some(m.clone());
        }
        self.superclass.as_ref().and_then(|s| s.find_method(name))
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl Callable for LoxClass {
    fn call(
        &self,
        interpreter: &mut crate::Interpreter,
        args: Vec<crate::Value>,
    ) -> Result<crate::Value, crate::InterpreterError> {
        let instance = LoxInstance::new(Rc::new(self.clone()));
        let instance_rc = Rc::new(instance);
        if let Some(initializer) = self.find_method("init") {
            let bound = BoundMethod {
                function: Rc::new(initializer),
                instance: instance_rc.clone(),
            };
            let bound_rc: Rc<dyn Callable> = Rc::new(bound);
            bound_rc.call(interpreter, args)?;
        }

        Ok(Value::Instance(instance_rc))
    }

    fn arity(&self) -> usize {
        self.find_method("init").map_or(0, |init| init.arity())
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
