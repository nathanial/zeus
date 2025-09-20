use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::types::{EvalError, EvalResult, Expr};

impl Evaluator {
    pub fn apply_builtin(&mut self, name: &str, args: &[Expr]) -> EvalResult {
        match name {
            // Arithmetic operations
            "+" => self.builtin_add(args),
            "-" => self.builtin_subtract(args),
            "*" => self.builtin_multiply(args),
            "/" => self.builtin_divide(args),

            // Comparison operations
            "=" => self.builtin_equal(args),
            "<" => self.builtin_less(args),
            ">" => self.builtin_greater(args),

            // List operations
            "list" => Ok(Expr::List(args.to_vec())),
            "car" => self.builtin_car(args),
            "cdr" => self.builtin_cdr(args),
            "cons" => self.builtin_cons(args),
            "append" => self.builtin_append(args),
            "reverse" => self.builtin_reverse(args),
            "length" => self.builtin_length(args),
            "nth" => self.builtin_nth(args),
            "nthcdr" => self.builtin_nthcdr(args),
            "member" => self.builtin_member(args),

            // Higher-order functions
            "mapcar" => self.builtin_mapcar(args),
            "filter" => self.builtin_filter(args),
            "remove" => self.builtin_remove(args),
            "reduce" => self.builtin_reduce(args),

            // Function application
            "apply" => self.builtin_apply(args),
            "funcall" => self.builtin_funcall(args),

            // I/O
            "print" => self.builtin_print(args),
            "println" => self.builtin_println(args),

            // Symbol operations
            "gensym" => self.builtin_gensym(args),
            "get" => self.builtin_get(args),
            "put" => self.builtin_put(args),
            "symbol-plist" => self.builtin_symbol_plist(args),

            _ => Err(EvalError::message(format!("Unknown function: {}", name))),
        }
    }

    fn builtin_add(&mut self, args: &[Expr]) -> EvalResult {
        let mut sum = 0.0;
        for arg in args {
            if let Expr::Number(n) = arg {
                sum += n;
            } else {
                return Err(EvalError::message(format!(
                    "+ requires numeric arguments, got {:?}",
                    arg
                )));
            }
        }
        Ok(Expr::Number(sum))
    }

    fn builtin_subtract(&mut self, args: &[Expr]) -> EvalResult {
        if args.is_empty() {
            return Err(EvalError::message("- requires at least 1 argument"));
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
                    return Err(EvalError::message(format!(
                        "- requires numeric arguments, got {:?}",
                        arg
                    )));
                }
            }
            Ok(Expr::Number(result))
        } else {
            Err(EvalError::message(format!(
                "- requires numeric arguments, got {:?}",
                args[0]
            )))
        }
    }

    fn builtin_multiply(&mut self, args: &[Expr]) -> EvalResult {
        let mut product = 1.0;
        for arg in args {
            if let Expr::Number(n) = arg {
                product *= n;
            } else {
                return Err(EvalError::message(format!(
                    "* requires numeric arguments, got {:?}",
                    arg
                )));
            }
        }
        Ok(Expr::Number(product))
    }

    fn builtin_divide(&mut self, args: &[Expr]) -> EvalResult {
        if args.is_empty() {
            return Err(EvalError::message("/ requires at least 1 argument"));
        }

        if let Expr::Number(first) = &args[0] {
            if args.len() == 1 {
                return Ok(Expr::Number(1.0 / first));
            }

            let mut result = *first;
            for arg in &args[1..] {
                if let Expr::Number(n) = arg {
                    if *n == 0.0 {
                        return Err(EvalError::message("Division by zero"));
                    }
                    result /= n;
                } else {
                    return Err(EvalError::message(format!(
                        "/ requires numeric arguments, got {:?}",
                        arg
                    )));
                }
            }
            Ok(Expr::Number(result))
        } else {
            Err(EvalError::message(format!(
                "/ requires numeric arguments, got {:?}",
                args[0]
            )))
        }
    }

    fn builtin_equal(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 2 {
            return Err(EvalError::message("= requires exactly 2 arguments"));
        }

        let equal = match (&args[0], &args[1]) {
            (Expr::Number(a), Expr::Number(b)) => (a - b).abs() < f64::EPSILON,
            (Expr::String(a), Expr::String(b)) => a == b,
            (Expr::Symbol(a), Expr::Symbol(b)) => a == b,
            _ => false,
        };

        Ok(Expr::Number(if equal { 1.0 } else { 0.0 }))
    }

    fn builtin_less(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 2 {
            return Err(EvalError::message("< requires exactly 2 arguments"));
        }

        if let (Expr::Number(a), Expr::Number(b)) = (&args[0], &args[1]) {
            Ok(Expr::Number(if a < b { 1.0 } else { 0.0 }))
        } else {
            Err(EvalError::message("< requires numeric arguments"))
        }
    }

    fn builtin_greater(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 2 {
            return Err(EvalError::message("> requires exactly 2 arguments"));
        }

        if let (Expr::Number(a), Expr::Number(b)) = (&args[0], &args[1]) {
            Ok(Expr::Number(if a > b { 1.0 } else { 0.0 }))
        } else {
            Err(EvalError::message("> requires numeric arguments"))
        }
    }

    fn builtin_car(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 1 {
            return Err(EvalError::message("car requires exactly 1 argument"));
        }

        if let Expr::List(list) = &args[0] {
            if list.is_empty() {
                Err(EvalError::message("car: empty list"))
            } else {
                Ok(list[0].clone())
            }
        } else {
            Err(EvalError::message("car requires a list argument"))
        }
    }

    fn builtin_cdr(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 1 {
            return Err(EvalError::message("cdr requires exactly 1 argument"));
        }

        if let Expr::List(list) = &args[0] {
            if list.is_empty() {
                Ok(Expr::List(vec![]))
            } else {
                Ok(Expr::List(list[1..].to_vec()))
            }
        } else {
            Err(EvalError::message("cdr requires a list argument"))
        }
    }

    fn builtin_cons(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 2 {
            return Err(EvalError::message("cons requires exactly 2 arguments"));
        }

        if let Expr::List(list) = &args[1] {
            let mut new_list = vec![args[0].clone()];
            new_list.extend_from_slice(list);
            Ok(Expr::List(new_list))
        } else {
            Ok(Expr::List(vec![args[0].clone(), args[1].clone()]))
        }
    }

    fn builtin_append(&mut self, args: &[Expr]) -> EvalResult {
        let mut result = Vec::new();
        for arg in args {
            match arg {
                Expr::List(list) => result.extend_from_slice(list),
                _ => return Err(EvalError::message("append requires list arguments")),
            }
        }
        Ok(Expr::List(result))
    }

    fn builtin_reverse(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 1 {
            return Err(EvalError::message("reverse requires exactly 1 argument"));
        }

        match &args[0] {
            Expr::List(list) => {
                let mut reversed = list.clone();
                reversed.reverse();
                Ok(Expr::List(reversed))
            }
            _ => Err(EvalError::message("reverse requires a list argument")),
        }
    }

    fn builtin_length(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 1 {
            return Err(EvalError::message("length requires exactly 1 argument"));
        }

        match &args[0] {
            Expr::List(list) => Ok(Expr::Number(list.len() as f64)),
            Expr::String(s) => Ok(Expr::Number(s.len() as f64)),
            _ => Err(EvalError::message(
                "length requires a list or string argument",
            )),
        }
    }

    fn builtin_gensym(&mut self, args: &[Expr]) -> EvalResult {
        let prefix = if args.is_empty() {
            ""
        } else if args.len() == 1 {
            match &args[0] {
                Expr::String(s) => s,
                Expr::Symbol(sym_data) => sym_data.name(),
                _ => return Err(EvalError::message("gensym prefix must be a string or symbol")),
            }
        } else {
            return Err(EvalError::message("gensym takes at most 1 argument"));
        };

        let sym_data = self.environment.generate_gensym(prefix);
        Ok(Expr::Symbol(sym_data))
    }

    fn builtin_get(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 2 {
            return Err(EvalError::message("get requires exactly 2 arguments"));
        }

        let symbol = match &args[0] {
            Expr::Symbol(sym_data) => sym_data.name(),
            _ => return Err(EvalError::message("get: first argument must be a symbol")),
        };

        let property = match &args[1] {
            Expr::Symbol(sym_data) => sym_data.name(),
            Expr::String(s) => s,
            _ => return Err(EvalError::message("get: second argument must be a symbol or string")),
        };

        Ok(self.environment.get_property(symbol, property)
            .unwrap_or_else(|| Expr::List(vec![])))
    }

    fn builtin_put(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 3 {
            return Err(EvalError::message("put requires exactly 3 arguments"));
        }

        let symbol = match &args[0] {
            Expr::Symbol(sym_data) => sym_data.name().to_string(),
            _ => return Err(EvalError::message("put: first argument must be a symbol")),
        };

        let property = match &args[1] {
            Expr::Symbol(sym_data) => sym_data.name().to_string(),
            Expr::String(s) => s.clone(),
            _ => return Err(EvalError::message("put: second argument must be a symbol or string")),
        };

        let value = args[2].clone();
        self.environment.set_property(symbol, property, value.clone());
        Ok(value)
    }

    fn builtin_symbol_plist(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 1 {
            return Err(EvalError::message("symbol-plist requires exactly 1 argument"));
        }

        let symbol = match &args[0] {
            Expr::Symbol(sym_data) => sym_data.name(),
            _ => return Err(EvalError::message("symbol-plist: argument must be a symbol")),
        };

        let plist = self.environment.get_symbol_plist(symbol);
        Ok(Expr::List(plist))
    }

    // Continued in next part...
}
