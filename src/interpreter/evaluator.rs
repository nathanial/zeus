use crate::interpreter::{
    environment::Environment,
    parser::Parser,
    tokenizer::Tokenizer,
    types::{EvalError, EvalResult, Expr, HashKey, SymbolData},
};

pub struct Evaluator {
    pub environment: Environment,
}

impl Evaluator {
    pub fn new() -> Self {
        let mut env = Environment::new();
        env.define_builtins();
        Evaluator { environment: env }
    }

    pub fn parse(input: &str) -> Result<Expr, String> {
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize()?;

        if tokens.is_empty() {
            return Ok(Expr::List(vec![]));
        }

        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    pub fn eval_str(&mut self, input: &str) -> Result<Expr, String> {
        let expr = Self::parse(input)?;
        self.eval(&expr).map_err(|e| e.to_string())
    }

    pub fn eval_once(input: &str) -> Result<Expr, String> {
        let mut evaluator = Self::new();
        evaluator.eval_str(input)
    }

    // Helper function to convert Expr to numeric value
    pub fn to_number(expr: &Expr) -> Result<f64, String> {
        match expr {
            Expr::Integer(n) => Ok(*n as f64),
            Expr::Float(f) => Ok(*f),
            Expr::Rational {
                numerator,
                denominator,
            } => Ok(*numerator as f64 / *denominator as f64),
            _ => Err(format!("Not a number: {:?}", expr)),
        }
    }

    // Helper to check if expression is truthy
    pub fn is_truthy(expr: &Expr) -> bool {
        !matches!(expr, Expr::List(list) if list.is_empty())
    }

    pub fn bool_to_expr(value: bool) -> Expr {
        if value {
            Expr::Symbol(SymbolData::Interned("t".to_string()))
        } else {
            Expr::List(vec![])
        }
    }

    // Helper to convert Expr to HashKey for hash tables
    pub fn expr_to_hashkey(expr: &Expr) -> Option<HashKey> {
        match expr {
            Expr::Integer(n) => Some(HashKey::Integer(*n)),
            Expr::String(s) => Some(HashKey::String(s.clone())),
            Expr::Character(ch) => Some(HashKey::Character(*ch)),
            Expr::Symbol(SymbolData::Keyword(name)) => Some(HashKey::Keyword(name.clone())),
            Expr::Symbol(SymbolData::Interned(name)) => Some(HashKey::Symbol(name.clone())),
            _ => None,
        }
    }

    pub fn eval(&mut self, expr: &Expr) -> EvalResult {
        match expr {
            Expr::Integer(_)
            | Expr::Float(_)
            | Expr::Rational { .. }
            | Expr::String(_)
            | Expr::Character(_)
            | Expr::Vector(_)
            | Expr::HashTable(_)
            | Expr::Cons(_, _) => Ok(expr.clone()),
            Expr::Symbol(sym_data) => {
                match sym_data {
                    SymbolData::Keyword(_) => {
                        // Keywords are self-evaluating
                        Ok(expr.clone())
                    }
                    SymbolData::Interned(name) | SymbolData::Uninterned(name, _) => {
                        // Regular symbols and uninterned symbols evaluate to their values
                        self.environment.get(name).map_err(EvalError::message)
                    }
                }
            }
            Expr::List(list) => {
                if list.is_empty() {
                    return Ok(Expr::List(vec![]));
                }

                let first = &list[0];
                match first {
                    Expr::Symbol(sym_data) => match sym_data.name() {
                        "define" => self.eval_define(list),
                        "defun" => self.eval_defun(list),
                        "if" => self.eval_if(list),
                        "quote" => self.eval_quote(list),
                        "lambda" => self.eval_lambda(list),
                        "let" => self.eval_let(list),
                        "let*" => self.eval_let_star(list),
                        "cond" => self.eval_cond(list),
                        "and" => self.eval_and(list),
                        "or" => self.eval_or(list),
                        "progn" => self.eval_progn(list),
                        "when" => self.eval_when(list),
                        "unless" => self.eval_unless(list),
                        "case" => self.eval_case(list),
                        "letrec" => self.eval_letrec(list),
                        "begin" => self.eval_begin(list),
                        "do" => self.eval_do(list),
                        "loop" => self.eval_loop(list),
                        "catch" => self.eval_catch(list),
                        "throw" => self.eval_throw(list),
                        "unwind-protect" => self.eval_unwind_protect(list),
                        "block" => self.eval_block(list),
                        "return-from" => self.eval_return_from(list),
                        "tagbody" => self.eval_tagbody(list),
                        "go" => self.eval_go(list),
                        _ => self.eval_application(list),
                    },
                    _ => self.eval_application(list),
                }
            }
        }
    }

    fn eval_define(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() != 3 {
            return Err(EvalError::message("define requires exactly 2 arguments"));
        }

        if let Expr::Symbol(sym_data) = &list[1] {
            if sym_data.is_keyword() {
                return Err(EvalError::message("Cannot define a keyword"));
            }
            let value = self.eval(&list[2])?;
            self.environment
                .set(sym_data.name().to_string(), value.clone());
            Ok(value)
        } else {
            Err(EvalError::message(
                "First argument to define must be a symbol",
            ))
        }
    }

    fn eval_defun(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() < 4 {
            return Err(EvalError::message(
                "defun requires at least 3 arguments: name, params, and body",
            ));
        }

        let name = match &list[1] {
            Expr::Symbol(sym_data) => {
                if sym_data.is_keyword() {
                    return Err(EvalError::message("Cannot defun a keyword"));
                }
                sym_data.name().to_string()
            }
            _ => {
                return Err(EvalError::message(
                    "First argument to defun must be a symbol",
                ))
            }
        };

        let params = match &list[2] {
            Expr::List(params) => {
                // Verify all params are symbols
                for param in params {
                    if !matches!(param, Expr::Symbol(_)) {
                        return Err(EvalError::message("All parameters must be symbols"));
                    }
                }
                list[2].clone()
            }
            _ => {
                return Err(EvalError::message(
                    "Second argument to defun must be a parameter list",
                ))
            }
        };

        // Build the lambda expression: (lambda params body...)
        let mut lambda_expr = vec![
            Expr::Symbol(SymbolData::Interned("lambda".to_string())),
            params,
        ];

        // If there are multiple body expressions, wrap them in progn
        if list.len() == 4 {
            // Single body expression
            lambda_expr.push(list[3].clone());
        } else {
            // Multiple body expressions - wrap in progn
            let mut progn_expr = vec![Expr::Symbol(SymbolData::Interned("progn".to_string()))];
            for body_expr in &list[3..] {
                progn_expr.push(body_expr.clone());
            }
            lambda_expr.push(Expr::List(progn_expr));
        }

        let lambda = Expr::List(lambda_expr);

        // Store the lambda in the environment
        self.environment.set(name.clone(), lambda.clone());

        // Return the function name as a symbol
        Ok(Expr::Symbol(SymbolData::Interned(name)))
    }

    fn eval_if(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() != 4 {
            return Err(EvalError::message("if requires exactly 3 arguments"));
        }

        let condition = self.eval(&list[1])?;
        let is_true = Self::is_truthy(&condition);

        if is_true {
            self.eval(&list[2])
        } else {
            self.eval(&list[3])
        }
    }

    fn eval_quote(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() != 2 {
            return Err(EvalError::message("quote requires exactly 1 argument"));
        }
        Ok(list[1].clone())
    }

    fn eval_lambda(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() != 3 {
            return Err(EvalError::message("lambda requires exactly 2 arguments"));
        }

        // Validate parameters - they must be symbols and not keywords
        if let Expr::List(params) = &list[1] {
            for param in params {
                match param {
                    Expr::Symbol(sym_data) => {
                        if sym_data.is_keyword() {
                            return Err(EvalError::message("Lambda parameter cannot be a keyword"));
                        }
                    }
                    _ => return Err(EvalError::message("Lambda parameters must be symbols")),
                }
            }
        } else {
            return Err(EvalError::message("Lambda parameters must be a list"));
        }

        Ok(Expr::List(list.to_vec()))
    }

    fn eval_let(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() < 3 {
            return Err(EvalError::message("let requires at least 2 arguments"));
        }

        let bindings = match &list[1] {
            Expr::List(bindings) => bindings,
            _ => return Err(EvalError::message("let bindings must be a list")),
        };

        self.environment.push_scope();

        // Process all bindings in parallel (standard let behavior)
        let mut binding_values = Vec::new();
        for binding in bindings {
            match binding {
                Expr::List(pair) if pair.len() == 2 => {
                    if let Expr::Symbol(_) = &pair[0] {
                        let value = self.eval(&pair[1])?;
                        binding_values.push((pair[0].clone(), value));
                    } else {
                        self.environment.pop_scope();
                        return Err(EvalError::message("let binding must start with a symbol"));
                    }
                }
                _ => {
                    self.environment.pop_scope();
                    return Err(EvalError::message(
                        "let binding must be a list of two elements",
                    ));
                }
            }
        }

        // Now set all the bindings
        for (symbol, value) in binding_values {
            if let Expr::Symbol(sym_data) = symbol {
                if !sym_data.is_keyword() {
                    self.environment.set(sym_data.name().to_string(), value);
                }
            }
        }

        // Evaluate body expressions
        let mut result = Ok(Expr::List(vec![]));
        for body_expr in &list[2..] {
            result = self.eval(body_expr);
            if result.is_err() {
                break;
            }
        }

        self.environment.pop_scope();
        result
    }

    fn eval_let_star(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() < 3 {
            return Err(EvalError::message("let* requires at least 2 arguments"));
        }

        let bindings = match &list[1] {
            Expr::List(bindings) => bindings,
            _ => return Err(EvalError::message("let* bindings must be a list")),
        };

        self.environment.push_scope();

        // Process bindings sequentially (let* behavior)
        for binding in bindings {
            match binding {
                Expr::List(pair) if pair.len() == 2 => {
                    if let Expr::Symbol(sym_data) = &pair[0] {
                        if sym_data.is_keyword() {
                            self.environment.pop_scope();
                            return Err(EvalError::message("Cannot bind to a keyword"));
                        }
                        let value = self.eval(&pair[1])?;
                        self.environment.set(sym_data.name().to_string(), value);
                    } else {
                        self.environment.pop_scope();
                        return Err(EvalError::message("let* binding must start with a symbol"));
                    }
                }
                _ => {
                    self.environment.pop_scope();
                    return Err(EvalError::message(
                        "let* binding must be a list of two elements",
                    ));
                }
            }
        }

        // Evaluate body expressions
        let mut result = Ok(Expr::List(vec![]));
        for body_expr in &list[2..] {
            result = self.eval(body_expr);
            if result.is_err() {
                break;
            }
        }

        self.environment.pop_scope();
        result
    }

    fn eval_letrec(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() < 3 {
            return Err(EvalError::message("letrec requires at least 2 arguments"));
        }

        let bindings = match &list[1] {
            Expr::List(bindings) => bindings,
            _ => return Err(EvalError::message("letrec bindings must be a list")),
        };

        self.environment.push_scope();
        let result = (|| -> EvalResult {
            // Pre-bind all variables to nil so they are visible during initialization
            for binding in bindings {
                match binding {
                    Expr::List(pair) if !pair.is_empty() => {
                        if let Expr::Symbol(sym_data) = &pair[0] {
                            if sym_data.is_keyword() {
                                return Err(EvalError::message("Cannot bind to a keyword"));
                            }
                            self.environment
                                .set(sym_data.name().to_string(), Expr::List(vec![]));
                        } else {
                            return Err(EvalError::message(
                                "letrec binding must start with a symbol",
                            ));
                        }
                    }
                    _ => {
                        return Err(EvalError::message(
                            "letrec binding must be a non-empty list",
                        ))
                    }
                }
            }

            // Evaluate initial values with access to all bindings
            for binding in bindings {
                match binding {
                    Expr::List(pair) if pair.len() >= 2 => {
                        if let Expr::Symbol(sym_data) = &pair[0] {
                            if sym_data.is_keyword() {
                                return Err(EvalError::message("Cannot bind to a keyword"));
                            }
                            let value = self.eval(&pair[1])?;
                            self.environment.set(sym_data.name().to_string(), value);
                        } else {
                            return Err(EvalError::message(
                                "letrec binding must start with a symbol",
                            ));
                        }
                    }
                    _ => {
                        return Err(EvalError::message(
                            "letrec binding must have at least a symbol and value",
                        ))
                    }
                }
            }

            // Evaluate body expressions
            let mut last = Expr::List(vec![]);
            for body_expr in &list[2..] {
                last = self.eval(body_expr)?;
            }
            Ok(last)
        })();
        self.environment.pop_scope();
        result
    }

    fn eval_begin(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() == 1 {
            return Ok(Expr::List(vec![]));
        }

        let mut result = Expr::List(vec![]);
        for expr in &list[1..] {
            result = self.eval(expr)?;
        }
        Ok(result)
    }

    // Continued in evaluator_special_forms.rs and evaluator_builtins.rs...
}
