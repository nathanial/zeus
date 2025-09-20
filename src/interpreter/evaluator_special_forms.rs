use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::types::{EvalError, EvalResult, Expr};
use std::collections::HashMap;

impl Evaluator {
    pub fn eval_cond(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() < 2 {
            return Err(EvalError::message("cond requires at least 1 clause"));
        }

        for clause in &list[1..] {
            match clause {
                Expr::List(clause_list) if !clause_list.is_empty() => {
                    let condition = &clause_list[0];

                    // Check for else clause
                    let is_else = match condition {
                        Expr::Symbol(sym_data) if sym_data.name() == "else" => true,
                        _ => false,
                    };

                    if is_else {
                        // Execute else branch
                        if clause_list.len() < 2 {
                            return Ok(Expr::Integer(1)); // else with no body returns true
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
                    let is_true = Evaluator::is_truthy(&cond_result);

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
                _ => return Err(EvalError::message("cond clause must be a non-empty list")),
            }
        }

        // No condition was true
        Ok(Expr::List(vec![]))
    }

    pub fn eval_and(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() == 1 {
            return Ok(Evaluator::bool_to_expr(true)); // (and) with no args returns true
        }

        let mut result = Evaluator::bool_to_expr(true);
        for expr in &list[1..] {
            result = self.eval(expr)?;
            let is_false = !Evaluator::is_truthy(&result);

            if is_false {
                return Ok(result);
            }
        }

        Ok(result) // Return last value if all are truthy
    }

    pub fn eval_or(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() == 1 {
            return Ok(Expr::List(vec![])); // (or) with no args returns false
        }

        for expr in &list[1..] {
            let result = self.eval(expr)?;
            let is_true = Evaluator::is_truthy(&result);

            if is_true {
                return Ok(result); // Return first truthy value
            }
        }

        Ok(Expr::List(vec![])) // All were falsy
    }

    pub fn eval_progn(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() == 1 {
            return Ok(Expr::List(vec![])); // (progn) with no args returns nil
        }

        let mut result = Ok(Expr::List(vec![]));
        for expr in &list[1..] {
            result = self.eval(expr);
            if result.is_err() {
                return result;
            }
        }
        result
    }

    pub fn eval_when(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() < 2 {
            return Err(EvalError::message("when requires at least 1 argument"));
        }

        let condition = self.eval(&list[1])?;
        let is_true = Evaluator::is_truthy(&condition);

        if is_true {
            let mut result = Ok(Expr::List(vec![]));
            for expr in &list[2..] {
                result = self.eval(expr);
                if result.is_err() {
                    return result;
                }
            }
            result
        } else {
            Ok(Expr::List(vec![]))
        }
    }

    pub fn eval_unless(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() < 2 {
            return Err(EvalError::message("unless requires at least 1 argument"));
        }

        let condition = self.eval(&list[1])?;
        let is_false = !Evaluator::is_truthy(&condition);

        if is_false {
            let mut result = Ok(Expr::List(vec![]));
            for expr in &list[2..] {
                result = self.eval(expr);
                if result.is_err() {
                    return result;
                }
            }
            result
        } else {
            Ok(Expr::List(vec![]))
        }
    }

    pub fn eval_case(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() < 2 {
            return Err(EvalError::message("case requires at least 1 argument"));
        }

        let key = self.eval(&list[1])?;

        for clause in &list[2..] {
            match clause {
                Expr::List(clause_list) if !clause_list.is_empty() => {
                    let test_value = &clause_list[0];

                    // Check for else/otherwise clause
                    let is_else = match test_value {
                        Expr::Symbol(sym_data)
                            if sym_data.name() == "else" || sym_data.name() == "otherwise" =>
                        {
                            true
                        }
                        _ => false,
                    };

                    if is_else {
                        // Execute else branch
                        let mut result = Ok(Expr::List(vec![]));
                        for expr in &clause_list[1..] {
                            result = self.eval(expr);
                            if result.is_err() {
                                return result;
                            }
                        }
                        return result;
                    }

                    // Check if key matches any value in the test list
                    let matches = match test_value {
                        Expr::List(values) => values.iter().any(|v| self.expr_equal(&key, v)),
                        single_value => self.expr_equal(&key, single_value),
                    };

                    if matches {
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
                _ => return Err(EvalError::message("case clause must be a non-empty list")),
            }
        }

        // No case matched
        Ok(Expr::List(vec![]))
    }

    pub fn expr_equal(&self, a: &Expr, b: &Expr) -> bool {
        match (a, b) {
            (x, y) if Evaluator::to_number(x).is_ok() && Evaluator::to_number(y).is_ok() => {
                let x_val = Evaluator::to_number(x).unwrap();
                let y_val = Evaluator::to_number(y).unwrap();
                (x_val - y_val).abs() < f64::EPSILON
            }
            (Expr::String(x), Expr::String(y)) => x == y,
            (Expr::Symbol(x), Expr::Symbol(y)) => x == y,
            (Expr::List(x), Expr::List(y)) => {
                x.len() == y.len() && x.iter().zip(y.iter()).all(|(a, b)| self.expr_equal(a, b))
            }
            (Expr::Cons(ax, ay), Expr::Cons(bx, by)) => {
                self.expr_equal(ax, bx) && self.expr_equal(ay, by)
            }
            _ => false,
        }
    }

    pub fn eval_do(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() < 3 {
            return Err(EvalError::message("do requires bindings and a test clause"));
        }

        let bindings = match &list[1] {
            Expr::List(items) => items,
            _ => return Err(EvalError::message("do bindings must be a list")),
        };

        let test_clause = match &list[2] {
            Expr::List(items) => items,
            _ => return Err(EvalError::message("do test clause must be a list")),
        };

        if test_clause.is_empty() {
            return Err(EvalError::message("do test clause cannot be empty"));
        }

        self.environment.push_scope();
        let result = (|| -> EvalResult {
            let mut binding_info: Vec<(String, Option<Expr>)> = Vec::new();

            for binding in bindings {
                let items = match binding {
                    Expr::List(items) if !items.is_empty() => items,
                    _ => {
                        return Err(EvalError::message(
                            "Each do binding must be a non-empty list",
                        ))
                    }
                };

                if items.len() > 3 {
                    return Err(EvalError::message(
                        "do binding may only specify name, init, and optional step",
                    ));
                }

                let name = match &items[0] {
                    Expr::Symbol(sym_data) => {
                        if sym_data.is_keyword() {
                            return Err(EvalError::message("Cannot bind to a keyword"));
                        }
                        sym_data.name().to_string()
                    }
                    _ => return Err(EvalError::message("do binding name must be a symbol")),
                };

                let init_value = if items.len() >= 2 {
                    self.eval(&items[1])?
                } else {
                    Expr::List(vec![])
                };
                self.environment.set(name.clone(), init_value);

                let step_expr = if items.len() == 3 {
                    Some(items[2].clone())
                } else {
                    None
                };
                binding_info.push((name, step_expr));
            }

            let body_forms = &list[3..];

            loop {
                let test_result = self.eval(&test_clause[0])?;
                if Evaluator::is_truthy(&test_result) {
                    if test_clause.len() == 1 {
                        return Ok(test_result);
                    }

                    let mut final_value = Expr::List(vec![]);
                    for expr in &test_clause[1..] {
                        final_value = self.eval(expr)?;
                    }
                    return Ok(final_value);
                }

                for expr in body_forms {
                    self.eval(expr)?;
                }

                let mut next_values: Vec<Option<Expr>> = Vec::with_capacity(binding_info.len());
                for (_, step_expr) in &binding_info {
                    if let Some(step) = step_expr {
                        next_values.push(Some(self.eval(step)?));
                    } else {
                        next_values.push(None);
                    }
                }

                for ((name, _), maybe_value) in binding_info.iter().zip(next_values.into_iter()) {
                    if let Some(value) = maybe_value {
                        self.environment.set(name.clone(), value);
                    }
                }
            }
        })();
        self.environment.pop_scope();
        result
    }

    pub fn eval_loop(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() >= 3
            && Self::is_do_binding_list(&list[1])
            && matches!(&list[2], Expr::List(_))
        {
            return self.eval_do(list);
        }

        if list.len() == 1 {
            return Ok(Expr::List(vec![]));
        }

        loop {
            for expr in &list[1..] {
                self.eval(expr)?;
            }
        }
    }

    pub fn eval_catch(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() < 2 {
            return Err(EvalError::message("catch requires a tag and optional body"));
        }

        let tag = self.eval(&list[1])?;
        let mut last_value = Expr::List(vec![]);

        for expr in &list[2..] {
            match self.eval(expr) {
                Ok(value) => last_value = value,
                Err(EvalError::Throw {
                    tag: thrown_tag,
                    value,
                }) => {
                    if self.expr_equal(&tag, &thrown_tag) {
                        return Ok(value);
                    } else {
                        return Err(EvalError::Throw {
                            tag: thrown_tag,
                            value,
                        });
                    }
                }
                Err(err) => return Err(err),
            }
        }

        Ok(last_value)
    }

    pub fn eval_throw(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() < 2 {
            return Err(EvalError::message("throw requires at least a tag"));
        }

        let tag = self.eval(&list[1])?;
        let value = if list.len() > 2 {
            self.eval(&list[2])?
        } else {
            Expr::List(vec![])
        };

        Err(EvalError::Throw { tag, value })
    }

    pub fn eval_unwind_protect(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() < 2 {
            return Err(EvalError::message(
                "unwind-protect requires a protected form",
            ));
        }

        let protected_result = self.eval(&list[1]);

        let cleanup_result = list[2..].iter().try_for_each(|expr| match self.eval(expr) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        });

        match cleanup_result {
            Err(err) => Err(err),
            Ok(_) => match protected_result {
                Ok(value) => Ok(value),
                Err(err) => Err(err),
            },
        }
    }

    pub fn eval_block(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() < 2 {
            return Err(EvalError::message("block requires a name"));
        }

        let name = match &list[1] {
            Expr::Symbol(sym_data) => sym_data.name().to_string(),
            _ => return Err(EvalError::message("block name must be a symbol")),
        };

        let mut last_value = Expr::List(vec![]);
        for expr in &list[2..] {
            match self.eval(expr) {
                Ok(value) => last_value = value,
                Err(EvalError::ReturnFrom {
                    name: target,
                    value,
                }) => {
                    if target == name {
                        return Ok(value);
                    } else {
                        return Err(EvalError::ReturnFrom {
                            name: target,
                            value,
                        });
                    }
                }
                Err(err) => return Err(err),
            }
        }

        Ok(last_value)
    }

    pub fn eval_return_from(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() < 2 {
            return Err(EvalError::message("return-from requires a block name"));
        }

        let name = match &list[1] {
            Expr::Symbol(sym_data) => sym_data.name().to_string(),
            _ => {
                return Err(EvalError::message(
                    "return-from block name must be a symbol",
                ))
            }
        };

        let value = if list.len() > 2 {
            self.eval(&list[2])?
        } else {
            Expr::List(vec![])
        };

        Err(EvalError::ReturnFrom { name, value })
    }

    pub fn eval_tagbody(&mut self, list: &[Expr]) -> EvalResult {
        let forms = &list[1..];

        let mut labels: HashMap<String, usize> = HashMap::new();
        for (idx, form) in forms.iter().enumerate() {
            if let Expr::Symbol(sym_data) = form {
                if !sym_data.is_keyword() {
                    labels.insert(sym_data.name().to_string(), idx);
                }
            }
        }

        let mut index = 0usize;
        while index < forms.len() {
            match &forms[index] {
                Expr::Symbol(_) => {
                    index += 1;
                }
                expr => match self.eval(expr) {
                    Ok(_) => index += 1,
                    Err(EvalError::Go { label }) => {
                        if let Some(target) = labels.get(&label) {
                            index = target + 1;
                        } else {
                            return Err(EvalError::Go { label });
                        }
                    }
                    Err(err) => return Err(err),
                },
            }
        }

        Ok(Expr::List(vec![]))
    }

    pub fn eval_go(&mut self, list: &[Expr]) -> EvalResult {
        if list.len() != 2 {
            return Err(EvalError::message("go requires exactly one label"));
        }

        let label = match &list[1] {
            Expr::Symbol(sym_data) => sym_data.name().to_string(),
            _ => return Err(EvalError::message("go label must be a symbol")),
        };

        Err(EvalError::Go { label })
    }

    fn is_do_binding_list(expr: &Expr) -> bool {
        match expr {
            Expr::List(items) => items.iter().all(|item| match item {
                Expr::List(inner) if !inner.is_empty() => true,
                _ => false,
            }),
            _ => false,
        }
    }

    pub fn eval_application(&mut self, list: &[Expr]) -> EvalResult {
        let func = self.eval(&list[0])?;
        let args: Result<Vec<_>, _> = list[1..].iter().map(|e| self.eval(e)).collect();
        let args = args?;

        match func {
            Expr::Symbol(sym_data) => self.apply_builtin(sym_data.name(), &args),
            Expr::List(lambda)
                if lambda.len() == 3
                    && matches!(&lambda[0], Expr::Symbol(sym_data) if sym_data.name() == "lambda") =>
            {
                self.apply_lambda(&lambda, &args)
            }
            _ => Err(EvalError::message(format!("Cannot apply: {:?}", func))),
        }
    }
}
