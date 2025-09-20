use crate::interpreter::*;
use crate::interpreter::types::SymbolData;

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
    repl.evaluate("(define square (lambda (n) (* n n)))")
        .unwrap();
    let result = repl.evaluate("(square 7)").unwrap();
    assert_eq!(result, Expr::Number(49.0));
}

#[test]
fn test_format_expr() {
    let repl = Repl::new();

    // Test number formatting
    assert_eq!(repl.format_expr(&Expr::Number(42.0)), "42");
    assert_eq!(repl.format_expr(&Expr::Number(3.14)), "3.14");
    assert_eq!(repl.format_expr(&Expr::Number(-17.0)), "-17");

    // Test string formatting
    assert_eq!(
        repl.format_expr(&Expr::String("hello".to_string())),
        r#""hello""#
    );

    // Test symbol formatting
    assert_eq!(repl.format_expr(&Expr::Symbol(SymbolData::Interned("x".to_string()))), "x");

    // Test list formatting
    let list = Expr::List(vec![
        Expr::Symbol(SymbolData::Interned("+".to_string())),
        Expr::Number(1.0),
        Expr::Number(2.0),
    ]);
    assert_eq!(repl.format_expr(&list), "(+ 1 2)");
}
