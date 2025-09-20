#[cfg(test)]
mod tests {
    use crate::interpreter::*;

    fn eval_to_number(input: &str) -> f64 {
        match Evaluator::eval_once(input).unwrap() {
            Expr::Number(n) => n,
            other => panic!("Expected number, got {:?}", other),
        }
    }

    fn eval_to_string(input: &str) -> String {
        match Evaluator::eval_once(input).unwrap() {
            Expr::String(s) => s,
            other => panic!("Expected string, got {:?}", other),
        }
    }

    fn eval_to_list(input: &str) -> Vec<Expr> {
        match Evaluator::eval_once(input).unwrap() {
            Expr::List(l) => l,
            other => panic!("Expected list, got {:?}", other),
        }
    }

    #[test]
    fn test_tokenizer_numbers() {
        let mut tokenizer = Tokenizer::new("42 3.14 -17 -2.5");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::Number(42.0),
            Token::Number(3.14),
            Token::Number(-17.0),
            Token::Number(-2.5),
        ]);
    }

    #[test]
    fn test_tokenizer_symbols() {
        let mut tokenizer = Tokenizer::new("+ define lambda x y");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::Symbol("+".to_string()),
            Token::Symbol("define".to_string()),
            Token::Symbol("lambda".to_string()),
            Token::Symbol("x".to_string()),
            Token::Symbol("y".to_string()),
        ]);
    }

    #[test]
    fn test_tokenizer_strings() {
        let mut tokenizer = Tokenizer::new(r#""hello" "world" "with \"quotes\"""#);
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::String("hello".to_string()),
            Token::String("world".to_string()),
            Token::String("with \"quotes\"".to_string()),
        ]);
    }

    #[test]
    fn test_tokenizer_lists() {
        let mut tokenizer = Tokenizer::new("(+ 1 2) (define x 10)");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::LeftParen,
            Token::Symbol("+".to_string()),
            Token::Number(1.0),
            Token::Number(2.0),
            Token::RightParen,
            Token::LeftParen,
            Token::Symbol("define".to_string()),
            Token::Symbol("x".to_string()),
            Token::Number(10.0),
            Token::RightParen,
        ]);
    }

    #[test]
    fn test_parser_number() {
        let tokens = vec![Token::Number(42.0)];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        assert_eq!(expr, Expr::Number(42.0));
    }

    #[test]
    fn test_parser_symbol() {
        let tokens = vec![Token::Symbol("x".to_string())];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        assert_eq!(expr, Expr::Symbol("x".to_string()));
    }

    #[test]
    fn test_parser_string() {
        let tokens = vec![Token::String("hello".to_string())];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        assert_eq!(expr, Expr::String("hello".to_string()));
    }

    #[test]
    fn test_parser_list() {
        let tokens = vec![
            Token::LeftParen,
            Token::Symbol("+".to_string()),
            Token::Number(1.0),
            Token::Number(2.0),
            Token::RightParen,
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        assert_eq!(expr, Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Number(1.0),
            Expr::Number(2.0),
        ]));
    }

    #[test]
    fn test_parser_nested_list() {
        let tokens = vec![
            Token::LeftParen,
            Token::Symbol("+".to_string()),
            Token::LeftParen,
            Token::Symbol("*".to_string()),
            Token::Number(2.0),
            Token::Number(3.0),
            Token::RightParen,
            Token::Number(4.0),
            Token::RightParen,
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        assert_eq!(expr, Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::List(vec![
                Expr::Symbol("*".to_string()),
                Expr::Number(2.0),
                Expr::Number(3.0),
            ]),
            Expr::Number(4.0),
        ]));
    }

    #[test]
    fn test_eval_number() {
        assert_eq!(eval_to_number("42"), 42.0);
        assert_eq!(eval_to_number("3.14"), 3.14);
        assert_eq!(eval_to_number("-17"), -17.0);
    }

    #[test]
    fn test_eval_string() {
        assert_eq!(eval_to_string(r#""hello""#), "hello");
        assert_eq!(eval_to_string(r#""world""#), "world");
    }

    #[test]
    fn test_eval_addition() {
        assert_eq!(eval_to_number("(+ 1 2)"), 3.0);
        assert_eq!(eval_to_number("(+ 1 2 3)"), 6.0);
        assert_eq!(eval_to_number("(+ 10 20 30 40)"), 100.0);
        assert_eq!(eval_to_number("(+)"), 0.0);
    }

    #[test]
    fn test_eval_subtraction() {
        assert_eq!(eval_to_number("(- 5 2)"), 3.0);
        assert_eq!(eval_to_number("(- 10 3 2)"), 5.0);
        assert_eq!(eval_to_number("(- 5)"), -5.0);
    }

    #[test]
    fn test_eval_multiplication() {
        assert_eq!(eval_to_number("(* 2 3)"), 6.0);
        assert_eq!(eval_to_number("(* 2 3 4)"), 24.0);
        assert_eq!(eval_to_number("(*)"), 1.0);
    }

    #[test]
    fn test_eval_division() {
        assert_eq!(eval_to_number("(/ 10 2)"), 5.0);
        assert_eq!(eval_to_number("(/ 20 2 2)"), 5.0);
        assert_eq!(eval_to_number("(/ 2)"), 0.5);
    }

    #[test]
    fn test_eval_division_by_zero() {
        let result = Evaluator::eval_once("(/ 10 0)");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Division by zero"));
    }

    #[test]
    fn test_eval_comparison() {
        assert_eq!(eval_to_number("(= 5 5)"), 1.0);
        assert_eq!(eval_to_number("(= 5 3)"), 0.0);
        assert_eq!(eval_to_number("(< 3 5)"), 1.0);
        assert_eq!(eval_to_number("(< 5 3)"), 0.0);
        assert_eq!(eval_to_number("(> 5 3)"), 1.0);
        assert_eq!(eval_to_number("(> 3 5)"), 0.0);
    }

    #[test]
    fn test_eval_nested_arithmetic() {
        assert_eq!(eval_to_number("(+ 1 (* 2 3))"), 7.0);
        assert_eq!(eval_to_number("(* (+ 1 2) (- 5 2))"), 9.0);
        assert_eq!(eval_to_number("(/ (+ 10 20) (- 10 4))"), 5.0);
    }

    #[test]
    fn test_eval_define_and_use() {
        let mut evaluator = Evaluator::new();
        evaluator.eval_str("(define x 10)").unwrap();
        let result = evaluator.eval_str("(+ x 5)").unwrap();
        assert_eq!(result, Expr::Number(15.0));
    }

    #[test]
    fn test_eval_if_true() {
        assert_eq!(eval_to_number("(if 1 10 20)"), 10.0);
        assert_eq!(eval_to_number("(if (> 5 3) 100 200)"), 100.0);
    }

    #[test]
    fn test_eval_if_false() {
        assert_eq!(eval_to_number("(if 0 10 20)"), 20.0);
        assert_eq!(eval_to_number("(if (< 5 3) 100 200)"), 200.0);
    }

    #[test]
    fn test_eval_quote() {
        let result = Evaluator::eval_once("(quote (+ 1 2))").unwrap();
        assert_eq!(result, Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Number(1.0),
            Expr::Number(2.0),
        ]));

        let result = Evaluator::eval_once("(quote x)").unwrap();
        assert_eq!(result, Expr::Symbol("x".to_string()));
    }

    #[test]
    fn test_eval_list() {
        let result = eval_to_list("(list 1 2 3)");
        assert_eq!(result, vec![
            Expr::Number(1.0),
            Expr::Number(2.0),
            Expr::Number(3.0),
        ]);

        let result = eval_to_list("(list)");
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_eval_car() {
        assert_eq!(eval_to_number("(car (list 1 2 3))"), 1.0);

        let result = Evaluator::eval_once("(car (list))");
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_cdr() {
        let result = eval_to_list("(cdr (list 1 2 3))");
        assert_eq!(result, vec![
            Expr::Number(2.0),
            Expr::Number(3.0),
        ]);

        let result = eval_to_list("(cdr (list))");
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_eval_cons() {
        let result = eval_to_list("(cons 0 (list 1 2))");
        assert_eq!(result, vec![
            Expr::Number(0.0),
            Expr::Number(1.0),
            Expr::Number(2.0),
        ]);

        let result = eval_to_list("(cons 1 2)");
        assert_eq!(result, vec![
            Expr::Number(1.0),
            Expr::Number(2.0),
        ]);
    }

    #[test]
    fn test_eval_lambda_simple() {
        let mut evaluator = Evaluator::new();
        evaluator.eval_str("(define square (lambda (n) (* n n)))").unwrap();
        let result = evaluator.eval_str("(square 5)").unwrap();
        assert_eq!(result, Expr::Number(25.0));
    }

    #[test]
    fn test_eval_lambda_multiple_params() {
        let mut evaluator = Evaluator::new();
        evaluator.eval_str("(define add (lambda (x y) (+ x y)))").unwrap();
        let result = evaluator.eval_str("(add 3 7)").unwrap();
        assert_eq!(result, Expr::Number(10.0));
    }

    #[test]
    fn test_eval_lambda_closure() {
        let mut evaluator = Evaluator::new();
        evaluator.eval_str("(define x 10)").unwrap();
        evaluator.eval_str("(define add-x (lambda (y) (+ x y)))").unwrap();
        let result = evaluator.eval_str("(add-x 5)").unwrap();
        assert_eq!(result, Expr::Number(15.0));
    }

    #[test]
    fn test_eval_immediate_lambda() {
        assert_eq!(eval_to_number("((lambda (x) (* x 2)) 5)"), 10.0);
        assert_eq!(eval_to_number("((lambda (x y) (+ x y)) 3 4)"), 7.0);
    }

    #[test]
    fn test_error_undefined_variable() {
        let result = Evaluator::eval_once("undefined_var");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }

    #[test]
    fn test_error_invalid_application() {
        let result = Evaluator::eval_once("(123 456)");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Cannot apply"));
    }

    #[test]
    fn test_error_wrong_arg_count() {
        let result = Evaluator::eval_once("(+ 1)"); // This should work
        assert!(result.is_ok());

        let result = Evaluator::eval_once("(= 1)"); // = requires exactly 2 args
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires exactly 2 arguments"));
    }

    #[test]
    fn test_empty_list() {
        let result = eval_to_list("()");
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_complex_expression() {
        let mut evaluator = Evaluator::new();
        evaluator.eval_str("(define fact3 (lambda (n) (* n 2 1)))").unwrap();
        let result = evaluator.eval_str("(+ (fact3 3) (- 10 4))").unwrap();
        assert_eq!(result, Expr::Number(12.0)); // 6 + 6
    }

    #[test]
    fn test_repl_integration() {
        let mut repl = Repl::new();

        // Test basic arithmetic
        let result = repl.evaluate("(+ 1 2 3)").unwrap();
        assert_eq!(result, Expr::Number(6.0));

        // Test define and use
        repl.evaluate("(define x 10)").unwrap();
        let result = repl.evaluate("(* x 2)").unwrap();
        assert_eq!(result, Expr::Number(20.0));

        // Test lambda
        repl.evaluate("(define square (lambda (n) (* n n)))").unwrap();
        let result = repl.evaluate("(square 7)").unwrap();
        assert_eq!(result, Expr::Number(49.0));
    }

    #[test]
    fn test_format_expr() {
        let repl = Repl::new();

        assert_eq!(repl.format_expr(&Expr::Number(42.0)), "42");
        assert_eq!(repl.format_expr(&Expr::Number(3.14)), "3.14");
        assert_eq!(repl.format_expr(&Expr::Symbol("x".to_string())), "x");
        assert_eq!(repl.format_expr(&Expr::String("hello".to_string())), "\"hello\"");

        let list = Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Number(1.0),
            Expr::Number(2.0),
        ]);
        assert_eq!(repl.format_expr(&list), "(+ 1 2)");
    }

    // Tests for let and let*
    #[test]
    fn test_let_basic() {
        assert_eq!(eval_to_number("(let ((x 10)) x)"), 10.0);
        assert_eq!(eval_to_number("(let ((x 10) (y 20)) (+ x y))"), 30.0);
    }

    #[test]
    fn test_let_shadowing() {
        let mut evaluator = Evaluator::new();
        evaluator.eval_str("(define x 5)").unwrap();
        let result = evaluator.eval_str("(let ((x 10)) x)").unwrap();
        assert_eq!(result, Expr::Number(10.0));
        // Original x should still be 5
        let result = evaluator.eval_str("x").unwrap();
        assert_eq!(result, Expr::Number(5.0));
    }

    #[test]
    fn test_let_parallel_binding() {
        // In let, all bindings are evaluated in parallel before any are bound
        let mut evaluator = Evaluator::new();
        evaluator.eval_str("(define x 10)").unwrap();
        let result = evaluator.eval_str("(let ((x 20) (y x)) y)").unwrap();
        assert_eq!(result, Expr::Number(10.0)); // y gets the outer x, not the new x
    }

    #[test]
    fn test_let_star_sequential_binding() {
        // In let*, bindings are evaluated sequentially
        assert_eq!(eval_to_number("(let* ((x 10) (y x)) y)"), 10.0);
        assert_eq!(eval_to_number("(let* ((x 10) (y (* x 2)) (z (+ x y))) z)"), 30.0);
    }

    #[test]
    fn test_let_multiple_body_expressions() {
        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_str("(let ((x 10)) (define y 20) (+ x y))").unwrap();
        assert_eq!(result, Expr::Number(30.0));
    }

    // Tests for cond
    #[test]
    fn test_cond_basic() {
        assert_eq!(eval_to_number("(cond ((> 3 2) 10) ((< 3 2) 20))"), 10.0);
        assert_eq!(eval_to_number("(cond ((< 3 2) 10) ((> 3 2) 20))"), 20.0);
    }

    #[test]
    fn test_cond_else() {
        assert_eq!(eval_to_number("(cond ((< 3 2) 10) (else 20))"), 20.0);
        assert_eq!(eval_to_number("(cond ((> 3 2) 10) (else 20))"), 10.0);
    }

    #[test]
    fn test_cond_no_match() {
        let result = eval_to_list("(cond ((< 3 2) 10) ((< 4 3) 20))");
        assert_eq!(result, vec![]); // Returns empty list when no condition matches
    }

    #[test]
    fn test_cond_multiple_expressions() {
        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_str(
            "(cond ((> 3 2) (define x 10) (+ x 5)) (else 0))"
        ).unwrap();
        assert_eq!(result, Expr::Number(15.0));
    }

    #[test]
    fn test_cond_returns_condition_value() {
        // When there's no body, cond returns the value of the condition
        assert_eq!(eval_to_number("(cond ((+ 2 3)))"), 5.0);
    }

    // Tests for and/or
    #[test]
    fn test_and_basic() {
        assert_eq!(eval_to_number("(and)"), 1.0); // No args returns true
        assert_eq!(eval_to_number("(and 1)"), 1.0);
        assert_eq!(eval_to_number("(and 1 2)"), 2.0); // Returns last value
        assert_eq!(eval_to_number("(and 1 2 3)"), 3.0);
    }

    #[test]
    fn test_and_short_circuit() {
        assert_eq!(eval_to_number("(and 1 0 3)"), 0.0);
        assert_eq!(eval_to_number("(and 0 (/ 1 0))"), 0.0); // Doesn't eval second arg
    }

    #[test]
    fn test_and_returns_last_truthy() {
        assert_eq!(eval_to_string("(and 1 2 \"hello\")"), "hello");
        let result = eval_to_list("(and 1 (list 1 2 3))");
        assert_eq!(result, vec![Expr::Number(1.0), Expr::Number(2.0), Expr::Number(3.0)]);
    }

    #[test]
    fn test_or_basic() {
        assert_eq!(eval_to_number("(or)"), 0.0); // No args returns false
        assert_eq!(eval_to_number("(or 0)"), 0.0);
        assert_eq!(eval_to_number("(or 0 2)"), 2.0); // Returns first truthy
        assert_eq!(eval_to_number("(or 0 0 3)"), 3.0);
    }

    #[test]
    fn test_or_short_circuit() {
        assert_eq!(eval_to_number("(or 1 (/ 1 0))"), 1.0); // Doesn't eval second arg
        assert_eq!(eval_to_number("(or 0 5 10)"), 5.0); // Returns first truthy
    }

    #[test]
    fn test_or_returns_first_truthy() {
        assert_eq!(eval_to_string("(or 0 \"hello\" \"world\")"), "hello");
        let result = eval_to_list("(or 0 (list 1 2) (list 3 4))");
        assert_eq!(result, vec![Expr::Number(1.0), Expr::Number(2.0)]);
    }

    // Tests for list operations
    #[test]
    fn test_append() {
        let result = eval_to_list("(append (list 1 2) (list 3 4))");
        assert_eq!(result, vec![
            Expr::Number(1.0), Expr::Number(2.0),
            Expr::Number(3.0), Expr::Number(4.0)
        ]);

        let result = eval_to_list("(append (list 1) (list 2) (list 3))");
        assert_eq!(result, vec![
            Expr::Number(1.0), Expr::Number(2.0), Expr::Number(3.0)
        ]);

        let result = eval_to_list("(append)");
        assert_eq!(result, vec![]); // Empty append returns empty list
    }

    #[test]
    fn test_reverse() {
        let result = eval_to_list("(reverse (list 1 2 3))");
        assert_eq!(result, vec![
            Expr::Number(3.0), Expr::Number(2.0), Expr::Number(1.0)
        ]);

        let result = eval_to_list("(reverse (list))");
        assert_eq!(result, vec![]); // Reverse of empty is empty
    }

    #[test]
    fn test_length() {
        assert_eq!(eval_to_number("(length (list 1 2 3))"), 3.0);
        assert_eq!(eval_to_number("(length (list))"), 0.0);
        assert_eq!(eval_to_number("(length \"hello\")"), 5.0); // Works on strings too
    }

    #[test]
    fn test_nth() {
        assert_eq!(eval_to_number("(nth 0 (list 10 20 30))"), 10.0);
        assert_eq!(eval_to_number("(nth 1 (list 10 20 30))"), 20.0);
        assert_eq!(eval_to_number("(nth 2 (list 10 20 30))"), 30.0);
    }

    #[test]
    fn test_nth_out_of_bounds() {
        let result = Evaluator::eval_once("(nth 5 (list 1 2 3))");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("out of bounds"));
    }

    // Integration tests
    #[test]
    fn test_complex_let_and_cond() {
        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_str(
            "(let ((x 10) (y 20))
               (cond ((> x y) \"x is greater\")
                     ((< x y) \"y is greater\")
                     (else \"equal\")))"
        ).unwrap();
        assert_eq!(result, Expr::String("y is greater".to_string()));
    }

    #[test]
    fn test_nested_let() {
        assert_eq!(eval_to_number(
            "(let ((x 10))
               (let ((y 20))
                 (+ x y)))"
        ), 30.0);
    }

    #[test]
    fn test_let_with_lambda() {
        assert_eq!(eval_to_number(
            "(let ((f (lambda (x) (* x 2))))
               (f 5))"
        ), 10.0);
    }

    #[test]
    fn test_and_or_combination() {
        assert_eq!(eval_to_number("(and (or 0 5) (or 10 0))"), 10.0);
        assert_eq!(eval_to_number("(or (and 0 5) (and 10 20))"), 20.0);
    }

    #[test]
    fn test_list_operations_combination() {
        let result = eval_to_list("(reverse (append (list 1 2) (list 3 4)))");
        assert_eq!(result, vec![
            Expr::Number(4.0), Expr::Number(3.0),
            Expr::Number(2.0), Expr::Number(1.0)
        ]);

        assert_eq!(eval_to_number("(length (append (list 1 2) (list 3 4 5)))"), 5.0);
    }
}