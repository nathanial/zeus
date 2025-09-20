use super::helpers::*;
use crate::interpreter::*;

#[test]
fn test_eval_number() {
    let result = Evaluator::eval_once("42");
    assert_eq!(result.unwrap(), Expr::Number(42.0));
}

#[test]
fn test_eval_string() {
    let result = Evaluator::eval_once(r#""hello""#).unwrap();
    assert_eq!(result, Expr::String("hello".to_string()));
}

#[test]
fn test_eval_define_and_use() {
    let mut evaluator = Evaluator::new();
    evaluator.eval_str("(define x 10)").unwrap();
    let result = evaluator.eval_str("x").unwrap();
    assert_eq!(result, Expr::Number(10.0));
}

#[test]
fn test_eval_if_true() {
    assert_eq!(eval_to_number(r#"(if (> 5 3) 10 20)"#), 10.0);
}

#[test]
fn test_eval_if_false() {
    assert_eq!(eval_to_number(r#"(if (< 5 3) 10 20)"#), 20.0);
}

#[test]
fn test_eval_quote() {
    let result = Evaluator::eval_once("(quote (+ 1 2))").unwrap();
    assert_eq!(
        result,
        Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Number(1.0),
            Expr::Number(2.0),
        ])
    );

    // Test single-quote syntax
    let result = Evaluator::eval_once("(quote x)").unwrap();
    assert_eq!(result, Expr::Symbol("x".to_string()));
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
}

#[test]
fn test_error_wrong_arg_count() {
    let result = Evaluator::eval_once("(car)");
    assert!(result.is_err());

    let result = Evaluator::eval_once("(car 1 2 3)");
    assert!(result.is_err());
}

#[test]
fn test_empty_list() {
    assert_eq!(eval_to_list("(list)"), vec![]);
}

#[test]
fn test_complex_expression() {
    let mut evaluator = Evaluator::new();
    evaluator.eval_str("(define x 5)").unwrap();
    evaluator.eval_str("(define y 10)").unwrap();
    let result = evaluator.eval_str("(+ (* x 2) y)").unwrap();
    assert_eq!(result, Expr::Number(20.0));
}
