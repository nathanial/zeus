use crate::interpreter::*;
use super::helpers::*;

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

#[test]
fn test_and_or_combination() {
    assert_eq!(eval_to_number("(and (or 0 5) (or 10 0))"), 10.0);
    assert_eq!(eval_to_number("(or (and 0 5) (and 10 20))"), 20.0);
}

#[test]
fn test_progn() {
    assert_eq!(eval_to_list("(progn)"), vec![]);
    assert_eq!(eval_to_number("(progn 1 2 3)"), 3.0);

    let mut evaluator = Evaluator::new();
    evaluator.eval_str("(progn (define x 10) (define y 20))").unwrap();
    assert_eq!(evaluator.eval_str("(+ x y)").unwrap(), Expr::Number(30.0));
}

#[test]
fn test_when() {
    assert_eq!(eval_to_number("(when 1 10)"), 10.0);
    assert_eq!(eval_to_list("(when 0 10)"), vec![]);
    assert_eq!(eval_to_number("(when (> 5 3) (+ 1 2) (* 3 4))"), 12.0);
}

#[test]
fn test_unless() {
    assert_eq!(eval_to_list("(unless 1 10)"), vec![]);
    assert_eq!(eval_to_number("(unless 0 10)"), 10.0);
    assert_eq!(eval_to_number("(unless (< 5 3) (+ 1 2) (* 3 4))"), 12.0);
}

#[test]
fn test_case() {
    assert_eq!(eval_to_string("(case 2 (1 \"one\") (2 \"two\") (else \"other\"))"), "two");
    assert_eq!(eval_to_string("(case 5 (1 \"one\") (2 \"two\") (else \"other\"))"), "other");
    assert_eq!(eval_to_list("(case 5 (1 \"one\") (2 \"two\"))"), vec![]);

    // Case with multiple values
    assert_eq!(eval_to_string("(case 2 ((1 2 3) \"small\") (else \"big\"))"), "small");
}

#[test]
fn test_case_string() {
    assert_eq!(eval_to_string("(case \"b\" ((\"a\") \"A\") ((\"b\") \"B\") (else \"?\"))"), "B");
}

#[test]
fn test_case_with_expressions() {
    let mut evaluator = Evaluator::new();
    evaluator.eval_str("(define x 2)").unwrap();
    let result = evaluator.eval_str(
        "(case x
           (1 \"one\")
           (2 (progn (define y 10) (+ y 5)))
           (else \"other\"))"
    ).unwrap();
    assert_eq!(result, Expr::Number(15.0));
}

#[test]
fn test_progn_in_conditionals() {
    assert_eq!(eval_to_number(
        "(when (> 3 2) (define y 20) (+ y 10))"
    ), 30.0);
}

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