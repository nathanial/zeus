use crate::interpreter::types::{Expr, SymbolData};
use std::collections::HashMap;

pub struct Environment {
    scopes: Vec<HashMap<String, Expr>>,
    symbol_properties: HashMap<String, HashMap<String, Expr>>,
    gensym_counter: u64,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            scopes: vec![HashMap::new()],
            symbol_properties: HashMap::new(),
            gensym_counter: 0,
        }
    }

    pub fn define_builtins(&mut self) {
        let builtins = vec![
            "+", "-", "*", "/", "=", "<", ">", "list", "car", "cdr", "cons", "append", "reverse",
            "length", "nth", "nthcdr", "mapcar", "filter", "remove", "member", "reduce", "apply",
            "funcall", "print", "println", "gensym", "get", "put", "symbol-plist",
        ];
        for builtin in builtins {
            self.set(builtin.to_string(), Expr::Symbol(SymbolData::Interned(builtin.to_string())));
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

    pub fn get_property(&self, symbol: &str, property: &str) -> Option<Expr> {
        self.symbol_properties
            .get(symbol)
            .and_then(|props| props.get(property))
            .cloned()
    }

    pub fn set_property(&mut self, symbol: String, property: String, value: Expr) {
        self.symbol_properties
            .entry(symbol)
            .or_insert_with(HashMap::new)
            .insert(property, value);
    }

    pub fn get_symbol_plist(&self, symbol: &str) -> Vec<Expr> {
        if let Some(props) = self.symbol_properties.get(symbol) {
            let mut plist = Vec::new();
            for (key, value) in props.iter() {
                plist.push(Expr::Symbol(SymbolData::Keyword(key.clone())));
                plist.push(value.clone());
            }
            plist
        } else {
            Vec::new()
        }
    }

    pub fn generate_gensym(&mut self, prefix: &str) -> SymbolData {
        let id = self.gensym_counter;
        self.gensym_counter += 1;
        let name = if prefix.is_empty() {
            format!("G{}", id)
        } else {
            format!("{}{}", prefix, id)
        };
        SymbolData::Uninterned(name, id)
    }
}
