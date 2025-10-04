use crate::{Expression, Interpreter, Statement};
use std::collections::HashMap;

#[derive(Debug)]
pub enum ResolverError {
    Message(String),
}

impl std::fmt::Display for ResolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolverError::Message(s) => write!(f, "Resolver Error: {}", s),
        }
    }
}

pub struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
    pub interpreter: Interpreter,
    current_function: FunctionType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FunctionType {
    None,
    Function,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        let scopes = vec![HashMap::new()];
        Resolver {
            scopes,
            interpreter,
            current_function: FunctionType::None,
        }
    }

    pub fn resolve_stmts(&mut self, statements: &mut [Statement]) -> Result<(), ResolverError> {
        for statement in statements.iter_mut() {
            self.resolve_stmt(statement)?;
        }
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn resolve_stmt(&mut self, stmt: &mut Statement) -> Result<(), ResolverError> {
        match stmt {
            Statement::Block(list) => {
                self.begin_scope();
                for statement in list.iter_mut() {
                    self.resolve_stmt(statement)?;
                }
                self.end_scope();
            }
            Statement::Var { name, initializer } => {
                self.declare(name)?;
                if let Some(expr) = initializer {
                    self.resolve_expr(expr)?;
                }
                self.define(name);
            }
            Statement::Function { name, params, body } => {
                self.declare(name)?;
                self.define(name);
                self.resolve_function(params, body, FunctionType::Function)?;
            }
            Statement::Expr(expr) | Statement::Print(expr) => {
                self.resolve_expr(expr)?;
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(then_branch)?;
                if let Some(else_stmt) = else_branch {
                    self.resolve_stmt(else_stmt)?;
                }
            }
            Statement::Return { value } => {
                if self.current_function == FunctionType::None {
                    return Err(ResolverError::Message(
                        "Can't return from top-level code.".to_string(),
                    ));
                }
                if let Some(expr) = value {
                    self.resolve_expr(expr)?;
                }
            }
            Statement::While { condition, body } => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(body)?;
            }
            Statement::For {
                initialize,
                condition,
                increment,
                body,
            } => {
                self.begin_scope();

                if let Some(init) = initialize {
                    self.resolve_stmt(init)?;
                }
                if let Some(con) = condition {
                    self.resolve_expr(con)?;
                }
                self.resolve_stmt(body)?;
                if let Some(inc) = increment {
                    self.resolve_expr(inc)?;
                }

                self.end_scope();
            }
        }
        Ok(())
    }

    fn resolve_expr(&mut self, expr: &mut Expression) -> Result<(), ResolverError> {
        match expr {
            Expression::Literal(_) | Expression::Group(_) => {}
            Expression::Unary { expression, .. } => {
                self.resolve_expr(expression.as_mut())?;
            }
            Expression::Binary { left, right, .. } | Expression::Logical { left, right, .. } => {
                self.resolve_expr(left.as_mut())?;
                self.resolve_expr(right.as_mut())?;
            }
            Expression::Variable { name, resolved } => {
                let distance = self.scopes.iter().rev().position(|s| s.contains_key(name));
                *resolved = distance;
            }
            Expression::Assign {
                name,
                value,
                resolved,
            } => {
                self.resolve_expr(value.as_mut())?;
                let distance = self.scopes.iter().rev().position(|s| s.contains_key(name));
                *resolved = distance;
            }
            Expression::Call { callee, args } => {
                self.resolve_expr(callee.as_mut())?;
                for arg in args.iter_mut() {
                    self.resolve_expr(arg)?;
                }
            }
        }
        Ok(())
    }

    fn declare(&mut self, name: &str) -> Result<(), ResolverError> {
        let len = self.scopes.len();
        let is_global = len == 1;
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(name) && !is_global {
                return Err(ResolverError::Message(format!(
                    "Already a variable with name '{}' in this scope.",
                    name
                )));
            }
            scope.insert(name.to_string(), false);
        }
        Ok(())
    }

    fn define(&mut self, name: &str) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), true);
        }
    }

    fn resolve_function(
        &mut self,
        params: &[String],
        body: &mut [Statement],
        function_type: FunctionType,
    ) -> Result<(), ResolverError> {
        let enclosing_function = self.current_function.clone();
        self.current_function = function_type;

        self.begin_scope();
        for param in params {
            self.declare(param)?;
            self.define(param);
        }
        self.resolve_stmts(body)?;
        self.end_scope();

        self.current_function = enclosing_function;
        Ok(())
    }
}
