use super::helpers::*;
use crate::interpreter::*;

#[test]
fn test_eval_lambda_simple() {
    let mut evaluator = Evaluator::new();
    evaluator
        .eval_str("(define square (lambda (n) (* n n)))")
        .unwrap();
    let result = evaluator.eval_str("(square 5)").unwrap();
    assert_eq!(result, Expr::Number(25.0));
}

#[test]
fn test_eval_lambda_multiple_params() {
    let mut evaluator = Evaluator::new();
    evaluator
        .eval_str("(define add (lambda (x y) (+ x y)))")
        .unwrap();
    let result = evaluator.eval_str("(add 3 7)").unwrap();
    assert_eq!(result, Expr::Number(10.0));
}

#[test]
fn test_eval_lambda_closure() {
    let mut evaluator = Evaluator::new();
    evaluator.eval_str("(define x 10)").unwrap();
    evaluator
        .eval_str("(define add-x (lambda (y) (+ x y)))")
        .unwrap();
    let result = evaluator.eval_str("(add-x 5)").unwrap();
    assert_eq!(result, Expr::Number(15.0));
}

#[test]
fn test_eval_immediate_lambda() {
    assert_eq!(eval_to_number("((lambda (x) (* x 2)) 5)"), 10.0);
    assert_eq!(eval_to_number("((lambda (x y) (+ x y)) 3 4)"), 7.0);
}

#[test]
fn test_let_with_lambda() {
    let mut evaluator = Evaluator::new();
    let result = evaluator
        .eval_str(
            "(let ((f (lambda (x) (* x 2))))
           (f 10))",
        )
        .unwrap();
    assert_eq!(result, Expr::Number(20.0));
}
