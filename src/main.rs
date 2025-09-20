pub mod interpreter;
pub mod tests;
pub mod ui;

use crate::interpreter::repl::Repl;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "-ui" {
        // Launch UI mode
        ui::run_ui();
    } else {
        // Run traditional REPL
        println!("Zeus LISP v0.1.0");
        println!("Type 'exit' or press Ctrl+C to quit\n");

        let mut repl = Repl::new();
        repl.run();
    }
}