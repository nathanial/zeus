use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::types::{EvalError, EvalResult, Expr, HashKey, SymbolData};
use std::char;
use std::collections::HashMap;
use std::rc::Rc;

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
            "/=" => self.builtin_not_equal(args),
            "<" => self.builtin_less(args),
            "<=" => self.builtin_less_equal(args),
            ">" => self.builtin_greater(args),
            ">=" => self.builtin_greater_equal(args),

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

            // Vector operations
            "vector" => Ok(Expr::Vector(args.to_vec())),
            "make-vector" => self.builtin_make_vector(args),
            "vector-ref" => self.builtin_vector_ref(args),
            "vector-set!" => self.builtin_vector_set(args),
            "vector-length" => self.builtin_vector_length(args),

            // Hash table operations
            "make-hash-table" => self.builtin_make_hash_table(args),
            "hash-set!" => self.builtin_hash_set(args),
            "hash-ref" => self.builtin_hash_ref(args),
            "hash-remove!" => self.builtin_hash_remove(args),
            "hash-keys" => self.builtin_hash_keys(args),

            // Character operations
            "char=" => self.builtin_char_equal(args),
            "char<" => self.builtin_char_less(args),
            "char>" => self.builtin_char_greater(args),
            "char->integer" => self.builtin_char_to_integer(args),
            "integer->char" => self.builtin_integer_to_char(args),

            // Type predicates
            "integerp" => self.builtin_integerp(args),
            "floatp" => self.builtin_floatp(args),
            "rationalp" => self.builtin_rationalp(args),
            "numberp" => self.builtin_numberp(args),
            "characterp" => self.builtin_characterp(args),
            "vectorp" => self.builtin_vectorp(args),
            "hash-table-p" => self.builtin_hash_table_p(args),

            _ => Err(EvalError::message(format!("Unknown function: {}", name))),
        }
    }

    fn builtin_add(&mut self, args: &[Expr]) -> EvalResult {
        let mut sum = 0.0;
        let mut all_integers = true;

        for arg in args {
            let n = Self::to_number(arg).map_err(|_| {
                EvalError::message(format!("+ requires numeric arguments, got {:?}", arg))
            })?;
            sum += n;
            if !matches!(arg, Expr::Integer(_)) {
                all_integers = false;
            }
        }

        if all_integers && sum.fract() == 0.0 {
            Ok(Expr::Integer(sum as i64))
        } else {
            Ok(Expr::Float(sum))
        }
    }

    fn builtin_subtract(&mut self, args: &[Expr]) -> EvalResult {
        if args.is_empty() {
            return Err(EvalError::message("- requires at least 1 argument"));
        }

        let first = Self::to_number(&args[0]).map_err(|_| {
            EvalError::message(format!("- requires numeric arguments, got {:?}", args[0]))
        })?;

        if args.len() == 1 {
            return match &args[0] {
                Expr::Integer(n) => Ok(Expr::Integer(-n)),
                _ => Ok(Expr::Float(-first)),
            };
        }

        let mut result = first;
        let mut all_integers = matches!(&args[0], Expr::Integer(_));

        for arg in &args[1..] {
            let n = Self::to_number(arg).map_err(|_| {
                EvalError::message(format!("- requires numeric arguments, got {:?}", arg))
            })?;
            result -= n;
            if !matches!(arg, Expr::Integer(_)) {
                all_integers = false;
            }
        }

        if all_integers && result.fract() == 0.0 {
            Ok(Expr::Integer(result as i64))
        } else {
            Ok(Expr::Float(result))
        }
    }

    fn builtin_multiply(&mut self, args: &[Expr]) -> EvalResult {
        let mut product = 1.0;
        let mut all_integers = true;

        for arg in args {
            let n = Self::to_number(arg).map_err(|_| {
                EvalError::message(format!("* requires numeric arguments, got {:?}", arg))
            })?;
            product *= n;
            if !matches!(arg, Expr::Integer(_)) {
                all_integers = false;
            }
        }

        if all_integers && product.fract() == 0.0 {
            Ok(Expr::Integer(product as i64))
        } else {
            Ok(Expr::Float(product))
        }
    }

    fn builtin_divide(&mut self, args: &[Expr]) -> EvalResult {
        if args.is_empty() {
            return Err(EvalError::message("/ requires at least 1 argument"));
        }

        let first = Self::to_number(&args[0]).map_err(|_| {
            EvalError::message(format!("/ requires numeric arguments, got {:?}", args[0]))
        })?;

        if first == 0.0 && args.len() == 1 {
            return Err(EvalError::message("Division by zero"));
        }

        if args.len() == 1 {
            return Ok(Expr::Float(1.0 / first));
        }

        let mut result = first;
        for arg in &args[1..] {
            let n = Self::to_number(arg).map_err(|_| {
                EvalError::message(format!("/ requires numeric arguments, got {:?}", arg))
            })?;
            if n == 0.0 {
                return Err(EvalError::message("Division by zero"));
            }
            result /= n;
        }
        Ok(Expr::Float(result))
    }

    fn builtin_equal(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() < 2 {
            return Err(EvalError::message("= requires at least 2 arguments"));
        }

        let all_numeric = args.iter().all(|arg| {
            matches!(
                arg,
                Expr::Integer(_) | Expr::Float(_) | Expr::Rational { .. }
            )
        });

        if all_numeric {
            let numbers = Self::collect_numeric_args(args, "=", 2)?;
            let first = numbers[0];
            let result = numbers.iter().skip(1).all(|&n| n == first);
            return Ok(Evaluator::bool_to_expr(result));
        }

        if args.len() != 2 {
            return Err(EvalError::message("= requires numeric arguments"));
        }

        let equal = match (&args[0], &args[1]) {
            (Expr::String(a), Expr::String(b)) => a == b,
            (Expr::Character(a), Expr::Character(b)) => a == b,
            (Expr::Symbol(a), Expr::Symbol(b)) => a == b,
            _ => false,
        };

        Ok(Evaluator::bool_to_expr(equal))
    }

    fn builtin_not_equal(&mut self, args: &[Expr]) -> EvalResult {
        let numbers = Self::collect_numeric_args(args, "/=", 2)?;

        for (idx, current) in numbers.iter().enumerate() {
            if numbers[..idx].iter().any(|prev| prev == current) {
                return Ok(Evaluator::bool_to_expr(false));
            }
        }

        Ok(Evaluator::bool_to_expr(true))
    }

    fn builtin_less(&mut self, args: &[Expr]) -> EvalResult {
        let numbers = Self::collect_numeric_args(args, "<", 2)?;

        for window in numbers.windows(2) {
            if !(window[0] < window[1]) {
                return Ok(Evaluator::bool_to_expr(false));
            }
        }

        Ok(Evaluator::bool_to_expr(true))
    }

    fn builtin_less_equal(&mut self, args: &[Expr]) -> EvalResult {
        let numbers = Self::collect_numeric_args(args, "<=", 2)?;

        for window in numbers.windows(2) {
            if !(window[0] <= window[1]) {
                return Ok(Evaluator::bool_to_expr(false));
            }
        }

        Ok(Evaluator::bool_to_expr(true))
    }

    fn builtin_greater(&mut self, args: &[Expr]) -> EvalResult {
        let numbers = Self::collect_numeric_args(args, ">", 2)?;

        for window in numbers.windows(2) {
            if !(window[0] > window[1]) {
                return Ok(Evaluator::bool_to_expr(false));
            }
        }

        Ok(Evaluator::bool_to_expr(true))
    }

    fn builtin_greater_equal(&mut self, args: &[Expr]) -> EvalResult {
        let numbers = Self::collect_numeric_args(args, ">=", 2)?;

        for window in numbers.windows(2) {
            if !(window[0] >= window[1]) {
                return Ok(Evaluator::bool_to_expr(false));
            }
        }

        Ok(Evaluator::bool_to_expr(true))
    }

    fn collect_numeric_args(
        args: &[Expr],
        name: &str,
        min_len: usize,
    ) -> Result<Vec<f64>, EvalError> {
        if args.len() < min_len {
            return Err(EvalError::message(format!(
                "{} requires at least {} arguments",
                name, min_len
            )));
        }

        let mut numbers = Vec::with_capacity(args.len());
        for arg in args {
            let value = Self::to_number(arg)
                .map_err(|_| EvalError::message(format!("{} requires numeric arguments", name)))?;
            numbers.push(value);
        }

        Ok(numbers)
    }

    // Vector operations
    fn builtin_make_vector(&mut self, args: &[Expr]) -> EvalResult {
        if args.is_empty() || args.len() > 2 {
            return Err(EvalError::message("make-vector requires 1 or 2 arguments"));
        }

        let size = match &args[0] {
            Expr::Integer(n) if *n >= 0 => *n as usize,
            _ => {
                return Err(EvalError::message(
                    "make-vector requires a non-negative integer size",
                ))
            }
        };

        let init = if args.len() == 2 {
            args[1].clone()
        } else {
            Expr::Integer(0)
        };

        Ok(Expr::Vector(vec![init; size]))
    }

    fn builtin_vector_ref(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 2 {
            return Err(EvalError::message(
                "vector-ref requires exactly 2 arguments",
            ));
        }

        let vec = match &args[0] {
            Expr::Vector(v) => v,
            _ => {
                return Err(EvalError::message(
                    "vector-ref requires a vector as first argument",
                ))
            }
        };

        let index = match &args[1] {
            Expr::Integer(n) if *n >= 0 => *n as usize,
            _ => {
                return Err(EvalError::message(
                    "vector-ref requires a non-negative integer index",
                ))
            }
        };

        vec.get(index)
            .cloned()
            .ok_or_else(|| EvalError::message(format!("vector-ref: index {} out of bounds", index)))
    }

    fn builtin_vector_set(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 3 {
            return Err(EvalError::message(
                "vector-set! requires exactly 3 arguments",
            ));
        }

        let mut vec = match &args[0] {
            Expr::Vector(v) => v.clone(),
            _ => {
                return Err(EvalError::message(
                    "vector-set! requires a vector as first argument",
                ))
            }
        };

        let index = match &args[1] {
            Expr::Integer(n) if *n >= 0 => *n as usize,
            _ => {
                return Err(EvalError::message(
                    "vector-set! requires a non-negative integer index",
                ))
            }
        };

        if index >= vec.len() {
            return Err(EvalError::message(format!(
                "vector-set!: index {} out of bounds",
                index
            )));
        }

        vec[index] = args[2].clone();
        Ok(Expr::Vector(vec))
    }

    fn builtin_vector_length(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 1 {
            return Err(EvalError::message(
                "vector-length requires exactly 1 argument",
            ));
        }

        match &args[0] {
            Expr::Vector(v) => Ok(Expr::Integer(v.len() as i64)),
            _ => Err(EvalError::message("vector-length requires a vector")),
        }
    }

    // Hash table operations
    fn builtin_make_hash_table(&mut self, _args: &[Expr]) -> EvalResult {
        Ok(Expr::HashTable(Rc::new(HashMap::new())))
    }

    fn builtin_hash_set(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 3 {
            return Err(EvalError::message("hash-set! requires exactly 3 arguments"));
        }

        let mut table = match &args[0] {
            Expr::HashTable(h) => (**h).clone(),
            _ => {
                return Err(EvalError::message(
                    "hash-set! requires a hash table as first argument",
                ))
            }
        };

        let key = Self::expr_to_hashkey(&args[1])
            .ok_or_else(|| EvalError::message("hash-set! requires a hashable key"))?;

        table.insert(key, args[2].clone());
        Ok(Expr::HashTable(Rc::new(table)))
    }

    fn builtin_hash_ref(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() < 2 || args.len() > 3 {
            return Err(EvalError::message("hash-ref requires 2 or 3 arguments"));
        }

        let table = match &args[0] {
            Expr::HashTable(h) => h,
            _ => {
                return Err(EvalError::message(
                    "hash-ref requires a hash table as first argument",
                ))
            }
        };

        let key = Self::expr_to_hashkey(&args[1])
            .ok_or_else(|| EvalError::message("hash-ref requires a hashable key"))?;

        table
            .get(&key)
            .cloned()
            .or_else(|| {
                if args.len() == 3 {
                    Some(args[2].clone())
                } else {
                    None
                }
            })
            .ok_or_else(|| EvalError::message(format!("hash-ref: key not found: {:?}", args[1])))
    }

    fn builtin_hash_remove(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 2 {
            return Err(EvalError::message(
                "hash-remove! requires exactly 2 arguments",
            ));
        }

        let mut table = match &args[0] {
            Expr::HashTable(h) => (**h).clone(),
            _ => {
                return Err(EvalError::message(
                    "hash-remove! requires a hash table as first argument",
                ))
            }
        };

        let key = Self::expr_to_hashkey(&args[1])
            .ok_or_else(|| EvalError::message("hash-remove! requires a hashable key"))?;

        table.remove(&key);
        Ok(Expr::HashTable(Rc::new(table)))
    }

    fn builtin_hash_keys(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 1 {
            return Err(EvalError::message("hash-keys requires exactly 1 argument"));
        }

        let table = match &args[0] {
            Expr::HashTable(h) => h,
            _ => return Err(EvalError::message("hash-keys requires a hash table")),
        };

        let keys: Vec<Expr> = table
            .keys()
            .map(|k| match k {
                HashKey::Integer(n) => Expr::Integer(*n),
                HashKey::Symbol(s) => Expr::Symbol(SymbolData::Interned(s.clone())),
                HashKey::String(s) => Expr::String(s.clone()),
                HashKey::Character(c) => Expr::Character(*c),
                HashKey::Keyword(s) => Expr::Symbol(SymbolData::Keyword(s.clone())),
            })
            .collect();

        Ok(Expr::List(keys))
    }

    // Character operations
    fn builtin_char_equal(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 2 {
            return Err(EvalError::message("char= requires exactly 2 arguments"));
        }

        match (&args[0], &args[1]) {
            (Expr::Character(a), Expr::Character(b)) => Ok(Evaluator::bool_to_expr(a == b)),
            _ => Err(EvalError::message("char= requires character arguments")),
        }
    }

    fn builtin_char_less(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 2 {
            return Err(EvalError::message("char< requires exactly 2 arguments"));
        }

        match (&args[0], &args[1]) {
            (Expr::Character(a), Expr::Character(b)) => Ok(Evaluator::bool_to_expr(a < b)),
            _ => Err(EvalError::message("char< requires character arguments")),
        }
    }

    fn builtin_char_greater(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 2 {
            return Err(EvalError::message("char> requires exactly 2 arguments"));
        }

        match (&args[0], &args[1]) {
            (Expr::Character(a), Expr::Character(b)) => Ok(Evaluator::bool_to_expr(a > b)),
            _ => Err(EvalError::message("char> requires character arguments")),
        }
    }

    fn builtin_char_to_integer(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 1 {
            return Err(EvalError::message(
                "char->integer requires exactly 1 argument",
            ));
        }

        match &args[0] {
            Expr::Character(c) => Ok(Expr::Integer(*c as i64)),
            _ => Err(EvalError::message("char->integer requires a character")),
        }
    }

    fn builtin_integer_to_char(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 1 {
            return Err(EvalError::message(
                "integer->char requires exactly 1 argument",
            ));
        }

        match &args[0] {
            Expr::Integer(n) if *n >= 0 && *n <= 1114111 => {
                Ok(Expr::Character(char::from_u32(*n as u32).unwrap()))
            }
            _ => Err(EvalError::message(
                "integer->char requires a valid Unicode code point",
            )),
        }
    }

    // Type predicates
    fn builtin_integerp(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 1 {
            return Err(EvalError::message("integerp requires exactly 1 argument"));
        }

        Ok(Evaluator::bool_to_expr(matches!(args[0], Expr::Integer(_))))
    }

    fn builtin_floatp(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 1 {
            return Err(EvalError::message("floatp requires exactly 1 argument"));
        }

        Ok(Evaluator::bool_to_expr(matches!(args[0], Expr::Float(_))))
    }

    fn builtin_rationalp(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 1 {
            return Err(EvalError::message("rationalp requires exactly 1 argument"));
        }

        Ok(Evaluator::bool_to_expr(matches!(
            args[0],
            Expr::Rational { .. }
        )))
    }

    fn builtin_numberp(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 1 {
            return Err(EvalError::message("numberp requires exactly 1 argument"));
        }

        let is_number = matches!(
            args[0],
            Expr::Integer(_) | Expr::Float(_) | Expr::Rational { .. }
        );
        Ok(Evaluator::bool_to_expr(is_number))
    }

    fn builtin_characterp(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 1 {
            return Err(EvalError::message("characterp requires exactly 1 argument"));
        }

        Ok(Evaluator::bool_to_expr(matches!(
            args[0],
            Expr::Character(_)
        )))
    }

    fn builtin_vectorp(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 1 {
            return Err(EvalError::message("vectorp requires exactly 1 argument"));
        }

        Ok(Evaluator::bool_to_expr(matches!(args[0], Expr::Vector(_))))
    }

    fn builtin_hash_table_p(&mut self, args: &[Expr]) -> EvalResult {
        if args.len() != 1 {
            return Err(EvalError::message(
                "hash-table-p requires exactly 1 argument",
            ));
        }

        Ok(Evaluator::bool_to_expr(matches!(
            args[0],
            Expr::HashTable(_)
        )))
    }

    // Continued in evaluator_builtins_cont.rs...
}
