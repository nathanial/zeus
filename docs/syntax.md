# Zeus Syntax

Zeus borrows the mathematical rigor of functional languages while
embracing the conversational style of Smalltalk. The surface syntax is
designed to read like prose without losing the precision required for
static typing and effect tracking.

## Guiding Principles

- **Sentence-friendly.** Programs favor keyword phrases that flow left to
  right: `list keep: evens andMapTo: squares`.
- **Expression oriented.** Every construct yields a value; blocks, loops,
and even declarations are expressions.
- **Statically typed.** Readable surface forms desugar into core
  expressions checked by the compiler.
- **Capability aware.** Effectful code names the capabilities it uses; no
  implicit ambient authority.

## Lexical Structure

- **Identifiers** may contain words separated by spaces and terminated by
  a colon to signal keyword sections: `between:and:`, `fold from:`.
- **Infix names** are enclosed in backticks: `` value `between:and:` low
  high ``. Precedence is declared alongside the definition.
- **Blocks** use square brackets: `[ binding <- read line ]`. Blocks are
  lambdas; parameters appear before a vertical bar: `[request | handle
  request]`.
- **Comments** begin with `--` for single lines or `{- ... -}` for nested
  block comments.

## Literals

- Numbers support separators: `10_000`.
- Strings are double quoted with interpolation via backtick prefices:
  `"Hello, `name`!"`.
- Records use Smalltalk-style slots: `{ greeting := "hi", count := 0 }`.
- Lists are `[1, 2, 3]`; ranges use keywords: `range from: 1 to: 10 by: 2`.

## Function Definitions

Functions are keyword chains. The compiler treats each keyword section as
a parameter. Currying happens automatically.

```zeus
route root: request using: config =
    let greeting = config.greeting defaultingTo: "Welcome"
        now = Time now
    in Response ok: "text/plain" body:
         greeting ++ " @ " ++ show now
```

Definitions may also use prefix form for single-word names:

```zeus
identity x = x
```

## Application Forms

- **Keyword application:** `Timer startAt: now withLabel: "web"`.
- **Infix application:** `value `between:and:` low high`.
- **Pipeline:** `request |> authorize |> handle |> respond`. Pipelines are
  left-associative and insert as first positional argument.
- **Block call:** `list each: [item | Console print: item]`.

## Pattern Matching

Zeus patterns strive for readability:

```zeus
match request with
    case GET path -> handle get: path for: request
    case POST path andBody: body -> handle post: path with: body
    default -> Response notFound
```

Patterns support guards via `provided` clauses and destructuring of lists,
records, and effect outcomes.

## Effects and Capabilities

Effectful functions list capabilities after a `requires` clause:

```zeus
Logger append: entry requires { Temporal, LogStorage } =
    let timestamp = Time now
    in Storage write: entry withTimestamp: timestamp
```

Capabilities flow through the type system. Calling an effectful function
adds its requirements to the caller unless discharged via handlers.

## Modules

Modules describe imports and exports in prose:

```zeus
module Web Server exposing { route root:, metrics stream }

dependencies
    import Prelude
    import Zeus Network Http as Http
    import Zeus Runtime Log
```

Exports use keyword signatures so tooling can surface the same readable
names.

## Desugaring Overview

During compilation, Zeus translates the surface syntax into a small core:

1. **Keyword functions** become lambda chains with named parameters.
2. **Pipelines** desugar to nested applications.
3. **Blocks** become closures with explicit capability sets.
4. **Records** compile to immutable structs with structural typing.

Because desugaring is predictable, error messages map back to the exact
keywords and clauses developers wrote.

## Tooling Hooks

The live runtime understands syntax nodes as first-class values. Editors
can select the phrase `Response ok: ... body:` and query its inferred
type, rewrite it, or replay its evaluation in the workspace. Syntax and
runtime stay in sync through the introspection fabric.

---

This syntax is intentionally ambitious. As the prototype matures we can
iterate on which surface forms feel natural, which ones need refinement,
and how the type checker surfaces feedback without breaking the prose-like
flow.
