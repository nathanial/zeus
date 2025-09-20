use crate::interpreter::*;
use super::helpers::*;

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