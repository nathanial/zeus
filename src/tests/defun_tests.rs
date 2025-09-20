use crate::interpreter::types::SymbolData;
use crate::interpreter::*;

#[test]
fn test_defun_basic() {
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_str("(defun square (x) (* x x))").unwrap();
    assert_eq!(
        result,
        Expr::Symbol(SymbolData::Interned("square".to_string()))
    );

    let result = evaluator.eval_str("(square 5)").unwrap();
    assert_eq!(result, Expr::Integer(25));
}

#[test]
fn test_defun_multiple_params() {
    let mut evaluator = Evaluator::new();
    evaluator
        .eval_str("(defun add3 (a b c) (+ a (+ b c)))")
        .unwrap();
    let result = evaluator.eval_str("(add3 10 20 30)").unwrap();
    assert_eq!(result, Expr::Integer(60));
}

#[test]
fn test_defun_multiple_body_expressions() {
    let mut evaluator = Evaluator::new();
    evaluator
        .eval_str(
            "(defun compute (x)
        (define temp (* x 2))
        (+ temp 10))",
        )
        .unwrap();
    let result = evaluator.eval_str("(compute 5)").unwrap();
    assert_eq!(result, Expr::Integer(20));
}

#[test]
fn test_defun_recursive() {
    let mut evaluator = Evaluator::new();
    evaluator
        .eval_str(
            "(defun factorial (n)
        (if (< n 2)
            1
            (* n (factorial (- n 1)))))",
        )
        .unwrap();
    let result = evaluator.eval_str("(factorial 5)").unwrap();
    assert_eq!(result, Expr::Integer(120));
}

#[test]
fn test_defun_with_closures() {
    let mut evaluator = Evaluator::new();
    evaluator.eval_str("(define offset 100)").unwrap();
    evaluator
        .eval_str("(defun add_offset (x) (+ x offset))")
        .unwrap();
    let result = evaluator.eval_str("(add_offset 50)").unwrap();
    assert_eq!(result, Expr::Integer(150));
}

#[test]
fn test_defun_no_params() {
    let mut evaluator = Evaluator::new();
    evaluator.eval_str("(defun get_constant () 42)").unwrap();
    let result = evaluator.eval_str("(get_constant)").unwrap();
    assert_eq!(result, Expr::Integer(42));
}

#[test]
fn test_defun_overwrite() {
    let mut evaluator = Evaluator::new();
    evaluator.eval_str("(defun foo (x) (* x 2))").unwrap();
    assert_eq!(evaluator.eval_str("(foo 5)").unwrap(), Expr::Integer(10));

    evaluator.eval_str("(defun foo (x) (* x 3))").unwrap();
    assert_eq!(evaluator.eval_str("(foo 5)").unwrap(), Expr::Integer(15));
}
