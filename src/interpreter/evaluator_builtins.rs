use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::types::Expr;

impl Evaluator {
    pub fn apply_builtin(&mut self, name: &str, args: &[Expr]) -> Result<Expr, String> {
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

            _ => Err(format!("Unknown function: {}", name)),
        }
    }

    fn builtin_add(&mut self, args: &[Expr]) -> Result<Expr, String> {
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

    fn builtin_subtract(&mut self, args: &[Expr]) -> Result<Expr, String> {
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

    fn builtin_multiply(&mut self, args: &[Expr]) -> Result<Expr, String> {
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

    fn builtin_divide(&mut self, args: &[Expr]) -> Result<Expr, String> {
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

    fn builtin_equal(&mut self, args: &[Expr]) -> Result<Expr, String> {
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

    fn builtin_less(&mut self, args: &[Expr]) -> Result<Expr, String> {
        if args.len() != 2 {
            return Err("< requires exactly 2 arguments".to_string());
        }

        if let (Expr::Number(a), Expr::Number(b)) = (&args[0], &args[1]) {
            Ok(Expr::Number(if a < b { 1.0 } else { 0.0 }))
        } else {
            Err("< requires numeric arguments".to_string())
        }
    }

    fn builtin_greater(&mut self, args: &[Expr]) -> Result<Expr, String> {
        if args.len() != 2 {
            return Err("> requires exactly 2 arguments".to_string());
        }

        if let (Expr::Number(a), Expr::Number(b)) = (&args[0], &args[1]) {
            Ok(Expr::Number(if a > b { 1.0 } else { 0.0 }))
        } else {
            Err("> requires numeric arguments".to_string())
        }
    }

    fn builtin_car(&mut self, args: &[Expr]) -> Result<Expr, String> {
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

    fn builtin_cdr(&mut self, args: &[Expr]) -> Result<Expr, String> {
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

    fn builtin_cons(&mut self, args: &[Expr]) -> Result<Expr, String> {
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

    fn builtin_append(&mut self, args: &[Expr]) -> Result<Expr, String> {
        let mut result = Vec::new();
        for arg in args {
            match arg {
                Expr::List(list) => result.extend_from_slice(list),
                _ => return Err("append requires list arguments".to_string()),
            }
        }
        Ok(Expr::List(result))
    }

    fn builtin_reverse(&mut self, args: &[Expr]) -> Result<Expr, String> {
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

    fn builtin_length(&mut self, args: &[Expr]) -> Result<Expr, String> {
        if args.len() != 1 {
            return Err("length requires exactly 1 argument".to_string());
        }

        match &args[0] {
            Expr::List(list) => Ok(Expr::Number(list.len() as f64)),
            Expr::String(s) => Ok(Expr::Number(s.len() as f64)),
            _ => Err("length requires a list or string argument".to_string()),
        }
    }

    // Continued in next part...
}
