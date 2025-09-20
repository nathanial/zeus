use super::helpers::*;
use crate::interpreter::*;

#[test]
fn test_mapcar() {
    let result = eval_to_list("(mapcar (lambda (x) (* x 2)) (list 1 2 3))");
    assert_eq!(
        result,
        vec![Expr::Integer(2), Expr::Integer(4), Expr::Integer(6)]
    );

    // Test with multiple lists
    let result = eval_to_list("(mapcar + (list 1 2 3) (list 10 20 30))");
    assert_eq!(
        result,
        vec![Expr::Integer(11), Expr::Integer(22), Expr::Integer(33)]
    );

    // Test with lists of different lengths (stops at shortest)
    let result = eval_to_list("(mapcar + (list 1 2) (list 10 20 30))");
    assert_eq!(result, vec![Expr::Integer(11), Expr::Integer(22)]);
}

#[test]
fn test_filter() {
    let result = eval_to_list("(filter (lambda (x) (> x 2)) (list 1 2 3 4))");
    assert_eq!(result, vec![Expr::Integer(3), Expr::Integer(4)]);

    // We don't have mod function yet, so let's use a simpler test
    let result = eval_to_list("(filter (lambda (x) (> x 0)) (list -2 -1 0 1 2))");
    assert_eq!(result, vec![Expr::Integer(1), Expr::Integer(2)]);
}

#[test]
fn test_remove() {
    let result = eval_to_list("(remove (lambda (x) (> x 2)) (list 1 2 3 4))");
    assert_eq!(result, vec![Expr::Integer(1), Expr::Integer(2)]);

    let result = eval_to_list("(remove (lambda (x) (< x 0)) (list -2 -1 0 1 2))");
    assert_eq!(
        result,
        vec![Expr::Integer(0), Expr::Integer(1), Expr::Integer(2)]
    );
}

#[test]
fn test_reduce() {
    assert_eq!(eval_to_number("(reduce + (list 1 2 3 4))"), 10.0);
    assert_eq!(eval_to_number("(reduce * (list 1 2 3 4))"), 24.0);

    // With initial value
    assert_eq!(eval_to_number("(reduce + (list 1 2 3) 10)"), 16.0);

    // With lambda
    assert_eq!(
        eval_to_number("(reduce (lambda (x y) (+ x (* y 2))) (list 1 2 3) 0)"),
        12.0
    );

    // Empty list with initial value
    assert_eq!(eval_to_number("(reduce + (list) 42)"), 42.0);
}

#[test]
fn test_reduce_empty_error() {
    let result = Evaluator::eval_once("(reduce + (list))");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("empty list"));
}

#[test]
fn test_apply() {
    assert_eq!(eval_to_number("(apply + (list 1 2 3))"), 6.0);
    assert_eq!(eval_to_number("(apply * (list 2 3 4))"), 24.0);

    // With lambda
    assert_eq!(
        eval_to_number("(apply (lambda (x y) (* x y)) (list 3 4))"),
        12.0
    );
}

#[test]
fn test_funcall() {
    assert_eq!(eval_to_number("(funcall + 1 2 3)"), 6.0);
    assert_eq!(eval_to_number("(funcall * 2 3 4)"), 24.0);

    // With lambda
    assert_eq!(eval_to_number("(funcall (lambda (x y) (* x y)) 3 4)"), 12.0);
}

#[test]
fn test_print_println() {
    // These functions are harder to test since they produce output
    // We can at least verify they return the correct value
    let mut evaluator = Evaluator::new();

    // print returns last arg or empty list
    let result = evaluator
        .eval_str("(print \"hello\" \" \" \"world\")")
        .unwrap();
    assert_eq!(result, Expr::String("world".to_string()));

    let result = evaluator.eval_str("(println \"hello\")").unwrap();
    assert_eq!(result, Expr::String("hello".to_string()));

    let result = evaluator.eval_str("(print)").unwrap();
    assert_eq!(result, Expr::List(vec![]));
}

#[test]
fn test_complex_mapcar() {
    let mut evaluator = Evaluator::new();
    evaluator
        .eval_str("(define square (lambda (x) (* x x)))")
        .unwrap();
    let result = evaluator
        .eval_str("(mapcar square (list 1 2 3 4))")
        .unwrap();
    assert_eq!(
        result,
        Expr::List(vec![
            Expr::Integer(1),
            Expr::Integer(4),
            Expr::Integer(9),
            Expr::Integer(16)
        ])
    );
}

#[test]
fn test_nested_higher_order() {
    // Combining multiple higher-order functions
    let result = eval_to_list(
        "(mapcar (lambda (x) (* x 2)) (filter (lambda (x) (> x 2)) (list 1 2 3 4 5)))",
    );
    assert_eq!(
        result,
        vec![Expr::Integer(6), Expr::Integer(8), Expr::Integer(10)]
    );

    // Reduce the mapped result
    assert_eq!(
        eval_to_number("(reduce + (mapcar (lambda (x) (* x 2)) (list 1 2 3)))"),
        12.0
    );
}

#[test]
fn test_apply_with_funcall() {
    assert_eq!(eval_to_number("(funcall apply + (list 1 2 3))"), 6.0);
    assert_eq!(eval_to_number("(apply funcall (list + 1 2 3))"), 6.0);
}
