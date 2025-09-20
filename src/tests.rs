#[cfg(test)]
mod tests {
    use crate::interpreter::*;

    fn eval_expr(input: &str) -> Result<Expr, String> {
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let expr = parser.parse()?;
        let mut evaluator = Evaluator::new();
        evaluator.eval(&expr)
    }

    fn eval_to_number(input: &str) -> f64 {
        match eval_expr(input).unwrap() {
            Expr::Number(n) => n,
            other => panic!("Expected number, got {:?}", other),
        }
    }

    fn eval_to_string(input: &str) -> String {
        match eval_expr(input).unwrap() {
            Expr::String(s) => s,
            other => panic!("Expected string, got {:?}", other),
        }
    }

    fn eval_to_list(input: &str) -> Vec<Expr> {
        match eval_expr(input).unwrap() {
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
        let result = eval_expr("(/ 10 0)");
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

        let define_expr = {
            let mut tokenizer = Tokenizer::new("(define x 10)");
            let tokens = tokenizer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            parser.parse().unwrap()
        };

        let use_expr = {
            let mut tokenizer = Tokenizer::new("(+ x 5)");
            let tokens = tokenizer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            parser.parse().unwrap()
        };

        evaluator.eval(&define_expr).unwrap();
        let result = evaluator.eval(&use_expr).unwrap();

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
        let result = eval_expr("(quote (+ 1 2))").unwrap();
        assert_eq!(result, Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Number(1.0),
            Expr::Number(2.0),
        ]));

        let result = eval_expr("(quote x)").unwrap();
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

        let result = eval_expr("(car (list))");
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

        // Define a square function
        let define_expr = {
            let mut tokenizer = Tokenizer::new("(define square (lambda (n) (* n n)))");
            let tokens = tokenizer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            parser.parse().unwrap()
        };

        // Use the square function
        let use_expr = {
            let mut tokenizer = Tokenizer::new("(square 5)");
            let tokens = tokenizer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            parser.parse().unwrap()
        };

        evaluator.eval(&define_expr).unwrap();
        let result = evaluator.eval(&use_expr).unwrap();

        assert_eq!(result, Expr::Number(25.0));
    }

    #[test]
    fn test_eval_lambda_multiple_params() {
        let mut evaluator = Evaluator::new();

        // Define an add function
        let define_expr = {
            let mut tokenizer = Tokenizer::new("(define add (lambda (x y) (+ x y)))");
            let tokens = tokenizer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            parser.parse().unwrap()
        };

        // Use the add function
        let use_expr = {
            let mut tokenizer = Tokenizer::new("(add 3 7)");
            let tokens = tokenizer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            parser.parse().unwrap()
        };

        evaluator.eval(&define_expr).unwrap();
        let result = evaluator.eval(&use_expr).unwrap();

        assert_eq!(result, Expr::Number(10.0));
    }

    #[test]
    fn test_eval_lambda_closure() {
        let mut evaluator = Evaluator::new();

        // Define x in outer scope
        let define_x = {
            let mut tokenizer = Tokenizer::new("(define x 10)");
            let tokens = tokenizer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            parser.parse().unwrap()
        };

        // Define function that uses x
        let define_func = {
            let mut tokenizer = Tokenizer::new("(define add-x (lambda (y) (+ x y)))");
            let tokens = tokenizer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            parser.parse().unwrap()
        };

        // Use the function
        let use_expr = {
            let mut tokenizer = Tokenizer::new("(add-x 5)");
            let tokens = tokenizer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            parser.parse().unwrap()
        };

        evaluator.eval(&define_x).unwrap();
        evaluator.eval(&define_func).unwrap();
        let result = evaluator.eval(&use_expr).unwrap();

        assert_eq!(result, Expr::Number(15.0));
    }

    #[test]
    fn test_eval_immediate_lambda() {
        assert_eq!(eval_to_number("((lambda (x) (* x 2)) 5)"), 10.0);
        assert_eq!(eval_to_number("((lambda (x y) (+ x y)) 3 4)"), 7.0);
    }

    #[test]
    fn test_error_undefined_variable() {
        let result = eval_expr("undefined_var");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }

    #[test]
    fn test_error_invalid_application() {
        let result = eval_expr("(123 456)");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Cannot apply"));
    }

    #[test]
    fn test_error_wrong_arg_count() {
        let result = eval_expr("(+ 1)"); // This should work
        assert!(result.is_ok());

        let result = eval_expr("(= 1)"); // = requires exactly 2 args
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

        // Define factorial function (non-recursive version for simplicity)
        let expr1 = {
            let mut tokenizer = Tokenizer::new("(define fact3 (lambda (n) (* n 2 1)))");
            let tokens = tokenizer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            parser.parse().unwrap()
        };

        // Use in complex expression
        let expr2 = {
            let mut tokenizer = Tokenizer::new("(+ (fact3 3) (- 10 4))");
            let tokens = tokenizer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            parser.parse().unwrap()
        };

        evaluator.eval(&expr1).unwrap();
        let result = evaluator.eval(&expr2).unwrap();

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
}