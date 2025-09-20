use crate::interpreter::types::{Expr, SymbolData, Token};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Some(Token::Integer(n)) => Ok(Expr::Integer(n)),
            Some(Token::Float(n)) => Ok(Expr::Float(n)),
            Some(Token::Character(ch)) => Ok(Expr::Character(ch)),
            Some(Token::Symbol(s)) => Ok(Expr::Symbol(SymbolData::Interned(s))),
            Some(Token::Keyword(s)) => Ok(Expr::Symbol(SymbolData::Keyword(s))),
            Some(Token::String(s)) => Ok(Expr::String(s)),
            Some(Token::LeftParen) => {
                let mut list = Vec::new();

                loop {
                    match self.peek() {
                        Some(Token::RightParen) => {
                            self.advance();
                            return Ok(Expr::List(list));
                        }
                        None => return Err("Unexpected end of input".to_string()),
                        _ => {
                            list.push(self.parse_expr()?);
                        }
                    }
                }
            }
            Some(Token::LeftBracket) => {
                let mut vector = Vec::new();

                loop {
                    match self.peek() {
                        Some(Token::RightBracket) => {
                            self.advance();
                            return Ok(Expr::Vector(vector));
                        }
                        None => return Err("Unexpected end of input in vector".to_string()),
                        _ => {
                            vector.push(self.parse_expr()?);
                        }
                    }
                }
            }
            Some(Token::RightParen) => Err("Unexpected )".to_string()),
            Some(Token::RightBracket) => Err("Unexpected ]".to_string()),
            None => Err("Unexpected end of input".to_string()),
        }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        let expr = self.parse_expr()?;

        if self.position < self.tokens.len() {
            Err("Extra tokens after expression".to_string())
        } else {
            Ok(expr)
        }
    }
}
