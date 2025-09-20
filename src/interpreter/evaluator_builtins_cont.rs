use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::types::{EvalError, EvalResult, Expr, SymbolData};

impl Evaluator {
    // Basic list operations
    pub fn builtin_car(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 1 {
            return Err(EvalError::message("car requires exactly 1 argument"));
        }

        match &args[0] {
            Expr::List(list) if !list.is_empty() => Ok(list[0].clone()),
            Expr::List(_) => Ok(Expr::List(vec![])),
            Expr::Cons(car, _) => Ok((**car).clone()),
            _ => Err(EvalError::message("car requires a list or cons cell")),
        }
    }

    pub fn builtin_cdr(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 1 {
            return Err(EvalError::message("cdr requires exactly 1 argument"));
        }

        match &args[0] {
            Expr::List(list) if !list.is_empty() => Ok(Expr::List(list[1..].to_vec())),
            Expr::List(_) => Ok(Expr::List(vec![])),
            Expr::Cons(_, cdr) => Ok((**cdr).clone()),
            _ => Err(EvalError::message("cdr requires a list or cons cell")),
        }
    }

    pub fn builtin_cons(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 2 {
            return Err(EvalError::message("cons requires exactly 2 arguments"));
        }

        match &args[1] {
            Expr::List(list) => {
                let mut new_list = vec![args[0].clone()];
                new_list.extend_from_slice(list);
                Ok(Expr::List(new_list))
            }
            other => Ok(Expr::Cons(
                Box::new(args[0].clone()),
                Box::new(other.clone()),
            )),
        }
    }

    pub fn builtin_append(&mut self, args: &[Expr]) -> EvalResult {
        let mut result = Vec::new();

        for arg in args {
            match arg {
                Expr::List(list) => result.extend_from_slice(list),
                _ => return Err(EvalError::message("append requires list arguments")),
            }
        }

        Ok(Expr::List(result))
    }

    pub fn builtin_reverse(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 1 {
            return Err(EvalError::message("reverse requires exactly 1 argument"));
        }

        match &args[0] {
            Expr::List(list) => {
                let mut reversed = list.clone();
                reversed.reverse();
                Ok(Expr::List(reversed))
            }
            _ => Err(EvalError::message("reverse requires a list")),
        }
    }

    pub fn builtin_length(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 1 {
            return Err(EvalError::message("length requires exactly 1 argument"));
        }

        match &args[0] {
            Expr::List(list) => Ok(Expr::Integer(list.len() as i64)),
            Expr::Vector(vec) => Ok(Expr::Integer(vec.len() as i64)),
            Expr::String(s) => Ok(Expr::Integer(s.len() as i64)),
            _ => Err(EvalError::message(
                "length requires a list, vector, or string",
            )),
        }
    }
    pub fn builtin_nth(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 2 {
            return Err(EvalError::message("nth requires exactly 2 arguments"));
        }

        let index = match &args[0] {
            Expr::Integer(n) if *n >= 0 => *n as usize,
            _ => {
                return Err(EvalError::message(
                    "nth index must be a non-negative integer",
                ))
            }
        };

        match &args[1] {
            Expr::List(list) => Ok(list
                .get(index)
                .cloned()
                .ok_or_else(|| EvalError::message("nth index out of bounds"))?),
            _ => Err(EvalError::message("nth requires a list as second argument")),
        }
    }

    pub fn builtin_nthcdr(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 2 {
            return Err(EvalError::message("nthcdr requires exactly 2 arguments"));
        }

        let n = match &args[0] {
            Expr::Integer(num) if *num >= 0 => *num as usize,
            _ => {
                return Err(EvalError::message(
                    "nthcdr index must be a non-negative integer",
                ))
            }
        };

        match &args[1] {
            Expr::List(list) => {
                if n >= list.len() {
                    Ok(Expr::List(vec![]))
                } else {
                    Ok(Expr::List(list[n..].to_vec()))
                }
            }
            _ => Err(EvalError::message(
                "nthcdr requires a list as second argument",
            )),
        }
    }

    pub fn builtin_member(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 2 {
            return Err(EvalError::message("member requires exactly 2 arguments"));
        }

        let item = &args[0];
        let list = match &args[1] {
            Expr::List(l) => l,
            _ => {
                return Err(EvalError::message(
                    "member requires a list as second argument",
                ))
            }
        };

        for (i, elem) in list.iter().enumerate() {
            if self.expr_equal(item, elem) {
                return Ok(Expr::List(list[i..].to_vec()));
            }
        }

        Ok(Expr::List(vec![])) // Not found returns empty list
    }

    pub fn builtin_mapcar(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() < 2 {
            return Err(EvalError::message("mapcar requires at least 2 arguments"));
        }

        let func = &args[0];
        let mut lists: Vec<&Vec<Expr>> = Vec::new();
        for arg in &args[1..] {
            match arg {
                Expr::List(l) => lists.push(l),
                _ => return Err(EvalError::message("mapcar requires list arguments")),
            }
        }

        if lists.is_empty() {
            return Ok(Expr::List(vec![]));
        }

        let min_len = lists.iter().map(|l| l.len()).min().unwrap_or(0);
        let mut result = Vec::new();

        for i in 0..min_len {
            let func_args: Vec<Expr> = lists.iter().map(|l| l[i].clone()).collect();

            let val = match func {
                Expr::Symbol(sym_data) => self.apply_builtin(sym_data.name(), &func_args)?,
                Expr::List(lambda)
                    if lambda.len() == 3
                        && matches!(&lambda[0], Expr::Symbol(sym_data) if sym_data.name() == "lambda") =>
                {
                    self.apply_lambda(lambda, &func_args)?
                }
                _ => {
                    return Err(EvalError::message(
                        "mapcar requires a function as first argument",
                    ))
                }
            };
            result.push(val);
        }

        Ok(Expr::List(result))
    }

    pub fn builtin_filter(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 2 {
            return Err(EvalError::message("filter requires exactly 2 arguments"));
        }

        let pred = &args[0];
        let list = match &args[1] {
            Expr::List(l) => l,
            _ => {
                return Err(EvalError::message(
                    "filter requires a list as second argument",
                ))
            }
        };

        let mut result = Vec::new();
        for item in list {
            let test_result = match pred {
                Expr::Symbol(sym_data) => self.apply_builtin(sym_data.name(), &[item.clone()])?,
                Expr::List(lambda)
                    if lambda.len() == 3
                        && matches!(&lambda[0], Expr::Symbol(sym_data) if sym_data.name() == "lambda") =>
                {
                    self.apply_lambda(lambda, &[item.clone()])?
                }
                _ => return Err(EvalError::message("filter requires a predicate function")),
            };

            let is_true = Evaluator::is_truthy(&test_result);

            if is_true {
                result.push(item.clone());
            }
        }

        Ok(Expr::List(result))
    }

    pub fn builtin_remove(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 2 {
            return Err(EvalError::message("remove requires exactly 2 arguments"));
        }

        let pred = &args[0];
        let list = match &args[1] {
            Expr::List(l) => l,
            _ => {
                return Err(EvalError::message(
                    "remove requires a list as second argument",
                ))
            }
        };

        let mut result = Vec::new();
        for item in list {
            let test_result = match pred {
                Expr::Symbol(sym_data) => self.apply_builtin(sym_data.name(), &[item.clone()])?,
                Expr::List(lambda)
                    if lambda.len() == 3
                        && matches!(&lambda[0], Expr::Symbol(sym_data) if sym_data.name() == "lambda") =>
                {
                    self.apply_lambda(lambda, &[item.clone()])?
                }
                _ => return Err(EvalError::message("remove requires a predicate function")),
            };

            let is_true = Evaluator::is_truthy(&test_result);

            if !is_true {
                // Note: opposite of filter
                result.push(item.clone());
            }
        }

        Ok(Expr::List(result))
    }

    pub fn builtin_reduce(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() < 2 || args.len() > 3 {
            return Err(EvalError::message("reduce requires 2 or 3 arguments"));
        }

        let func = &args[0];
        let list = match &args[1] {
            Expr::List(l) => l,
            _ => {
                return Err(EvalError::message(
                    "reduce requires a list as second argument",
                ))
            }
        };

        if list.is_empty() {
            if args.len() == 3 {
                return Ok(args[2].clone()); // Return initial value
            } else {
                return Err(EvalError::message(
                    "reduce of empty list with no initial value",
                ));
            }
        }

        let (mut acc, start_idx) = if args.len() == 3 {
            (args[2].clone(), 0)
        } else {
            (list[0].clone(), 1)
        };

        for item in &list[start_idx..] {
            acc = match func {
                Expr::Symbol(sym_data) => {
                    self.apply_builtin(sym_data.name(), &[acc, item.clone()])?
                }
                Expr::List(lambda)
                    if lambda.len() == 3
                        && matches!(&lambda[0], Expr::Symbol(sym_data) if sym_data.name() == "lambda") =>
                {
                    self.apply_lambda(lambda, &[acc, item.clone()])?
                }
                _ => {
                    return Err(EvalError::message(
                        "reduce requires a function as first argument",
                    ))
                }
            };
        }

        Ok(acc)
    }

    pub fn builtin_apply(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 2 {
            return Err(EvalError::message("apply requires exactly 2 arguments"));
        }

        let func = &args[0];
        let list_args = match &args[1] {
            Expr::List(l) => l.clone(),
            _ => {
                return Err(EvalError::message(
                    "apply requires a list as second argument",
                ))
            }
        };

        match func {
            Expr::Symbol(sym_data) => self.apply_builtin(sym_data.name(), &list_args),
            Expr::List(lambda)
                if lambda.len() == 3
                    && matches!(&lambda[0], Expr::Symbol(sym_data) if sym_data.name() == "lambda") =>
            {
                self.apply_lambda(lambda, &list_args)
            }
            _ => Err(EvalError::message(
                "apply requires a function as first argument",
            )),
        }
    }

    pub fn builtin_funcall(&mut self, args: &[Expr]) -> EvalResult {
        if args.is_empty() {
            return Err(EvalError::message("funcall requires at least 1 argument"));
        }

        let func = &args[0];
        let func_args = &args[1..];

        match func {
            Expr::Symbol(sym_data) => self.apply_builtin(sym_data.name(), func_args),
            Expr::List(lambda)
                if lambda.len() == 3
                    && matches!(&lambda[0], Expr::Symbol(sym_data) if sym_data.name() == "lambda") =>
            {
                self.apply_lambda(lambda, func_args)
            }
            _ => Err(EvalError::message(
                "funcall requires a function as first argument",
            )),
        }
    }

    pub fn builtin_print(&mut self, args: &[Expr]) -> EvalResult {
        use std::io::{self, Write};
        for arg in args {
            print!("{}", self.format_expr_for_print(arg));
        }
        io::stdout().flush().unwrap();
        Ok(args.last().cloned().unwrap_or(Expr::List(vec![])))
    }

    pub fn builtin_println(&mut self, args: &[Expr]) -> EvalResult {
        for arg in args {
            println!("{}", self.format_expr_for_print(arg));
        }
        Ok(args.last().cloned().unwrap_or(Expr::List(vec![])))
    }

    // Symbol operations
    pub fn builtin_gensym(&mut self, args: &[Expr]) -> EvalResult {
        let (prefix, counter_override) = if args.is_empty() {
            (String::new(), None)
        } else if args.len() == 1 {
            match &args[0] {
                Expr::String(s) => (s.clone(), None),
                Expr::Integer(n) if *n >= 0 => (String::new(), Some(*n as u64)),
                _ => {
                    return Err(EvalError::message(
                        "gensym argument must be a string prefix or non-negative integer",
                    ))
                }
            }
        } else {
            return Err(EvalError::message("gensym takes 0 or 1 arguments"));
        };

        let sym_data = self
            .environment
            .generate_gensym(prefix.as_str(), counter_override);
        Ok(Expr::Symbol(sym_data))
    }

    pub fn builtin_get(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 2 {
            return Err(EvalError::message("get requires exactly 2 arguments"));
        }

        let symbol_name = match &args[0] {
            Expr::Symbol(SymbolData::Interned(name)) => name.clone(),
            _ => {
                return Err(EvalError::message(
                    "get requires an interned symbol as first argument",
                ))
            }
        };

        let property_name = match &args[1] {
            Expr::Symbol(sym_data) => sym_data.name().to_string(),
            _ => return Err(EvalError::message("get requires a symbol as property name")),
        };

        Ok(self
            .environment
            .get_property(&symbol_name, &property_name)
            .unwrap_or_else(|| Expr::List(vec![])))
    }

    pub fn builtin_put(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 3 {
            return Err(EvalError::message("put requires exactly 3 arguments"));
        }

        let symbol_name = match &args[0] {
            Expr::Symbol(SymbolData::Interned(name)) => name.clone(),
            _ => {
                return Err(EvalError::message(
                    "put requires an interned symbol as first argument",
                ))
            }
        };

        let property_name = match &args[1] {
            Expr::Symbol(sym_data) => sym_data.name().to_string(),
            _ => return Err(EvalError::message("put requires a symbol as property name")),
        };

        let value = args[2].clone();
        self.environment
            .set_property(symbol_name, property_name, value.clone());
        Ok(value)
    }

    pub fn builtin_symbol_plist(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 1 {
            return Err(EvalError::message(
                "symbol-plist requires exactly 1 argument",
            ));
        }

        let symbol_name = match &args[0] {
            Expr::Symbol(SymbolData::Interned(name)) => name.clone(),
            _ => {
                return Err(EvalError::message(
                    "symbol-plist requires an interned symbol",
                ))
            }
        };

        let plist = self.environment.get_symbol_plist(&symbol_name);
        Ok(Expr::List(plist))
    }

    pub fn format_expr_for_print(&self, expr: &Expr) -> String {
        match expr {
            Expr::Integer(n) => format!("{}", n),
            Expr::Float(f) => format!("{}", f),
            Expr::Rational {
                numerator,
                denominator,
            } => format!("{}/{}", numerator, denominator),
            Expr::Character(ch) => format!("{}", ch),
            Expr::Symbol(sym_data) => sym_data.name().to_string(),
            Expr::String(s) => s.clone(), // Print strings without quotes
            Expr::List(list) => {
                let items: Vec<String> =
                    list.iter().map(|e| self.format_expr_for_print(e)).collect();
                format!("({})", items.join(" "))
            }
            Expr::Cons(car, cdr) => {
                let mut repr = String::from("(");
                repr.push_str(&self.format_expr_for_print(car));

                let mut tail = cdr.as_ref();
                loop {
                    match tail {
                        Expr::Cons(next_car, next_cdr) => {
                            repr.push(' ');
                            repr.push_str(&self.format_expr_for_print(next_car));
                            tail = next_cdr.as_ref();
                        }
                        Expr::List(list) => {
                            for item in list {
                                repr.push(' ');
                                repr.push_str(&self.format_expr_for_print(item));
                            }
                            repr.push(')');
                            break;
                        }
                        other => {
                            repr.push_str(" . ");
                            repr.push_str(&self.format_expr_for_print(other));
                            repr.push(')');
                            break;
                        }
                    }
                }

                repr
            }
            Expr::Vector(vec) => {
                let items: Vec<String> =
                    vec.iter().map(|e| self.format_expr_for_print(e)).collect();
                format!("[{}]", items.join(" "))
            }
            Expr::HashTable(h) => {
                format!("#<hash-table:{}>", h.len())
            }
        }
    }

    pub fn apply_lambda(&mut self, lambda: &[Expr], args: &[Expr]) -> EvalResult {
        if let Expr::List(params) = &lambda[1] {
            if params.len() != args.len() {
                return Err(EvalError::message(format!(
                    "Lambda expects {} arguments, got {}",
                    params.len(),
                    args.len()
                )));
            }

            self.environment.push_scope();

            for (param, arg) in params.iter().zip(args.iter()) {
                if let Expr::Symbol(sym_data) = param {
                    if sym_data.is_keyword() {
                        self.environment.pop_scope();
                        return Err(EvalError::message("Cannot use keyword as parameter"));
                    }
                    self.environment
                        .set(sym_data.name().to_string(), arg.clone());
                } else {
                    self.environment.pop_scope();
                    return Err(EvalError::message("Lambda parameters must be symbols"));
                }
            }

            let result = self.eval(&lambda[2]);
            self.environment.pop_scope();
            result
        } else {
            Err(EvalError::message("Lambda parameters must be a list"))
        }
    }
}
