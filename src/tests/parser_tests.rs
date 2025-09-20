use crate::interpreter::*;

#[test]
fn test_parser_number() {
    let tokens = vec![Token::Number(42.0)];
    let mut parser = Parser::new(tokens);
    let expr = parser.parse().unwrap();
    assert_eq!(expr, Expr::Number(42.0));
}

#[test]
fn test_parser_symbol() {
    let tokens = vec![Token::Symbol("x".to_string())];
    let mut parser = Parser::new(tokens);
    let expr = parser.parse().unwrap();
    assert_eq!(expr, Expr::Symbol("x".to_string()));
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
        Token::Number(1.0),
        Token::Number(2.0),
        Token::RightParen,
    ];
    let mut parser = Parser::new(tokens);
    let expr = parser.parse().unwrap();
    assert_eq!(
        expr,
        Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Number(1.0),
            Expr::Number(2.0),
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
        Token::Number(2.0),
        Token::Number(3.0),
        Token::RightParen,
        Token::Number(4.0),
        Token::RightParen,
    ];
    let mut parser = Parser::new(tokens);
    let expr = parser.parse().unwrap();
    assert_eq!(
        expr,
        Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::List(vec![
                Expr::Symbol("*".to_string()),
                Expr::Number(2.0),
                Expr::Number(3.0),
            ]),
            Expr::Number(4.0),
        ])
    );
}
