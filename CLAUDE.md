# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Zeus is a live functional programming language written in Rust that combines Haskell-inspired purity with Smalltalk-style runtime introspection. It's currently in early prototype stage, focusing on experimentation with language and runtime concepts.

## Development Commands

```bash
# Run the Zeus runtime
cargo run

# Run tests
cargo test

# Check code with clippy
cargo clippy

# Format code
cargo fmt

# Build the project
cargo build

# Build for release
cargo build --release
```

## Architecture

### Module Structure
- **`src/cli.rs`**: Entry point for command-line interface, bootstraps runtime and workspace
- **`src/lib.rs`**: Core library exports and error types (`ZeusResult`, `ZeusError`)
- **`src/runtime/`**: Runtime system implementation
  - `mod.rs`: `ZeusRuntime` - manages execution and live state
  - `world.rs`: `World` and `WorldSnapshot` - represents the live program graph
- **`src/language/`**: Language structures for syntax and semantics
  - `syntax.rs`: AST definitions (`Module`, `Declaration`, `Expression`, `Literal`)
  - `semantics.rs`: Type system components (`InferenceMode`, `TypeDescriptor`, `Kind`)
- **`src/introspection/`**: Tools for live development experience
  - `Workspace` - mirrors runtime state for tooling

### Key Concepts
- **World**: Central data structure containing all modules and runtime state
- **Bootstrap**: Runtime initialization creates seed modules (Prelude, Temporal, Reflect)
- **Snapshots**: Immutable views of the world state for introspection
- **Pure Functional Core**: Expressions are lazy and purely functional with explicit effect handlers

## Design Principles
- Maintain purity and laziness in the core language
- Keep runtime malleable and always inspectable
- Prioritize experimentation and rapid iteration
- Follow Rust idioms and safety practices

## Current State
The implementation is intentionally minimal - a prototype for exploring language concepts. The runtime boots with example modules and prints a world snapshot. Focus areas for development include:
1. Executable semantics and evaluation strategy
2. Type checker integrated with introspection
3. Module system compatible with live editing
4. Inspector UI and workspace tooling