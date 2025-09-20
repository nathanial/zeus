use crate::interpreter::{
    types::Expr,
    environment::Environment,
    tokenizer::Tokenizer,
    parser::Parser,
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
        self.eval(&expr)
    }

    pub fn eval_once(input: &str) -> Result<Expr, String> {
        let mut evaluator = Self::new();
        evaluator.eval_str(input)
    }

    pub fn eval(&mut self, expr: &Expr) -> Result<Expr, String> {
        match expr {
            Expr::Number(_) | Expr::String(_) => Ok(expr.clone()),
            Expr::Symbol(name) => self.environment.get(name),
            Expr::List(list) => {
                if list.is_empty() {
                    return Ok(Expr::List(vec![]));
                }

                let first = &list[0];
                match first {
                    Expr::Symbol(name) => match name.as_str() {
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
                        _ => self.eval_application(list),
                    },
                    _ => self.eval_application(list),
                }
            }
        }
    }

    fn eval_define(&mut self, list: &[Expr]) -> Result<Expr, String> {
        if list.len() != 3 {
            return Err("define requires exactly 2 arguments".to_string());
        }

        if let Expr::Symbol(name) = &list[1] {
            let value = self.eval(&list[2])?;
            self.environment.set(name.clone(), value.clone());
            Ok(value)
        } else {
            Err("First argument to define must be a symbol".to_string())
        }
    }

    fn eval_defun(&mut self, list: &[Expr]) -> Result<Expr, String> {
        if list.len() < 4 {
            return Err("defun requires at least 3 arguments: name, params, and body".to_string());
        }

        let name = match &list[1] {
            Expr::Symbol(s) => s.clone(),
            _ => return Err("First argument to defun must be a symbol".to_string()),
        };

        let params = match &list[2] {
            Expr::List(params) => {
                // Verify all params are symbols
                for param in params {
                    if !matches!(param, Expr::Symbol(_)) {
                        return Err("All parameters must be symbols".to_string());
                    }
                }
                list[2].clone()
            }
            _ => return Err("Second argument to defun must be a parameter list".to_string()),
        };

        // Build the lambda expression: (lambda params body...)
        let mut lambda_expr = vec![
            Expr::Symbol("lambda".to_string()),
            params,
        ];

        // If there are multiple body expressions, wrap them in progn
        if list.len() == 4 {
            // Single body expression
            lambda_expr.push(list[3].clone());
        } else {
            // Multiple body expressions - wrap in progn
            let mut progn_expr = vec![Expr::Symbol("progn".to_string())];
            for body_expr in &list[3..] {
                progn_expr.push(body_expr.clone());
            }
            lambda_expr.push(Expr::List(progn_expr));
        }

        let lambda = Expr::List(lambda_expr);

        // Store the lambda in the environment
        self.environment.set(name.clone(), lambda.clone());

        // Return the function name as a symbol
        Ok(Expr::Symbol(name))
    }

    fn eval_if(&mut self, list: &[Expr]) -> Result<Expr, String> {
        if list.len() != 4 {
            return Err("if requires exactly 3 arguments".to_string());
        }

        let condition = self.eval(&list[1])?;
        let is_true = match condition {
            Expr::Number(n) => n != 0.0,
            Expr::List(ref l) => !l.is_empty(),
            _ => true,
        };

        if is_true {
            self.eval(&list[2])
        } else {
            self.eval(&list[3])
        }
    }

    fn eval_quote(&mut self, list: &[Expr]) -> Result<Expr, String> {
        if list.len() != 2 {
            return Err("quote requires exactly 1 argument".to_string());
        }
        Ok(list[1].clone())
    }

    fn eval_lambda(&mut self, list: &[Expr]) -> Result<Expr, String> {
        if list.len() != 3 {
            return Err("lambda requires exactly 2 arguments".to_string());
        }

        Ok(Expr::List(list.to_vec()))
    }

    fn eval_let(&mut self, list: &[Expr]) -> Result<Expr, String> {
        if list.len() < 3 {
            return Err("let requires at least 2 arguments".to_string());
        }

        let bindings = match &list[1] {
            Expr::List(bindings) => bindings,
            _ => return Err("let bindings must be a list".to_string()),
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
                        return Err("let binding must start with a symbol".to_string());
                    }
                }
                _ => {
                    self.environment.pop_scope();
                    return Err("let binding must be a list of two elements".to_string());
                }
            }
        }

        // Now set all the bindings
        for (symbol, value) in binding_values {
            if let Expr::Symbol(name) = symbol {
                self.environment.set(name, value);
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

    fn eval_let_star(&mut self, list: &[Expr]) -> Result<Expr, String> {
        if list.len() < 3 {
            return Err("let* requires at least 2 arguments".to_string());
        }

        let bindings = match &list[1] {
            Expr::List(bindings) => bindings,
            _ => return Err("let* bindings must be a list".to_string()),
        };

        self.environment.push_scope();

        // Process bindings sequentially (let* behavior)
        for binding in bindings {
            match binding {
                Expr::List(pair) if pair.len() == 2 => {
                    if let Expr::Symbol(name) = &pair[0] {
                        let value = self.eval(&pair[1])?;
                        self.environment.set(name.clone(), value);
                    } else {
                        self.environment.pop_scope();
                        return Err("let* binding must start with a symbol".to_string());
                    }
                }
                _ => {
                    self.environment.pop_scope();
                    return Err("let* binding must be a list of two elements".to_string());
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

    // Continued in evaluator_special_forms.rs and evaluator_builtins.rs...
}