use crate::interpreter::*;

#[test]
fn test_tokenizer_numbers() {
    let mut tokenizer = Tokenizer::new("42 3.14 -17 -2.5");
    let tokens = tokenizer.tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::Integer(42),
            Token::Float(3.14),
            Token::Integer(-17),
            Token::Float(-2.5),
        ]
    );
}

#[test]
fn test_tokenizer_symbols() {
    let mut tokenizer = Tokenizer::new("+ define lambda x y");
    let tokens = tokenizer.tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::Symbol("+".to_string()),
            Token::Symbol("define".to_string()),
            Token::Symbol("lambda".to_string()),
            Token::Symbol("x".to_string()),
            Token::Symbol("y".to_string()),
        ]
    );
}

#[test]
fn test_tokenizer_strings() {
    let mut tokenizer = Tokenizer::new(r#""hello" "world" "with \"quotes\"""#);
    let tokens = tokenizer.tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::String("hello".to_string()),
            Token::String("world".to_string()),
            Token::String("with \"quotes\"".to_string()),
        ]
    );
}

#[test]
fn test_tokenizer_lists() {
    let mut tokenizer = Tokenizer::new("(+ 1 2) (define x 10)");
    let tokens = tokenizer.tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::LeftParen,
            Token::Symbol("+".to_string()),
            Token::Integer(1),
            Token::Integer(2),
            Token::RightParen,
            Token::LeftParen,
            Token::Symbol("define".to_string()),
            Token::Symbol("x".to_string()),
            Token::Integer(10),
            Token::RightParen,
        ]
    );
}
