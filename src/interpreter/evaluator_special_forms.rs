use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::types::Expr;

impl Evaluator {
    pub fn eval_cond(&mut self, list: &[Expr]) -> Result<Expr, String> {
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

    pub fn eval_and(&mut self, list: &[Expr]) -> Result<Expr, String> {
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

    pub fn eval_or(&mut self, list: &[Expr]) -> Result<Expr, String> {
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

    pub fn eval_progn(&mut self, list: &[Expr]) -> Result<Expr, String> {
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

    pub fn eval_when(&mut self, list: &[Expr]) -> Result<Expr, String> {
        if list.len() < 2 {
            return Err("when requires at least 1 argument".to_string());
        }

        let condition = self.eval(&list[1])?;
        let is_true = match condition {
            Expr::Number(n) => n != 0.0,
            Expr::List(ref l) => !l.is_empty(),
            _ => true,
        };

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

    pub fn eval_unless(&mut self, list: &[Expr]) -> Result<Expr, String> {
        if list.len() < 2 {
            return Err("unless requires at least 1 argument".to_string());
        }

        let condition = self.eval(&list[1])?;
        let is_false = match condition {
            Expr::Number(n) => n == 0.0,
            Expr::List(ref l) => l.is_empty(),
            _ => false,
        };

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

    pub fn eval_case(&mut self, list: &[Expr]) -> Result<Expr, String> {
        if list.len() < 2 {
            return Err("case requires at least 1 argument".to_string());
        }

        let key = self.eval(&list[1])?;

        for clause in &list[2..] {
            match clause {
                Expr::List(clause_list) if !clause_list.is_empty() => {
                    let test_value = &clause_list[0];

                    // Check for else/otherwise clause
                    let is_else = match test_value {
                        Expr::Symbol(s) if s == "else" || s == "otherwise" => true,
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
                _ => return Err("case clause must be a non-empty list".to_string()),
            }
        }

        // No case matched
        Ok(Expr::List(vec![]))
    }

    pub fn expr_equal(&self, a: &Expr, b: &Expr) -> bool {
        match (a, b) {
            (Expr::Number(x), Expr::Number(y)) => (x - y).abs() < f64::EPSILON,
            (Expr::String(x), Expr::String(y)) => x == y,
            (Expr::Symbol(x), Expr::Symbol(y)) => x == y,
            (Expr::List(x), Expr::List(y)) => {
                x.len() == y.len() && x.iter().zip(y.iter()).all(|(a, b)| self.expr_equal(a, b))
            }
            _ => false,
        }
    }

    pub fn eval_application(&mut self, list: &[Expr]) -> Result<Expr, String> {
        let func = self.eval(&list[0])?;
        let args: Result<Vec<_>, _> = list[1..].iter().map(|e| self.eval(e)).collect();
        let args = args?;

        match func {
            Expr::Symbol(name) => self.apply_builtin(&name, &args),
            Expr::List(lambda)
                if lambda.len() == 3 && lambda[0] == Expr::Symbol("lambda".to_string()) =>
            {
                self.apply_lambda(&lambda, &args)
            }
            _ => Err(format!("Cannot apply: {:?}", func)),
        }
    }
}
