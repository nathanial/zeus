use super::helpers::*;
use crate::interpreter::*;

#[test]
fn test_eval_list() {
    let result = eval_to_list("(list 1 2 3)");
    assert_eq!(
        result,
        vec![Expr::Integer(1), Expr::Integer(2), Expr::Integer(3),]
    );

    let result = eval_to_list("(list)");
    assert_eq!(result, vec![]);
}

#[test]
fn test_eval_car() {
    assert_eq!(eval_to_number("(car (list 1 2 3))"), 1.0);

    let result = Evaluator::eval_once("(car (list))").unwrap();
    assert_eq!(result, Expr::List(vec![]));
}

#[test]
fn test_eval_cdr() {
    let result = eval_to_list("(cdr (list 1 2 3))");
    assert_eq!(result, vec![Expr::Integer(2), Expr::Integer(3),]);

    let result = eval_to_list("(cdr (list))");
    assert_eq!(result, vec![]);
}

#[test]
fn test_eval_cons() {
    let result = eval_to_list("(cons 0 (list 1 2))");
    assert_eq!(
        result,
        vec![Expr::Integer(0), Expr::Integer(1), Expr::Integer(2),]
    );

    let result = Evaluator::eval_once("(cons 1 2)").unwrap();
    match result {
        Expr::Cons(car, cdr) => {
            assert_eq!(*car, Expr::Integer(1));
            assert_eq!(*cdr, Expr::Integer(2));
        }
        other => panic!("Expected cons cell, got {:?}", other),
    }
}

#[test]
fn test_append() {
    let result = eval_to_list("(append (list 1 2) (list 3 4))");
    assert_eq!(
        result,
        vec![
            Expr::Integer(1),
            Expr::Integer(2),
            Expr::Integer(3),
            Expr::Integer(4),
        ]
    );

    let result = eval_to_list("(append (list 1) (list 2 3) (list 4 5))");
    assert_eq!(
        result,
        vec![
            Expr::Integer(1),
            Expr::Integer(2),
            Expr::Integer(3),
            Expr::Integer(4),
            Expr::Integer(5),
        ]
    );

    let result = eval_to_list("(append)");
    assert_eq!(result, vec![]);
}

#[test]
fn test_reverse() {
    let result = eval_to_list("(reverse (list 1 2 3 4))");
    assert_eq!(
        result,
        vec![
            Expr::Integer(4),
            Expr::Integer(3),
            Expr::Integer(2),
            Expr::Integer(1),
        ]
    );

    let result = eval_to_list("(reverse (list))");
    assert_eq!(result, vec![]);
}

#[test]
fn test_length() {
    assert_eq!(eval_to_number("(length (list 1 2 3))"), 3.0);
    assert_eq!(eval_to_number("(length (list))"), 0.0);
    assert_eq!(eval_to_number("(length \"hello\")"), 5.0);
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

#[test]
fn test_nthcdr() {
    let result = eval_to_list("(nthcdr 0 (list 1 2 3))");
    assert_eq!(
        result,
        vec![Expr::Integer(1), Expr::Integer(2), Expr::Integer(3)]
    );

    let result = eval_to_list("(nthcdr 1 (list 1 2 3))");
    assert_eq!(result, vec![Expr::Integer(2), Expr::Integer(3)]);

    let result = eval_to_list("(nthcdr 3 (list 1 2 3))");
    assert_eq!(result, vec![]);

    let result = eval_to_list("(nthcdr 10 (list 1 2 3))");
    assert_eq!(result, vec![]);
}

#[test]
fn test_member() {
    let result = eval_to_list("(member 2 (list 1 2 3))");
    assert_eq!(result, vec![Expr::Integer(2), Expr::Integer(3)]);

    let result = eval_to_list("(member 3 (list 1 2 3))");
    assert_eq!(result, vec![Expr::Integer(3)]);

    let result = eval_to_list("(member 5 (list 1 2 3))");
    assert_eq!(result, vec![]);

    let result = Evaluator::eval_once("(member \"b\" (list \"a\" \"b\" \"c\"))").unwrap();
    assert_eq!(
        result,
        Expr::List(vec![
            Expr::String("b".to_string()),
            Expr::String("c".to_string()),
        ])
    );
}

#[test]
fn test_list_operations_combination() {
    let mut evaluator = Evaluator::new();
    evaluator.eval_str("(define lst (list 1 2 3 4 5))").unwrap();

    let result = evaluator.eval_str("(car (cdr lst))").unwrap();
    assert_eq!(result, Expr::Integer(2));

    let result = evaluator.eval_str("(cons 0 (reverse (cdr lst)))").unwrap();
    assert_eq!(
        result,
        Expr::List(vec![
            Expr::Integer(0),
            Expr::Integer(5),
            Expr::Integer(4),
            Expr::Integer(3),
            Expr::Integer(2),
        ])
    );
}
