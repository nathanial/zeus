use crate::interpreter::types::SymbolData;
use crate::interpreter::*;

#[test]
fn test_repl_integration() {
    let mut repl = Repl::new();

    // Test basic arithmetic
    let result = repl.evaluate("(+ 1 2 3)").unwrap();
    assert_eq!(result, Expr::Integer(6));

    // Test define and use
    repl.evaluate("(define x 10)").unwrap();
    let result = repl.evaluate("(* x 2)").unwrap();
    assert_eq!(result, Expr::Integer(20));

    // Test lambda
    repl.evaluate("(define square (lambda (n) (* n n)))")
        .unwrap();
    let result = repl.evaluate("(square 7)").unwrap();
    assert_eq!(result, Expr::Integer(49));
}

#[test]
fn test_format_expr() {
    let repl = Repl::new();

    // Test number formatting
    assert_eq!(repl.format_expr(&Expr::Integer(42)), "42");
    assert_eq!(repl.format_expr(&Expr::Float(3.14)), "3.14");
    assert_eq!(repl.format_expr(&Expr::Integer(-17)), "-17");

    // Test string formatting
    assert_eq!(
        repl.format_expr(&Expr::String("hello".to_string())),
        r#""hello""#
    );

    // Test symbol formatting
    assert_eq!(
        repl.format_expr(&Expr::Symbol(SymbolData::Interned("x".to_string()))),
        "x"
    );

    // Test list formatting
    let list = Expr::List(vec![
        Expr::Symbol(SymbolData::Interned("+".to_string())),
        Expr::Integer(1),
        Expr::Integer(2),
    ]);
    assert_eq!(repl.format_expr(&list), "(+ 1 2)");
}
