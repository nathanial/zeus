# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Build and Run
```bash
# Build the project
cargo build

# Run the REPL
cargo run

# Build in release mode with optimizations
cargo build --release
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output visible
cargo test -- --nocapture

# Run a specific test
cargo test test_name

# Run tests matching a pattern
cargo test test_arithmetic
```

### Development Tools
```bash
# Format code
cargo fmt

# Check for linting issues
cargo clippy

# Check if code compiles without building
cargo check
```

## Architecture

Zeus is a LISP interpreter written in Rust with a modular architecture split across multiple files in `src/interpreter/`:

### Core Evaluation Flow
1. **Input Processing**: User input → `Tokenizer` → `Parser` → `Evaluator`
2. **Expression Tree**: The parser builds `Expr` enum trees representing S-expressions
3. **Environment**: Variables are stored in scoped environments that can be pushed/popped for local bindings
4. **Evaluation**: The evaluator recursively processes expressions, dispatching to special forms or built-in functions

### Module Organization
- **types.rs**: Core data structures (`Token` and `Expr` enums)
- **tokenizer.rs**: Lexical analysis, converting strings to tokens
- **parser.rs**: Builds expression trees from token streams
- **environment.rs**: Variable scope management with nested environments
- **evaluator.rs**: Core evaluation logic and helper methods (`eval`, `eval_str`, `parse`)
- **evaluator_special_forms.rs**: Language constructs like `cond`, `case`, `when`, `progn`
- **evaluator_builtins.rs**: Built-in functions (arithmetic, lists, higher-order functions)
- **evaluator_builtins_cont.rs**: Additional built-ins (nth, nthcdr, member, mapcar, etc.)
- **repl.rs**: Read-Eval-Print Loop implementation

### Key Design Patterns

**Special Forms vs Functions**: Special forms (like `if`, `let`, `lambda`) control evaluation of their arguments and are handled directly in the evaluator. Built-in functions receive already-evaluated arguments.

**Environment Scoping**: The environment uses a stack of hash maps. `push_scope()` creates a new local scope, `pop_scope()` removes it. This enables lexical scoping for `let` bindings and function parameters.

**Lambda Representation**: Lambdas are stored as lists `(lambda (params...) body)` and create new scopes when applied, binding parameters to arguments.

**Error Propagation**: All evaluation functions return `Result<Expr, String>` for consistent error handling throughout the interpreter.

## Testing Strategy

Tests are in `src/tests.rs` and use helper functions:
- `eval_to_number()`: Assert expression evaluates to a number
- `eval_to_string()`: Assert expression evaluates to a string
- `eval_to_list()`: Assert expression evaluates to a list
- `Evaluator::eval_once()`: One-shot evaluation for testing

Tests cover:
- Tokenization and parsing
- Arithmetic operations
- List operations
- Special forms (let, cond, case, etc.)
- Lambda functions and closures
- Higher-order functions (mapcar, filter, reduce)

## LISP Dialect Features

Zeus implements a subset of Common Lisp. Key implemented features:
- Basic data types: numbers (f64), symbols, strings, lists
- Special forms: `define`, `if`, `quote`, `lambda`, `let`, `let*`, `cond`, `case`, `and`, `or`, `progn`, `when`, `unless`
- List operations: `car`, `cdr`, `cons`, `list`, `append`, `reverse`, `length`, `nth`, `nthcdr`, `member`
- Higher-order functions: `mapcar`, `filter`, `remove`, `reduce`, `apply`, `funcall`
- I/O: `print`, `println`

See `MissingFeatures.md` for Common Lisp features not yet implemented.

## Adding New Features

### Adding a Built-in Function
1. Add the function name to the match statement in `evaluator_builtins.rs::apply_builtin()`
2. Implement the function in the same file or `evaluator_builtins_cont.rs`
3. Add tests in `src/tests.rs`

### Adding a Special Form
1. Add the form name to the match statement in `evaluator.rs::eval()`
2. Implement the evaluation logic in `evaluator_special_forms.rs`
3. Special forms control evaluation of their arguments
4. Add comprehensive tests

### Type System Extensions
To add new data types, modify the `Expr` enum in `types.rs` and update:
- Parser to recognize literals
- Evaluator to handle the new type
- Format functions for REPL output
- Equality and comparison operations if applicable