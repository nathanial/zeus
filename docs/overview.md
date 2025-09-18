# Zeus Overview

Zeus is an experiment in live functional programming. We couple the
algebraic clarity of Haskell with the moldable development experience of
classic Smalltalk systems. Programs are expressed in a pure, lazily
evaluated language while the runtime stays malleable and explorable at
all times.

Key ideas:

- **Interactive runtime** – the running system is always inspectable and
  editable; values retain their history and shape, making time-travel
  debugging a first-class feature.
- **Composable abstractions** – core language constructs are just data;
  macros, type classes, effects, and schedulers can be modified from the
  live image without recompilation.
- **Deterministic foundations** – purity and laziness give predictable
  semantics, while effect handlers model interaction with the outside
  world.

We expect Zeus to evolve quickly. This document captures the current
shape of the language and tooling so the project always has a single
source of truth.
