pub mod interpreter;
pub mod tests;

use interpreter::Repl;

fn main() {
    println!("Zeus LISP v0.1.0");
    println!("Type 'exit' or press Ctrl+C to quit\n");

    let mut repl = Repl::new();
    repl.run();
}