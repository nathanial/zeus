use crate::interpreter::{
    evaluator::Evaluator,
    types::{Expr, SymbolData},
};
use std::io::{self, Write};

pub struct Repl {
    evaluator: Evaluator,
}

impl Repl {
    pub fn new() -> Self {
        Repl {
            evaluator: Evaluator::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            print!("zeus> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    // EOF reached
                    println!("\nGoodbye!");
                    break;
                }
                Ok(_) => {
                    let input = input.trim();

                    if input == "exit" {
                        println!("Goodbye!");
                        break;
                    }

                    if input.is_empty() {
                        continue;
                    }

                    match self.evaluate(input) {
                        Ok(result) => println!("{}", self.format_expr(&result)),
                        Err(e) => println!("Error: {}", e),
                    }
                }
                Err(error) => {
                    println!("Error reading input: {}", error);
                    break;
                }
            }
        }
    }

    pub fn evaluate(&mut self, input: &str) -> Result<Expr, String> {
        self.evaluator.eval_str(input)
    }

    pub fn format_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Integer(n) => format!("{}", n),
            Expr::Float(f) => format!("{}", f),
            Expr::Rational {
                numerator,
                denominator,
            } => format!("{}/{}", numerator, denominator),
            Expr::Character(ch) => format!(
                "#\\{}",
                match *ch {
                    ' ' => "space".to_string(),
                    '\n' => "newline".to_string(),
                    '\t' => "tab".to_string(),
                    '\r' => "return".to_string(),
                    c => c.to_string(),
                }
            ),
            Expr::Symbol(sym_data) => match sym_data {
                SymbolData::Keyword(name) => format!(":{}", name),
                SymbolData::Uninterned(name, id) => format!("#:{}#{}", name, id),
                SymbolData::Interned(name) => name.clone(),
            },
            Expr::String(s) => format!("\"{}\"", s),
            Expr::List(list) => {
                let items: Vec<String> = list.iter().map(|e| self.format_expr(e)).collect();
                format!("({})", items.join(" "))
            }
            Expr::Cons(car, cdr) => {
                let mut repr = String::from("(");
                repr.push_str(&self.format_expr(car));

                let mut tail = cdr.as_ref();
                loop {
                    match tail {
                        Expr::Cons(next_car, next_cdr) => {
                            repr.push(' ');
                            repr.push_str(&self.format_expr(next_car));
                            tail = next_cdr.as_ref();
                        }
                        Expr::List(list) => {
                            for item in list {
                                repr.push(' ');
                                repr.push_str(&self.format_expr(item));
                            }
                            repr.push(')');
                            break;
                        }
                        other => {
                            repr.push_str(" . ");
                            repr.push_str(&self.format_expr(other));
                            repr.push(')');
                            break;
                        }
                    }
                }

                repr
            }
            Expr::Vector(vec) => {
                let items: Vec<String> = vec.iter().map(|e| self.format_expr(e)).collect();
                format!("[{}]", items.join(" "))
            }
            Expr::HashTable(h) => {
                format!("#<hash-table:{}>", h.len())
            }
        }
    }
}
