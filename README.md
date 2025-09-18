# Zeus

Zeus is a live functional programming language written in Rust. It
combines Haskell-inspired purity with the moldable, always-on runtime of
classic Smalltalk systems. The goal is to make experimentation cheap,
introspection effortless, and collaboration delightful.

## Highlights

- Lazy, purely functional core with explicit effect handlers.
- Runtime image that can be inspected and edited while code is running.
- Rich introspection APIs designed for time-travel debugging and
  collaborative tooling.

## Getting Started

```bash
cargo run
```

Running the binary boots a tiny prototype runtime and prints a snapshot
of the current world. The implementation is intentionally skeletal right
now; it acts as a playground for iterating on the language and runtime
concepts.

## Project Layout

- `src/` – language, runtime, and tooling scaffolding.
  - `cli.rs` – command-line entry point.
  - `language/` – syntax and semantics prototypes.
  - `runtime/` – core runtime structures and world model.
  - `introspection/` – hooks that expose live state to tools.
- `docs/` – living documentation covering the language, runtime, and
  philosophy.

## Documentation

- [Overview](docs/overview.md) – big-picture goals and design pillars.
- [Runtime](docs/runtime.md) – how the live world is structured.
- [Philosophy](docs/philosophy.md) – guiding principles for Zeus.

## Roadmap (sketchy)

1. Flesh out the executable semantics and evaluation strategy.
2. Introduce a type checker that feeds into the introspection fabric.
3. Build a moldable inspector UI and workspace tooling.
4. Define a module system that plays well with live editing.

If you are curious or want to riff on the ideas, open an issue, share a
sketch, or hack on the runtime. Zeus is meant to grow in the open.
