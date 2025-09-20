use crate::interpreter::types::Expr;
use std::collections::HashMap;

pub struct Environment {
    scopes: Vec<HashMap<String, Expr>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn define_builtins(&mut self) {
        let builtins = vec![
            "+", "-", "*", "/", "=", "<", ">", "list", "car", "cdr", "cons", "append", "reverse",
            "length", "nth", "nthcdr", "mapcar", "filter", "remove", "member", "reduce", "apply",
            "funcall", "print", "println",
        ];
        for builtin in builtins {
            self.set(builtin.to_string(), Expr::Symbol(builtin.to_string()));
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn set(&mut self, name: String, value: Expr) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    pub fn get(&self, name: &str) -> Result<Expr, String> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }
        Err(format!("Undefined variable: {}", name))
    }
}
