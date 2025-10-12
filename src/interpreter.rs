use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    Callable, Expression, Literal, LoxClass, LoxInstance, NativeFunction, Resolver, Statement,
    TokenKind, Visitor,
};

#[derive(Debug)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Nil,
    String(String),
    Function(Rc<dyn Callable>),
    Class(Rc<LoxClass>),
    Instance(Rc<LoxInstance>),
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Self::Number(n) => Self::Number(*n),
            Self::String(s) => Self::String(s.clone()),
            Self::Nil => Self::Nil,
            Self::Boolean(b) => Self::Boolean(*b),
            Self::Function(f) => Self::Function(f.clone()),
            Self::Class(class) => Self::Class(class.clone()),
            Self::Instance(instance) => Self::Instance(instance.clone()),
        }
    }
}

#[derive(Debug)]
pub enum ExitCode {
    RunTimeError,
    CompilerError,
}

impl From<ExitCode> for i32 {
    fn from(value: ExitCode) -> Self {
        match value {
            ExitCode::CompilerError => 65,
            ExitCode::RunTimeError => 70,
        }
    }
}

#[derive(Debug)]
pub enum InterpreterError {
    Message(String, ExitCode),
    UndefinedVariable(String),
    ReturnError(Value),
}

#[derive(Debug, Clone)]
struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct LoxFunction {
    name: String,
    params: Vec<String>,
    body: Vec<Statement>,
    environment: Rc<RefCell<Environment>>,
    is_initializer: bool,
}

impl Callable for LoxFunction {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Value>,
    ) -> Result<Value, InterpreterError> {
        let old_env = interpreter.environment.clone();
        let new_env = Environment::new_enclosed(&self.environment.clone());

        for (name, value) in self.params.iter().zip(args.iter()) {
            new_env.borrow_mut().define(name.as_str(), value.clone());
        }

        interpreter.environment = new_env;
        let result = interpreter.visit_block(&self.body);
        interpreter.environment = old_env;

        match result {
            Ok(_) => Ok(Value::Nil),
            Err(InterpreterError::ReturnError(v)) => Ok(v),
            Err(e) => Err(e),
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn arity(&self) -> usize {
        self.params.len()
    }
}

#[derive(Debug, Clone)]
pub struct BoundMethod {
    pub function: Rc<LoxFunction>,
    pub instance: Rc<LoxInstance>,
}

impl Callable for BoundMethod {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Value>,
    ) -> Result<Value, InterpreterError> {
        let old_env = interpreter.environment.clone();
        let new_env = Environment::new_enclosed(&self.function.environment);

        for (name, value) in self.function.params.iter().zip(args.iter()) {
            new_env.borrow_mut().define(name.as_str(), value.clone());
        }

        new_env
            .borrow_mut()
            .define("this", Value::Instance(self.instance.clone()));
        interpreter.environment = new_env;
        let result = interpreter.visit_block(&self.function.body);
        interpreter.environment = old_env;

        match result {
            Ok(_) => {
                if self.function.is_initializer {
                    Ok(Value::Instance(self.instance.clone()))
                } else {
                    Ok(Value::Nil)
                }
            }
            Err(InterpreterError::ReturnError(v)) => {
                if self.function.is_initializer {
                    Ok(Value::Instance(self.instance.clone()))
                } else {
                    Ok(v)
                }
            }
            Err(e) => Err(e),
        }
    }

    fn arity(&self) -> usize {
        self.function.arity()
    }

    fn name(&self) -> String {
        format!(
            "<bound method {} of {}>",
            self.function.name,
            self.instance.name()
        )
    }
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn new_enclosed(enclosing: &Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment {
            enclosing: Some(Rc::clone(enclosing)),
            values: HashMap::new(),
        }))
    }

    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.values.get(name) {
            Some(value.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow().get(name)
        } else {
            None
        }
    }

    pub fn assign(&mut self, name: &str, value: Value) -> bool {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            true
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            false
        }
    }
}

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
    pub locals: HashMap<String, usize>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let global = Rc::new(RefCell::new(Environment::new()));
        global.borrow_mut().define(
            "clock",
            Value::Function(Rc::new(NativeFunction {
                name: "clock".to_string(),
                arity: 0,
                function: |_| {
                    let start_time = SystemTime::now();
                    let since_the_epoch = start_time
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards");
                    Ok(Value::Number(since_the_epoch.as_secs_f64()))
                },
            })),
        );
        Interpreter {
            environment: global,
            locals: HashMap::new(),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Visitor<Value, InterpreterError> for Interpreter {
    fn visit_expr(&mut self, expr: &crate::Expression) -> Result<Value, InterpreterError> {
        self.evaluate(expr)
    }

    fn visit_logical(
        &mut self,
        left: &Expression,
        operator: &TokenKind,
        right: &Expression,
    ) -> Result<Value, InterpreterError> {
        let left_value = self.evaluate(left)?;

        if *operator == TokenKind::Or {
            if is_truthy(&left_value) {
                return Ok(left_value);
            }
        } else if *operator == TokenKind::And && !is_truthy(&left_value) {
            return Ok(left_value);
        }

        self.evaluate(right)
    }

    fn visit_while(
        &mut self,
        condition: &Expression,
        body: &Statement,
    ) -> Result<(), InterpreterError> {
        while is_truthy(&self.evaluate(condition)?) {
            self.visit_stmt(body)?;
        }

        Ok(())
    }

    fn visit_stmt(&mut self, stms: &Statement) -> Result<(), InterpreterError> {
        match stms {
            Statement::Print(expr) => {
                let eval = self.evaluate(expr)?;
                println!("{eval}");
            }
            Statement::Expr(expr) => {
                let _result = self.visit_expr(expr)?;
            }

            Statement::Var { name, initializer } => {
                let value = if let Some(expr) = initializer {
                    self.visit_expr(expr)?
                } else {
                    Value::Nil
                };

                self.environment.borrow_mut().define(name.as_str(), value);
            }

            Statement::Block(list) => {
                let new_env = Environment::new_enclosed(&self.environment);
                let old_env = self.environment.clone();
                self.environment = new_env;
                self.visit_block(list)?;

                self.environment = old_env;
            }

            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.visit_if_stms(condition, then_branch, else_branch)?;
            }

            Statement::While { condition, body } => self.visit_while(condition, body)?,
            Statement::For {
                initialize,
                condition,
                increment,
                body,
            } => {
                let previous = self.environment.clone();
                let loop_env = Environment::new_enclosed(&previous);
                self.environment = loop_env;

                if let Some(init) = initialize {
                    self.visit_stmt(init)?;
                }

                loop {
                    if let Some(con) = condition {
                        if !is_truthy(&self.evaluate(con)?) {
                            break;
                        }
                    }

                    self.visit_stmt(body)?;

                    if let Some(inc) = increment {
                        self.evaluate(inc)?;
                    }
                }

                self.environment = previous;
            }

            Statement::Function { name, params, body } => {
                self.visit_function_stms(name, params, body)
            }

            Statement::Return { value } => self.visit_return_stms(value)?,

            Statement::Class {
                name,
                superclass,
                methods,
            } => self.visit_class(name.as_str(), superclass.as_deref(), methods)?,
        }

        Ok(())
    }
    fn visit_block(&mut self, list: &[Statement]) -> Result<(), InterpreterError> {
        for s in list.iter() {
            self.visit_stmt(s)?;
        }
        Ok(())
    }

    fn visit_if_stms(
        &mut self,
        condition: &Expression,
        then_branch: &Statement,
        else_branch: &Option<Box<Statement>>,
    ) -> Result<(), InterpreterError> {
        if is_truthy(&self.evaluate(condition)?) {
            self.visit_stmt(then_branch)?;
        } else if let Some(stms) = else_branch {
            self.visit_stmt(stms)?;
        }

        Ok(())
    }

    fn visit_call_expr(
        &mut self,
        callee: &Expression,
        args: &[Expression],
    ) -> Result<Value, InterpreterError> {
        let callee_value = self.evaluate(callee)?;
        if let Value::Function(function) = callee_value {
            if function.arity() != args.len() {
                return Err(InterpreterError::Message(
                    format!(
                        "Expected {} arguments but got {}.",
                        function.arity(),
                        args.len(),
                    ),
                    ExitCode::RunTimeError,
                ));
            }

            let mut arg_values = Vec::new();
            for arg_expr in args {
                arg_values.push(self.evaluate(arg_expr)?);
            }
            function.call(self, arg_values)
        } else if let Value::Class(class) = callee_value {
            let mut arg_values = Vec::new();
            for arg_expr in args {
                arg_values.push(self.evaluate(arg_expr)?);
            }
            class.call(self, arg_values)
        } else {
            Err(InterpreterError::Message(
                "Can only call functions and classes.".to_string(),
                ExitCode::RunTimeError,
            ))
        }
    }

    fn visit_function_stms(&mut self, name: &str, params: &[String], body: &[Statement]) {
        let function = LoxFunction {
            name: name.to_string(),
            params: params.to_vec(),
            body: body.to_vec(),
            environment: self.environment.clone(),
            is_initializer: false,
        };

        self.environment
            .borrow_mut()
            .define(name, Value::Function(Rc::new(function)));
    }

    fn visit_return_stms(&mut self, expr: &Option<Expression>) -> Result<(), InterpreterError> {
        let return_value = if let Some(value) = expr {
            self.evaluate(value)?
        } else {
            Value::Nil
        };

        Err(InterpreterError::ReturnError(return_value))
    }

    fn visit_class(
        &mut self,
        name: &str,
        superclass: Option<&str>,
        methods: &[Statement],
    ) -> Result<(), InterpreterError> {
        self.environment.borrow_mut().define(name, Value::Nil);

        let superclass_value = if let Some(super_name) = superclass {
            Some(
                self.environment
                    .borrow()
                    .get(super_name)
                    .and_then(|v| match v {
                        Value::Class(c) => Some(c.clone()),
                        _ => None,
                    })
                    .ok_or_else(|| {
                        InterpreterError::Message(
                            format!("Undefined superclass '{}'.", super_name),
                            ExitCode::RunTimeError,
                        )
                    })?,
            )
        } else {
            None
        };

        let closure = Environment::new_enclosed(&self.environment);
        if let Some(sclass) = &superclass_value {
            closure
                .borrow_mut()
                .define("super", Value::Class(sclass.clone()));
        }
        let closure_rc = Rc::new(RefCell::new(closure));

        let class = LoxClass::new(name.to_string(), superclass_value);
        for method in methods {
            match method {
                Statement::Function {
                    name: method_name,
                    params,
                    body,
                } => {
                    let function = LoxFunction {
                        name: method_name.clone(),
                        params: params.clone(),
                        body: body.clone(),
                        environment: closure_rc.borrow().clone(),
                        is_initializer: method_name == "init",
                    };

                    class.create_method(method_name.clone(), function);
                }
                _ => unreachable!(),
            }
        }

        let value = Value::Class(Rc::new(class));
        self.environment.borrow_mut().assign(name, value);

        Ok(())
    }

    fn visit_get_expr(
        &mut self,
        expr: &Expression,
        name: String,
    ) -> Result<Value, InterpreterError> {
        let value = self.evaluate(expr)?;
        match value {
            Value::Instance(instance) => instance.get(&name),
            _ => Err(InterpreterError::Message(
                "Only instances have properties.".to_string(),
                ExitCode::RunTimeError,
            )),
        }
    }

    fn visit_set_expr(
        &mut self,
        expr: &Expression,
        name: String,
        value: &Expression,
    ) -> Result<Value, InterpreterError> {
        let val = self.evaluate(expr)?;
        match val {
            Value::Instance(instance) => {
                let value = self.evaluate(value)?;
                instance.set(&name, value.clone());
                Ok(value)
            }
            _ => Err(InterpreterError::Message(
                "Only instances have fields.".to_string(),
                ExitCode::RunTimeError,
            )),
        }
    }
}

impl Interpreter {
    pub fn run(mut stmt: Vec<Statement>) -> Result<(), InterpreterError> {
        let interpreter = Interpreter::new();
        let mut resolver = Resolver::new(interpreter);
        if let Err(e) = resolver.resolve_stmts(&mut stmt[..]) {
            return Err(InterpreterError::Message(
                format!("Resolution error: {}", e),
                ExitCode::CompilerError,
            ));
        }

        let mut interpreter = resolver.interpreter;

        for st in stmt.iter() {
            interpreter.visit_stmt(st)?;
        }

        Ok(())
    }

    pub fn resolve(&mut self, name: &str, distance: usize) {
        self.locals.insert(name.to_string(), distance);
    }

    fn get_at(
        &self,
        environment: Rc<RefCell<Environment>>,
        distance: usize,
        name: &str,
    ) -> Option<Value> {
        let mut current_env = environment;
        for _ in 0..distance {
            let next_env = {
                let env_ref = current_env.borrow();

                if let Some(enclosing) = &env_ref.enclosing {
                    Rc::clone(enclosing)
                } else {
                    return None;
                }
            };
            current_env = next_env;
        }

        let value = current_env.borrow().values.get(name).cloned();
        value
    }

    fn assign_at(
        &mut self,
        environment: Rc<RefCell<Environment>>,
        distance: usize,
        name: &str,
        value: Value,
    ) {
        let mut current_env = environment;
        for _ in 0..distance {
            let next_env = {
                let env_ref = current_env.borrow();

                if let Some(enclosing) = &env_ref.enclosing {
                    Rc::clone(enclosing)
                } else {
                    return;
                }
            };
            current_env = next_env;
        }

        current_env
            .borrow_mut()
            .values
            .insert(name.to_string(), value);
    }

    pub fn evaluate(&mut self, expr: &Expression) -> Result<Value, InterpreterError> {
        match expr {
            Expression::Literal(literal) => self.visit_literal_expr(literal),
            Expression::Unary {
                operator,
                expression,
            } => self.visit_unary_expr(expression, operator),
            Expression::Group(inner_expr) => self.evaluate(inner_expr),
            Expression::Variable { name, resolved } => {
                if let Some(distance) = *resolved {
                    self.get_at(self.environment.clone(), distance, name.as_str())
                        .ok_or_else(|| InterpreterError::UndefinedVariable(name.clone()))
                } else {
                    self.environment
                        .borrow()
                        .get(name.as_str())
                        .ok_or_else(|| InterpreterError::UndefinedVariable(name.clone()))
                }
            }
            Expression::Assign {
                name,
                value,
                resolved,
            } => {
                let new_value = self.evaluate(value)?;
                if let Some(distance) = *resolved {
                    self.assign_at(
                        self.environment.clone(),
                        distance,
                        name.as_str(),
                        new_value.clone(),
                    );
                    Ok(new_value)
                } else {
                    let mut env = self.environment.borrow_mut();
                    if env.assign(name.as_str(), new_value.clone()) {
                        Ok(new_value)
                    } else {
                        Err(InterpreterError::UndefinedVariable(name.clone()))
                    }
                }
            }
            Expression::Logical {
                left,
                operator,
                right,
            } => self.visit_logical(left, operator, right),
            Expression::Call { callee, args } => self.visit_call_expr(callee, args),
            Expression::Get { object, name } => self.visit_get_expr(object, name.clone()),
            Expression::Set {
                object,
                property,
                value,
            } => self.visit_set_expr(object, property.clone(), value),
            Expression::This { resolved } => {
                if let Some(distance) = *resolved {
                    self.get_at(self.environment.clone(), distance, "this")
                        .ok_or_else(|| InterpreterError::UndefinedVariable("this".to_string()))
                        .and_then(|v| match v {
                            Value::Instance(_) => Ok(v),
                            _ => Err(InterpreterError::Message(
                                "Expected an instance for 'this'.".to_string(),
                                ExitCode::RunTimeError,
                            )),
                        })
                } else {
                    Err(InterpreterError::Message(
                        "Cannot use 'this' here.".to_string(),
                        ExitCode::RunTimeError,
                    ))
                }
            }
            Expression::Super { method, resolved } => {
                let distance = resolved.ok_or_else(|| {
                    InterpreterError::Message(
                        "Cannot use 'super' outside of a class.".to_string(),
                        ExitCode::RunTimeError,
                    )
                })?;

                let super_class_val = self
                    .get_at(self.environment.clone(), distance, "super")
                    .ok_or(InterpreterError::Message(
                        "Cannot use 'super' outside of a class.".to_string(),
                        ExitCode::RunTimeError,
                    ))?;

                let super_class = match super_class_val {
                    Value::Class(c) => c,
                    _ => {
                        return Err(InterpreterError::Message(
                            "'super' must be used within a class that has a superclass."
                                .to_string(),
                            ExitCode::RunTimeError,
                        ))
                    }
                };

                let this_val =
                    self.environment
                        .borrow()
                        .get("this")
                        .ok_or(InterpreterError::Message(
                            "Cannot use 'super' in a static context.".to_string(),
                            ExitCode::RunTimeError,
                        ))?;

                let this_instance = match this_val {
                    Value::Instance(i) => i,
                    _ => {
                        return Err(InterpreterError::Message(
                            "Cannot use 'super' in a static context.".to_string(),
                            ExitCode::RunTimeError,
                        ))
                    }
                };

                let method_func = super_class.find_method(method).ok_or_else(|| {
                    InterpreterError::Message(
                        format!("Undefined property '{}'.", method),
                        ExitCode::RunTimeError,
                    )
                })?;

                Ok(Value::Function(Rc::new(BoundMethod {
                    function: Rc::new(method_func),
                    instance: this_instance,
                })))
            }
            Expression::Binary { .. } => self.visit_binary_expr(expr),
        }
    }

    fn visit_literal_expr(&mut self, literal: &crate::Literal) -> Result<Value, InterpreterError> {
        let value = match literal {
            Literal::Number(v) => Value::Number(*v),
            Literal::Boolean(v) => Value::Boolean(*v),
            Literal::Nil => Value::Nil,
            Literal::String(v) => Value::String(v.clone()),
        };

        Ok(value)
    }

    fn visit_unary_expr(
        &mut self,
        expr: &Expression,
        op: &TokenKind,
    ) -> Result<Value, InterpreterError> {
        let value = self.evaluate(expr)?;
        match (op, value.clone()) {
            (TokenKind::Minus, val) => match val {
                Value::Number(v) => Ok(Value::Number(-v)),
                _ => Err(InterpreterError::Message(
                    "Operand must be a number.".to_string(),
                    ExitCode::RunTimeError,
                )),
            },
            (TokenKind::Bang, val) => match val {
                Value::Boolean(v) => Ok(Value::Boolean(!v)),
                Value::Nil => Ok(Value::Boolean(true)),
                _ => Ok(Value::Boolean(false)),
            },
            _ => Ok(value),
        }
    }

    fn visit_binary_expr(&mut self, expr: &Expression) -> Result<Value, InterpreterError> {
        if let Expression::Binary {
            left,
            operator,
            right,
        } = expr
        {
            let left = self.evaluate(left)?;
            let right = self.evaluate(right)?;
            match (left, operator, right) {
                (Value::Number(n), TokenKind::Plus, Value::Number(n1)) => Ok(Value::Number(n + n1)),
                (Value::String(s), TokenKind::Plus, Value::String(s1)) => {
                    let s = format!("{s}{s1}");
                    Ok(Value::String(s))
                }
                (Value::Number(n), TokenKind::Minus, Value::Number(n1)) => {
                    Ok(Value::Number(n - n1))
                }
                (Value::Number(n), TokenKind::Star, Value::Number(n1)) => Ok(Value::Number(n * n1)),
                (Value::Number(n), TokenKind::Slash, Value::Number(n1)) => {
                    Ok(Value::Number(n / n1))
                }
                (Value::Number(n), TokenKind::Greater, Value::Number(n1)) => {
                    Ok(Value::Boolean(n > n1))
                }
                (Value::Number(n), TokenKind::Less, Value::Number(n1)) => {
                    Ok(Value::Boolean(n < n1))
                }
                (Value::Number(n), TokenKind::GreaterEqual, Value::Number(n1)) => {
                    Ok(Value::Boolean(n >= n1))
                }
                (Value::Number(n), TokenKind::LessEqual, Value::Number(n1)) => {
                    Ok(Value::Boolean(n <= n1))
                }

                (l, TokenKind::EqualEqual, r) => Ok(Value::Boolean(is_equal(&l, &r))),
                (l, TokenKind::BangEqual, r) => Ok(Value::Boolean(!is_equal(&l, &r))),
                _ => Err(InterpreterError::Message(
                    "Unsupported operation".to_string(),
                    ExitCode::RunTimeError,
                )),
            }
        } else {
            unreachable!()
        }
    }
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpreterError::UndefinedVariable(s) => write!(f, "Undefined variable '{s}'"),
            InterpreterError::Message(s, _) => write!(f, "{s}"),
            InterpreterError::ReturnError(v) => write!(f, "{v}"),
        }
    }
}

impl std::error::Error for InterpreterError {}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(v) => write!(f, "{v}"),
            Value::Boolean(v) => write!(f, "{v}"),
            Value::Nil => write!(f, "nil"),
            Value::String(v) => write!(f, "{v}"),
            Value::Function(fun) => write!(f, "<fn {}>", fun.name()),
            Value::Class(class) => write!(f, "{}", class.name()),
            Value::Instance(ins) => write!(f, "{}", ins.name()),
        }
    }
}

fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Boolean(v) => *v,
        Value::Nil => false,
        _ => true,
    }
}

fn is_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Nil, Value::Nil) => true,
        (Value::Nil, _) | (_, Value::Nil) => false,
        (Value::Boolean(b1), Value::Boolean(b2)) => b1 == b2,
        (Value::Number(n1), Value::Number(n2)) => n1 == n2,
        (Value::String(s1), Value::String(s2)) => s1 == s2,
        _ => false,
    }
}
