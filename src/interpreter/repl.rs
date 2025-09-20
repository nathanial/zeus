use crate::interpreter::{evaluator::Evaluator, types::{Expr, SymbolData}};
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
            Expr::Number(n) => {
                if n.fract() == 0.0 && n.abs() < 1e10 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Expr::Symbol(sym_data) => {
                match sym_data {
                    SymbolData::Keyword(name) => format!(":{}", name),
                    SymbolData::Uninterned(name, id) => format!("#:{}#{}", name, id),
                    SymbolData::Interned(name) => name.clone(),
                }
            }
            Expr::String(s) => format!("\"{}\"", s),
            Expr::List(list) => {
                let items: Vec<String> = list.iter().map(|e| self.format_expr(e)).collect();
                format!("({})", items.join(" "))
            }
        }
    }
}
