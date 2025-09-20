use crate::interpreter::types::Token;

pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Tokenizer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let ch = self.input[self.position];
            self.position += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_string(&mut self) -> Result<String, String> {
        let mut result = String::new();
        self.advance(); // Skip opening quote

        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance(); // Skip closing quote
                return Ok(result);
            } else if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.advance() {
                    match escaped {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '\\' => result.push('\\'),
                        '"' => result.push('"'),
                        _ => {
                            result.push('\\');
                            result.push(escaped);
                        }
                    }
                }
            } else {
                result.push(ch);
                self.advance();
            }
        }

        Err("Unterminated string".to_string())
    }

    fn read_symbol(&mut self) -> String {
        let mut result = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || "+-*/<>=!?_".contains(ch) {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        result
    }

    fn read_number(&mut self) -> Result<f64, String> {
        let mut result = String::new();
        let mut has_dot = false;

        if self.peek() == Some('-') {
            result.push('-');
            self.advance();
        }

        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                result.push(ch);
                self.advance();
            } else if ch == '.' && !has_dot {
                has_dot = true;
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        result.parse().map_err(|_| "Invalid number".to_string())
    }

    fn next_token(&mut self) -> Result<Option<Token>, String> {
        self.skip_whitespace();

        match self.peek() {
            None => Ok(None),
            Some('(') => {
                self.advance();
                Ok(Some(Token::LeftParen))
            }
            Some(')') => {
                self.advance();
                Ok(Some(Token::RightParen))
            }
            Some('"') => {
                let s = self.read_string()?;
                Ok(Some(Token::String(s)))
            }
            Some(ch) if ch == '-' || ch.is_ascii_digit() => {
                let start_pos = self.position;
                if ch == '-' {
                    self.advance();
                    if let Some(next_ch) = self.peek() {
                        if next_ch.is_ascii_digit() {
                            self.position = start_pos;
                            let num = self.read_number()?;
                            Ok(Some(Token::Number(num)))
                        } else {
                            self.position = start_pos;
                            let sym = self.read_symbol();
                            Ok(Some(Token::Symbol(sym)))
                        }
                    } else {
                        self.position = start_pos;
                        let sym = self.read_symbol();
                        Ok(Some(Token::Symbol(sym)))
                    }
                } else {
                    let num = self.read_number()?;
                    Ok(Some(Token::Number(num)))
                }
            }
            Some(':') => {
                self.advance(); // consume the ':'
                let name = self.read_symbol();
                if name.is_empty() {
                    Err("Invalid keyword: empty name after ':'".to_string())
                } else {
                    Ok(Some(Token::Keyword(name)))
                }
            }
            Some(_) => {
                let sym = self.read_symbol();
                if sym.is_empty() {
                    Err(format!("Unexpected character: {:?}", self.peek()))
                } else {
                    Ok(Some(Token::Symbol(sym)))
                }
            }
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }

        Ok(tokens)
    }
}
