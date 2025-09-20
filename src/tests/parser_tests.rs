use crate::interpreter::types::SymbolData;
use crate::interpreter::*;

#[test]
fn test_parser_number() {
    let tokens = vec![Token::Integer(42)];
    let mut parser = Parser::new(tokens);
    let expr = parser.parse().unwrap();
    assert_eq!(expr, Expr::Integer(42));
}

#[test]
fn test_parser_symbol() {
    let tokens = vec![Token::Symbol("x".to_string())];
    let mut parser = Parser::new(tokens);
    let expr = parser.parse().unwrap();
    assert_eq!(expr, Expr::Symbol(SymbolData::Interned("x".to_string())));
}

#[test]
fn test_parser_string() {
    let tokens = vec![Token::String("hello".to_string())];
    let mut parser = Parser::new(tokens);
    let expr = parser.parse().unwrap();
    assert_eq!(expr, Expr::String("hello".to_string()));
}

#[test]
fn test_parser_list() {
    let tokens = vec![
        Token::LeftParen,
        Token::Symbol("+".to_string()),
        Token::Integer(1),
        Token::Integer(2),
        Token::RightParen,
    ];
    let mut parser = Parser::new(tokens);
    let expr = parser.parse().unwrap();
    assert_eq!(
        expr,
        Expr::List(vec![
            Expr::Symbol(SymbolData::Interned("+".to_string())),
            Expr::Integer(1),
            Expr::Integer(2),
        ])
    );
}

#[test]
fn test_parser_nested_list() {
    let tokens = vec![
        Token::LeftParen,
        Token::Symbol("+".to_string()),
        Token::LeftParen,
        Token::Symbol("*".to_string()),
        Token::Integer(2),
        Token::Integer(3),
        Token::RightParen,
        Token::Integer(4),
        Token::RightParen,
    ];
    let mut parser = Parser::new(tokens);
    let expr = parser.parse().unwrap();
    assert_eq!(
        expr,
        Expr::List(vec![
            Expr::Symbol(SymbolData::Interned("+".to_string())),
            Expr::List(vec![
                Expr::Symbol(SymbolData::Interned("*".to_string())),
                Expr::Integer(2),
                Expr::Integer(3),
            ]),
            Expr::Integer(4),
        ])
    );
}
