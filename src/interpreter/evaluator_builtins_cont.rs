use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::types::Expr;

impl Evaluator {
    pub fn builtin_nth(&mut self, args: &[Expr]) -> Result<Expr, String> {
        if args.len() != 2 {
            return Err("nth requires exactly 2 arguments".to_string());
        }

        let index = match &args[0] {
            Expr::Number(n) if *n >= 0.0 && n.fract() == 0.0 => *n as usize,
            _ => return Err("nth index must be a non-negative integer".to_string()),
        };

        match &args[1] {
            Expr::List(list) => list
                .get(index)
                .cloned()
                .ok_or_else(|| "nth index out of bounds".to_string()),
            _ => Err("nth requires a list as second argument".to_string()),
        }
    }

    pub fn builtin_nthcdr(&mut self, args: &[Expr]) -> Result<Expr, String> {
        if args.len() != 2 {
            return Err("nthcdr requires exactly 2 arguments".to_string());
        }

        let n = match &args[0] {
            Expr::Number(num) if *num >= 0.0 && num.fract() == 0.0 => *num as usize,
            _ => return Err("nthcdr index must be a non-negative integer".to_string()),
        };

        match &args[1] {
            Expr::List(list) => {
                if n >= list.len() {
                    Ok(Expr::List(vec![]))
                } else {
                    Ok(Expr::List(list[n..].to_vec()))
                }
            }
            _ => Err("nthcdr requires a list as second argument".to_string()),
        }
    }

    pub fn builtin_member(&mut self, args: &[Expr]) -> Result<Expr, String> {
        if args.len() != 2 {
            return Err("member requires exactly 2 arguments".to_string());
        }

        let item = &args[0];
        let list = match &args[1] {
            Expr::List(l) => l,
            _ => return Err("member requires a list as second argument".to_string()),
        };

        for (i, elem) in list.iter().enumerate() {
            if self.expr_equal(item, elem) {
                return Ok(Expr::List(list[i..].to_vec()));
            }
        }

        Ok(Expr::List(vec![])) // Not found returns empty list
    }

    pub fn builtin_mapcar(&mut self, args: &[Expr]) -> Result<Expr, String> {
        if args.len() < 2 {
            return Err("mapcar requires at least 2 arguments".to_string());
        }

        let func = &args[0];
        let lists: Vec<&Vec<Expr>> = args[1..]
            .iter()
            .map(|arg| match arg {
                Expr::List(l) => Ok(l),
                _ => Err("mapcar requires list arguments"),
            })
            .collect::<Result<Vec<_>, _>>()?;

        if lists.is_empty() {
            return Ok(Expr::List(vec![]));
        }

        let min_len = lists.iter().map(|l| l.len()).min().unwrap_or(0);
        let mut result = Vec::new();

        for i in 0..min_len {
            let func_args: Vec<Expr> = lists.iter().map(|l| l[i].clone()).collect();

            let val = match func {
                Expr::Symbol(name) => self.apply_builtin(name, &func_args)?,
                Expr::List(lambda)
                    if lambda.len() == 3 && lambda[0] == Expr::Symbol("lambda".to_string()) =>
                {
                    self.apply_lambda(lambda, &func_args)?
                }
                _ => return Err("mapcar requires a function as first argument".to_string()),
            };
            result.push(val);
        }

        Ok(Expr::List(result))
    }

    pub fn builtin_filter(&mut self, args: &[Expr]) -> Result<Expr, String> {
        if args.len() != 2 {
            return Err("filter requires exactly 2 arguments".to_string());
        }

        let pred = &args[0];
        let list = match &args[1] {
            Expr::List(l) => l,
            _ => return Err("filter requires a list as second argument".to_string()),
        };

        let mut result = Vec::new();
        for item in list {
            let test_result = match pred {
                Expr::Symbol(name) => self.apply_builtin(name, &[item.clone()])?,
                Expr::List(lambda)
                    if lambda.len() == 3 && lambda[0] == Expr::Symbol("lambda".to_string()) =>
                {
                    self.apply_lambda(lambda, &[item.clone()])?
                }
                _ => return Err("filter requires a predicate function".to_string()),
            };

            let is_true = match test_result {
                Expr::Number(n) => n != 0.0,
                Expr::List(ref l) => !l.is_empty(),
                _ => true,
            };

            if is_true {
                result.push(item.clone());
            }
        }

        Ok(Expr::List(result))
    }

    pub fn builtin_remove(&mut self, args: &[Expr]) -> Result<Expr, String> {
        if args.len() != 2 {
            return Err("remove requires exactly 2 arguments".to_string());
        }

        let pred = &args[0];
        let list = match &args[1] {
            Expr::List(l) => l,
            _ => return Err("remove requires a list as second argument".to_string()),
        };

        let mut result = Vec::new();
        for item in list {
            let test_result = match pred {
                Expr::Symbol(name) => self.apply_builtin(name, &[item.clone()])?,
                Expr::List(lambda)
                    if lambda.len() == 3 && lambda[0] == Expr::Symbol("lambda".to_string()) =>
                {
                    self.apply_lambda(lambda, &[item.clone()])?
                }
                _ => return Err("remove requires a predicate function".to_string()),
            };

            let is_true = match test_result {
                Expr::Number(n) => n != 0.0,
                Expr::List(ref l) => !l.is_empty(),
                _ => true,
            };

            if !is_true {
                // Note: opposite of filter
                result.push(item.clone());
            }
        }

        Ok(Expr::List(result))
    }

    pub fn builtin_reduce(&mut self, args: &[Expr]) -> Result<Expr, String> {
        if args.len() < 2 || args.len() > 3 {
            return Err("reduce requires 2 or 3 arguments".to_string());
        }

        let func = &args[0];
        let list = match &args[1] {
            Expr::List(l) => l,
            _ => return Err("reduce requires a list as second argument".to_string()),
        };

        if list.is_empty() {
            if args.len() == 3 {
                return Ok(args[2].clone()); // Return initial value
            } else {
                return Err("reduce of empty list with no initial value".to_string());
            }
        }

        let (mut acc, start_idx) = if args.len() == 3 {
            (args[2].clone(), 0)
        } else {
            (list[0].clone(), 1)
        };

        for item in &list[start_idx..] {
            acc = match func {
                Expr::Symbol(name) => self.apply_builtin(name, &[acc, item.clone()])?,
                Expr::List(lambda)
                    if lambda.len() == 3 && lambda[0] == Expr::Symbol("lambda".to_string()) =>
                {
                    self.apply_lambda(lambda, &[acc, item.clone()])?
                }
                _ => return Err("reduce requires a function as first argument".to_string()),
            };
        }

        Ok(acc)
    }

    pub fn builtin_apply(&mut self, args: &[Expr]) -> Result<Expr, String> {
        if args.len() != 2 {
            return Err("apply requires exactly 2 arguments".to_string());
        }

        let func = &args[0];
        let list_args = match &args[1] {
            Expr::List(l) => l.clone(),
            _ => return Err("apply requires a list as second argument".to_string()),
        };

        match func {
            Expr::Symbol(name) => self.apply_builtin(name, &list_args),
            Expr::List(lambda)
                if lambda.len() == 3 && lambda[0] == Expr::Symbol("lambda".to_string()) =>
            {
                self.apply_lambda(lambda, &list_args)
            }
            _ => Err("apply requires a function as first argument".to_string()),
        }
    }

    pub fn builtin_funcall(&mut self, args: &[Expr]) -> Result<Expr, String> {
        if args.is_empty() {
            return Err("funcall requires at least 1 argument".to_string());
        }

        let func = &args[0];
        let func_args = &args[1..];

        match func {
            Expr::Symbol(name) => self.apply_builtin(name, func_args),
            Expr::List(lambda)
                if lambda.len() == 3 && lambda[0] == Expr::Symbol("lambda".to_string()) =>
            {
                self.apply_lambda(lambda, func_args)
            }
            _ => Err("funcall requires a function as first argument".to_string()),
        }
    }

    pub fn builtin_print(&mut self, args: &[Expr]) -> Result<Expr, String> {
        use std::io::{self, Write};
        for arg in args {
            print!("{}", self.format_expr_for_print(arg));
        }
        io::stdout().flush().unwrap();
        Ok(args.last().cloned().unwrap_or(Expr::List(vec![])))
    }

    pub fn builtin_println(&mut self, args: &[Expr]) -> Result<Expr, String> {
        for arg in args {
            println!("{}", self.format_expr_for_print(arg));
        }
        Ok(args.last().cloned().unwrap_or(Expr::List(vec![])))
    }

    pub fn format_expr_for_print(&self, expr: &Expr) -> String {
        match expr {
            Expr::Number(n) => {
                if n.fract() == 0.0 && n.abs() < 1e10 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Expr::Symbol(s) => s.clone(),
            Expr::String(s) => s.clone(), // Print strings without quotes
            Expr::List(list) => {
                let items: Vec<String> =
                    list.iter().map(|e| self.format_expr_for_print(e)).collect();
                format!("({})", items.join(" "))
            }
        }
    }

    pub fn apply_lambda(&mut self, lambda: &[Expr], args: &[Expr]) -> Result<Expr, String> {
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
