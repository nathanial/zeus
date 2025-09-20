use crate::interpreter::*;

#[test]
fn test_tokenizer_numbers() {
    let mut tokenizer = Tokenizer::new("42 3.14 -17 -2.5");
    let tokens = tokenizer.tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::Number(42.0),
            Token::Number(3.14),
            Token::Number(-17.0),
            Token::Number(-2.5),
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
            Token::Number(1.0),
            Token::Number(2.0),
            Token::RightParen,
            Token::LeftParen,
            Token::Symbol("define".to_string()),
            Token::Symbol("x".to_string()),
            Token::Number(10.0),
            Token::RightParen,
        ]
    );
}
