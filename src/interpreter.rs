use std::io::{self, Write};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LeftParen,
    RightParen,
    Symbol(String),
    Number(f64),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Symbol(String),
    String(String),
    List(Vec<Expr>),
}

pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Tokenizer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let ch = self.input[self.position];
            self.position += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_string(&mut self) -> Result<String, String> {
        let mut result = String::new();
        self.advance(); // Skip opening quote

        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance(); // Skip closing quote
                return Ok(result);
            } else if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.advance() {
                    match escaped {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '\\' => result.push('\\'),
                        '"' => result.push('"'),
                        _ => {
                            result.push('\\');
                            result.push(escaped);
                        }
                    }
                }
            } else {
                result.push(ch);
                self.advance();
            }
        }

        Err("Unterminated string".to_string())
    }

    fn read_symbol(&mut self) -> String {
        let mut result = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || "+-*/<>=!?_".contains(ch) {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        result
    }

    fn read_number(&mut self) -> Result<f64, String> {
        let mut result = String::new();
        let mut has_dot = false;

        if self.peek() == Some('-') {
            result.push('-');
            self.advance();
        }

        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                result.push(ch);
                self.advance();
            } else if ch == '.' && !has_dot {
                has_dot = true;
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        result.parse().map_err(|_| "Invalid number".to_string())
    }

    fn next_token(&mut self) -> Result<Option<Token>, String> {
        self.skip_whitespace();

        match self.peek() {
            None => Ok(None),
            Some('(') => {
                self.advance();
                Ok(Some(Token::LeftParen))
            }
            Some(')') => {
                self.advance();
                Ok(Some(Token::RightParen))
            }
            Some('"') => {
                let s = self.read_string()?;
                Ok(Some(Token::String(s)))
            }
            Some(ch) if ch == '-' || ch.is_ascii_digit() => {
                let start_pos = self.position;
                if ch == '-' {
                    self.advance();
                    if let Some(next_ch) = self.peek() {
                        if next_ch.is_ascii_digit() {
                            self.position = start_pos;
                            let num = self.read_number()?;
                            Ok(Some(Token::Number(num)))
                        } else {
                            self.position = start_pos;
                            let sym = self.read_symbol();
                            Ok(Some(Token::Symbol(sym)))
                        }
                    } else {
                        self.position = start_pos;
                        let sym = self.read_symbol();
                        Ok(Some(Token::Symbol(sym)))
                    }
                } else {
                    let num = self.read_number()?;
                    Ok(Some(Token::Number(num)))
                }
            }
            Some(_) => {
                let sym = self.read_symbol();
                if sym.is_empty() {
                    Err(format!("Unexpected character: {:?}", self.peek()))
                } else {
                    Ok(Some(Token::Symbol(sym)))
                }
            }
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }

        Ok(tokens)
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Some(Token::Number(n)) => Ok(Expr::Number(n)),
            Some(Token::Symbol(s)) => Ok(Expr::Symbol(s)),
            Some(Token::String(s)) => Ok(Expr::String(s)),
            Some(Token::LeftParen) => {
                let mut list = Vec::new();

                loop {
                    match self.peek() {
                        Some(Token::RightParen) => {
                            self.advance();
                            return Ok(Expr::List(list));
                        }
                        None => return Err("Unexpected end of input".to_string()),
                        _ => {
                            list.push(self.parse_expr()?);
                        }
                    }
                }
            }
            Some(Token::RightParen) => Err("Unexpected )".to_string()),
            None => Err("Unexpected end of input".to_string()),
        }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        let expr = self.parse_expr()?;

        if self.position < self.tokens.len() {
            Err("Extra tokens after expression".to_string())
        } else {
            Ok(expr)
        }
    }
}

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
                        "if" => self.eval_if(list),
                        "quote" => self.eval_quote(list),
                        "lambda" => self.eval_lambda(list),
                        "let" => self.eval_let(list),
                        "let*" => self.eval_let_star(list),
                        "cond" => self.eval_cond(list),
                        "and" => self.eval_and(list),
                        "or" => self.eval_or(list),
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

    fn eval_cond(&mut self, list: &[Expr]) -> Result<Expr, String> {
        if list.len() < 2 {
            return Err("cond requires at least 1 clause".to_string());
        }

        for clause in &list[1..] {
            match clause {
                Expr::List(clause_list) if !clause_list.is_empty() => {
                    let condition = &clause_list[0];

                    // Check for else clause
                    let is_else = match condition {
                        Expr::Symbol(s) if s == "else" => true,
                        _ => false,
                    };

                    if is_else {
                        // Execute else branch
                        if clause_list.len() < 2 {
                            return Ok(Expr::Number(1.0)); // else with no body returns true
                        }
                        let mut result = Ok(Expr::List(vec![]));
                        for expr in &clause_list[1..] {
                            result = self.eval(expr);
                            if result.is_err() {
                                return result;
                            }
                        }
                        return result;
                    }

                    // Evaluate condition
                    let cond_result = self.eval(condition)?;
                    let is_true = match cond_result {
                        Expr::Number(n) => n != 0.0,
                        Expr::List(ref l) => !l.is_empty(),
                        Expr::String(ref s) => !s.is_empty(),
                        _ => true,
                    };

                    if is_true {
                        if clause_list.len() == 1 {
                            return Ok(cond_result); // Return condition value if no body
                        }
                        // Execute this branch
                        let mut result = Ok(Expr::List(vec![]));
                        for expr in &clause_list[1..] {
                            result = self.eval(expr);
                            if result.is_err() {
                                return result;
                            }
                        }
                        return result;
                    }
                }
                _ => return Err("cond clause must be a non-empty list".to_string()),
            }
        }

        // No condition was true
        Ok(Expr::List(vec![]))
    }

    fn eval_and(&mut self, list: &[Expr]) -> Result<Expr, String> {
        if list.len() == 1 {
            return Ok(Expr::Number(1.0)); // (and) with no args returns true
        }

        let mut result = Expr::Number(1.0);
        for expr in &list[1..] {
            result = self.eval(expr)?;
            let is_false = match &result {
                Expr::Number(n) => *n == 0.0,
                Expr::List(l) => l.is_empty(),
                _ => false,
            };

            if is_false {
                return Ok(Expr::Number(0.0));
            }
        }

        Ok(result) // Return last value if all are truthy
    }

    fn eval_or(&mut self, list: &[Expr]) -> Result<Expr, String> {
        if list.len() == 1 {
            return Ok(Expr::Number(0.0)); // (or) with no args returns false
        }

        for expr in &list[1..] {
            let result = self.eval(expr)?;
            let is_true = match &result {
                Expr::Number(n) => *n != 0.0,
                Expr::List(l) => !l.is_empty(),
                _ => true,
            };

            if is_true {
                return Ok(result); // Return first truthy value
            }
        }

        Ok(Expr::Number(0.0)) // All were falsy
    }

    fn eval_application(&mut self, list: &[Expr]) -> Result<Expr, String> {
        let func = self.eval(&list[0])?;
        let args: Result<Vec<_>, _> = list[1..].iter().map(|e| self.eval(e)).collect();
        let args = args?;

        match func {
            Expr::Symbol(name) => self.apply_builtin(&name, &args),
            Expr::List(lambda) if lambda.len() == 3 && lambda[0] == Expr::Symbol("lambda".to_string()) => {
                self.apply_lambda(&lambda, &args)
            }
            _ => Err(format!("Cannot apply: {:?}", func)),
        }
    }

    fn apply_builtin(&mut self, name: &str, args: &[Expr]) -> Result<Expr, String> {
        match name {
            "+" => {
                let mut sum = 0.0;
                for arg in args {
                    if let Expr::Number(n) = arg {
                        sum += n;
                    } else {
                        return Err(format!("+ requires numeric arguments, got {:?}", arg));
                    }
                }
                Ok(Expr::Number(sum))
            }
            "-" => {
                if args.is_empty() {
                    return Err("- requires at least 1 argument".to_string());
                }

                if let Expr::Number(first) = &args[0] {
                    if args.len() == 1 {
                        return Ok(Expr::Number(-first));
                    }

                    let mut result = *first;
                    for arg in &args[1..] {
                        if let Expr::Number(n) = arg {
                            result -= n;
                        } else {
                            return Err(format!("- requires numeric arguments, got {:?}", arg));
                        }
                    }
                    Ok(Expr::Number(result))
                } else {
                    Err(format!("- requires numeric arguments, got {:?}", args[0]))
                }
            }
            "*" => {
                let mut product = 1.0;
                for arg in args {
                    if let Expr::Number(n) = arg {
                        product *= n;
                    } else {
                        return Err(format!("* requires numeric arguments, got {:?}", arg));
                    }
                }
                Ok(Expr::Number(product))
            }
            "/" => {
                if args.is_empty() {
                    return Err("/ requires at least 1 argument".to_string());
                }

                if let Expr::Number(first) = &args[0] {
                    if args.len() == 1 {
                        return Ok(Expr::Number(1.0 / first));
                    }

                    let mut result = *first;
                    for arg in &args[1..] {
                        if let Expr::Number(n) = arg {
                            if *n == 0.0 {
                                return Err("Division by zero".to_string());
                            }
                            result /= n;
                        } else {
                            return Err(format!("/ requires numeric arguments, got {:?}", arg));
                        }
                    }
                    Ok(Expr::Number(result))
                } else {
                    Err(format!("/ requires numeric arguments, got {:?}", args[0]))
                }
            }
            "=" => {
                if args.len() != 2 {
                    return Err("= requires exactly 2 arguments".to_string());
                }

                let equal = match (&args[0], &args[1]) {
                    (Expr::Number(a), Expr::Number(b)) => (a - b).abs() < f64::EPSILON,
                    (Expr::String(a), Expr::String(b)) => a == b,
                    (Expr::Symbol(a), Expr::Symbol(b)) => a == b,
                    _ => false,
                };

                Ok(Expr::Number(if equal { 1.0 } else { 0.0 }))
            }
            "<" => {
                if args.len() != 2 {
                    return Err("< requires exactly 2 arguments".to_string());
                }

                if let (Expr::Number(a), Expr::Number(b)) = (&args[0], &args[1]) {
                    Ok(Expr::Number(if a < b { 1.0 } else { 0.0 }))
                } else {
                    Err("< requires numeric arguments".to_string())
                }
            }
            ">" => {
                if args.len() != 2 {
                    return Err("> requires exactly 2 arguments".to_string());
                }

                if let (Expr::Number(a), Expr::Number(b)) = (&args[0], &args[1]) {
                    Ok(Expr::Number(if a > b { 1.0 } else { 0.0 }))
                } else {
                    Err("> requires numeric arguments".to_string())
                }
            }
            "list" => Ok(Expr::List(args.to_vec())),
            "car" => {
                if args.len() != 1 {
                    return Err("car requires exactly 1 argument".to_string());
                }

                if let Expr::List(list) = &args[0] {
                    if list.is_empty() {
                        Err("car: empty list".to_string())
                    } else {
                        Ok(list[0].clone())
                    }
                } else {
                    Err("car requires a list argument".to_string())
                }
            }
            "cdr" => {
                if args.len() != 1 {
                    return Err("cdr requires exactly 1 argument".to_string());
                }

                if let Expr::List(list) = &args[0] {
                    if list.is_empty() {
                        Ok(Expr::List(vec![]))
                    } else {
                        Ok(Expr::List(list[1..].to_vec()))
                    }
                } else {
                    Err("cdr requires a list argument".to_string())
                }
            }
            "cons" => {
                if args.len() != 2 {
                    return Err("cons requires exactly 2 arguments".to_string());
                }

                if let Expr::List(list) = &args[1] {
                    let mut new_list = vec![args[0].clone()];
                    new_list.extend_from_slice(list);
                    Ok(Expr::List(new_list))
                } else {
                    Ok(Expr::List(vec![args[0].clone(), args[1].clone()]))
                }
            }
            "append" => {
                let mut result = Vec::new();
                for arg in args {
                    match arg {
                        Expr::List(list) => result.extend_from_slice(list),
                        _ => return Err("append requires list arguments".to_string()),
                    }
                }
                Ok(Expr::List(result))
            }
            "reverse" => {
                if args.len() != 1 {
                    return Err("reverse requires exactly 1 argument".to_string());
                }

                match &args[0] {
                    Expr::List(list) => {
                        let mut reversed = list.clone();
                        reversed.reverse();
                        Ok(Expr::List(reversed))
                    }
                    _ => Err("reverse requires a list argument".to_string()),
                }
            }
            "length" => {
                if args.len() != 1 {
                    return Err("length requires exactly 1 argument".to_string());
                }

                match &args[0] {
                    Expr::List(list) => Ok(Expr::Number(list.len() as f64)),
                    Expr::String(s) => Ok(Expr::Number(s.len() as f64)),
                    _ => Err("length requires a list or string argument".to_string()),
                }
            }
            "nth" => {
                if args.len() != 2 {
                    return Err("nth requires exactly 2 arguments".to_string());
                }

                let index = match &args[0] {
                    Expr::Number(n) if *n >= 0.0 && n.fract() == 0.0 => *n as usize,
                    _ => return Err("nth index must be a non-negative integer".to_string()),
                };

                match &args[1] {
                    Expr::List(list) => {
                        list.get(index)
                            .cloned()
                            .ok_or_else(|| "nth index out of bounds".to_string())
                    }
                    _ => Err("nth requires a list as second argument".to_string()),
                }
            }
            _ => Err(format!("Unknown function: {}", name)),
        }
    }

    fn apply_lambda(&mut self, lambda: &[Expr], args: &[Expr]) -> Result<Expr, String> {
        if let Expr::List(params) = &lambda[1] {
            if params.len() != args.len() {
                return Err(format!(
                    "Lambda expects {} arguments, got {}",
                    params.len(),
                    args.len()
                ));
            }

            self.environment.push_scope();

            for (param, arg) in params.iter().zip(args.iter()) {
                if let Expr::Symbol(name) = param {
                    self.environment.set(name.clone(), arg.clone());
                } else {
                    self.environment.pop_scope();
                    return Err("Lambda parameters must be symbols".to_string());
                }
            }

            let result = self.eval(&lambda[2]);
            self.environment.pop_scope();
            result
        } else {
            Err("Lambda parameters must be a list".to_string())
        }
    }
}

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
            "+", "-", "*", "/", "=", "<", ">",
            "list", "car", "cdr", "cons",
            "append", "reverse", "length", "nth"
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

pub struct Repl {
    evaluator: Evaluator,
}

impl Repl {
    pub fn new() -> Self {
        Repl {
            evaluator: Evaluator::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            print!("zeus> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    // EOF reached
                    println!("\nGoodbye!");
                    break;
                }
                Ok(_) => {
                    let input = input.trim();

                    if input == "exit" {
                        println!("Goodbye!");
                        break;
                    }

                    if input.is_empty() {
                        continue;
                    }

                    match self.evaluate(input) {
                        Ok(result) => println!("{}", self.format_expr(&result)),
                        Err(e) => println!("Error: {}", e),
                    }
                }
                Err(error) => {
                    println!("Error reading input: {}", error);
                    break;
                }
            }
        }
    }

    pub fn evaluate(&mut self, input: &str) -> Result<Expr, String> {
        self.evaluator.eval_str(input)
    }

    pub fn format_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Number(n) => {
                if n.fract() == 0.0 && n.abs() < 1e10 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Expr::Symbol(s) => s.clone(),
            Expr::String(s) => format!("\"{}\"", s),
            Expr::List(list) => {
                let items: Vec<String> = list.iter().map(|e| self.format_expr(e)).collect();
                format!("({})", items.join(" "))
            }
        }
    }
}