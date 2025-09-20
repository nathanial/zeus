use super::helpers::*;
use crate::interpreter::*;

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
    assert!(eval_to_bool("(= 5 5)"));
    assert!(!eval_to_bool("(= 5 3)"));
    assert!(eval_to_bool("(< 3 5)"));
    assert!(!eval_to_bool("(< 5 3)"));
    assert!(eval_to_bool("(> 5 3)"));
    assert!(!eval_to_bool("(> 3 5)"));
    assert!(eval_to_bool("(= 1 1 1.0)"));
    assert!(!eval_to_bool("(= 1 2 1)"));
    assert!(eval_to_bool("(/= 1 2 3)"));
    assert!(!eval_to_bool("(/= 1 2 1)"));
    assert!(eval_to_bool("(< 1 2 3 4)"));
    assert!(!eval_to_bool("(< 1 2 2 4)"));
    assert!(eval_to_bool("(<= 1 2 2 3.5)"));
    assert!(!eval_to_bool("(<= 1 3 2)"));
    assert!(eval_to_bool("(> 5 4 3 2)"));
    assert!(!eval_to_bool("(> 5 5 4)"));
    assert!(eval_to_bool("(>= 5 5 4 4)"));
    assert!(!eval_to_bool("(>= 5 4 4 5)"));
}

#[test]
fn test_eval_nested_arithmetic() {
    assert_eq!(eval_to_number("(+ 1 (* 2 3))"), 7.0);
    assert_eq!(eval_to_number("(* (+ 1 2) (- 5 2))"), 9.0);
    assert_eq!(eval_to_number("(/ (+ 10 20) (- 10 4))"), 5.0);
}

#[test]
fn test_comparison_supports_rationals() {
    let mut evaluator = Evaluator::new();
    let half = Expr::Rational {
        numerator: 1,
        denominator: 2,
    };
    let third = Expr::Rational {
        numerator: 1,
        denominator: 3,
    };

    let lt_result = evaluator
        .apply_builtin("<", &[third.clone(), half.clone()])
        .unwrap();
    assert_eq!(lt_result, Evaluator::bool_to_expr(true));

    let eq_result = evaluator
        .apply_builtin("=", &[half.clone(), Expr::Float(0.5)])
        .unwrap();
    assert_eq!(eq_result, Evaluator::bool_to_expr(true));

    let neq_result = evaluator
        .apply_builtin("/=", &[half.clone(), Expr::Float(0.5)])
        .unwrap();
    assert_eq!(neq_result, Evaluator::bool_to_expr(false));

    let ge_result = evaluator
        .apply_builtin(">=", &[half, third.clone(), third])
        .unwrap();
    assert_eq!(ge_result, Evaluator::bool_to_expr(true));
}
