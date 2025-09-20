pub mod interpreter;
pub mod tests;
pub mod ui;
pub mod ide;

use crate::interpreter::repl::Repl;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "-ui" => {
                // Launch IDE mode (Phase 1)
                let mut app = ide::IdeApp::new();
                app.run();
            }
            "-ui-old" => {
                // Launch old UI mode
                ui::run_ui();
            }
            "--load" => {
                if args.len() < 3 {
                    eprintln!("Error: --load requires a filename");
                    eprintln!("Usage: {} --load <filename.lisp>", args[0]);
                    std::process::exit(1);
                }

                let filename = &args[2];
                if !Path::new(filename).exists() {
                    eprintln!("Error: File '{}' not found", filename);
                    std::process::exit(1);
                }

                // Read the file
                match fs::read_to_string(filename) {
                    Ok(contents) => {
                        // Create a REPL instance and evaluate the file
                        let mut repl = Repl::new();

                        // Remove comments and join lines, then process complete expressions
                        let mut buffer = String::new();
                        let mut paren_count = 0;

                        for line in contents.lines() {
                            // Skip comment lines
                            if line.trim().starts_with(';') {
                                continue;
                            }

                            // Add the line to buffer
                            buffer.push_str(line);
                            buffer.push(' '); // Add space to separate lines

                            // Count parentheses to detect complete expressions
                            for ch in line.chars() {
                                match ch {
                                    '(' => paren_count += 1,
                                    ')' => paren_count -= 1,
                                    _ => {}
                                }
                            }

                            // If we have a complete expression (balanced parens), evaluate it
                            if paren_count == 0 && !buffer.trim().is_empty() {
                                match repl.evaluate(buffer.trim()) {
                                    Ok(_result) => {
                                        // Don't print the result - let print/println handle output
                                    }
                                    Err(e) => {
                                        eprintln!("Error: {}", e);
                                        eprintln!("In expression: {}", buffer.trim());
                                        std::process::exit(1);
                                    }
                                }
                                buffer.clear();
                            }
                        }

                        // Evaluate any remaining buffer
                        if !buffer.trim().is_empty() {
                            if paren_count != 0 {
                                eprintln!("Error: Unbalanced parentheses in file");
                                std::process::exit(1);
                            }
                            match repl.evaluate(buffer.trim()) {
                                Ok(_result) => {}
                                Err(e) => {
                                    eprintln!("Error: {}", e);
                                    std::process::exit(1);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading file '{}': {}", filename, e);
                        std::process::exit(1);
                    }
                }
            }
            _ => {
                // Run traditional REPL with unknown arguments
                println!("Zeus LISP v0.1.0");
                println!("Usage: {} [-ui | --load <filename.lisp>]", args[0]);
                println!("Type 'exit' or press Ctrl+C to quit\n");

                let mut repl = Repl::new();
                repl.run();
            }
        }
    } else {
        // Run traditional REPL
        println!("Zeus LISP v0.1.0");
        println!("Type 'exit' or press Ctrl+C to quit\n");

        let mut repl = Repl::new();
        repl.run();
    }
}
