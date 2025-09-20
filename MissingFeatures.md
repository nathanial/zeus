# Missing Features from Common Lisp

This document outlines the features that Zeus currently lacks compared to a full Common Lisp implementation. Features are organized by category and priority.

## Core Language Features

### Special Forms & Control Flow
- [x] `let` and `let*` - Local variable bindings ✅
- [ ] `letrec` - Recursive local bindings
- [x] `cond` - Multi-branch conditional ✅
- [ ] `case` - Value-based branching
- [x] `and` / `or` - Logical operators with short-circuit evaluation ✅
- [ ] `when` / `unless` - Single-branch conditionals
- [ ] `progn` / `begin` - Sequential evaluation
- [ ] `do` / `loop` - Iteration constructs
- [ ] `catch` / `throw` - Non-local control transfer
- [ ] `unwind-protect` - Cleanup guarantees
- [ ] `block` / `return-from` - Named blocks
- [ ] `tagbody` / `go` - Low-level control flow

### Data Types
- [ ] **Symbols**
  - [ ] Symbol properties (plist)
  - [ ] Uninterned symbols (gensym)
  - [ ] Keywords (self-evaluating symbols)
- [ ] **Numbers**
  - [ ] Integers (arbitrary precision)
  - [ ] Rationals
  - [ ] Complex numbers
  - [ ] Type-specific operations
- [ ] **Characters** - Distinct from strings
- [ ] **Arrays** - Multi-dimensional arrays
- [ ] **Hash Tables** - Key-value maps
- [ ] **Structures** - User-defined types
- [ ] **Classes** (CLOS) - Object-oriented programming
- [ ] **Packages** - Namespace management
- [ ] **Streams** - I/O abstraction

### List Operations
- [x] `append` - List concatenation ✅
- [x] `reverse` - List reversal ✅
- [x] `length` - List/sequence length ✅
- [x] `nth` - N-th element access ✅
- [ ] `nthcdr` - N-th cdr access
- [ ] `member` / `assoc` - List searching
- [ ] `mapcar` / `maplist` - List mapping
- [ ] `reduce` - List reduction
- [ ] `filter` / `remove` - List filtering
- [ ] `sort` - List sorting
- [ ] `subseq` - Subsequence extraction
- [ ] Destructive operations (`nconc`, `rplaca`, `rplacd`)

### Functions & Closures
- [ ] `funcall` - Explicit function call
- [ ] `apply` - Apply function to list of arguments
- [ ] `&optional` - Optional parameters
- [ ] `&rest` - Variable number of arguments
- [ ] `&key` - Keyword parameters
- [ ] `&aux` - Auxiliary variables
- [ ] `flet` / `labels` - Local function definitions
- [ ] `function` special form (`#'` reader macro)
- [ ] Multiple return values (`values`, `multiple-value-bind`)

### Macros
- [ ] `defmacro` - Macro definition
- [ ] `macroexpand` - Macro expansion
- [ ] Backquote (`` ` ``) - Template syntax
- [ ] Comma (`,`) - Unquote in templates
- [ ] Comma-at (`,@`) - Splice in templates
- [ ] `gensym` - Generate unique symbols
- [ ] Compiler macros
- [ ] Reader macros

### Type System
- [ ] `deftype` - Type definitions
- [ ] `typep` - Type checking
- [ ] `coerce` - Type conversion
- [ ] `check-type` - Type assertions
- [ ] `declare` - Type declarations
- [ ] `the` - Type hints
- [ ] Type specifiers (satisfies, member, or, and, not)

### Error Handling
- [ ] Condition system
- [ ] `error` / `signal` - Raise conditions
- [ ] `handler-case` / `handler-bind` - Exception handling
- [ ] `restart-case` / `restart-bind` - Restarts
- [ ] `warn` - Warnings
- [ ] Debugger integration

### I/O & Formatting
- [ ] `read` - Read S-expressions
- [ ] `print` / `prin1` / `princ` - Output functions
- [ ] `format` - Formatted output
- [ ] `with-open-file` - File handling
- [ ] Pretty printing
- [ ] Read macros
- [ ] Custom print methods

### Evaluation & Compilation
- [ ] `eval` - Explicit evaluation
- [ ] `compile` - Function compilation
- [ ] `load` - Load source files
- [ ] `require` / `provide` - Module loading
- [ ] Compiler optimizations
- [ ] Inline declarations
- [ ] Special variable declarations

### CLOS (Common Lisp Object System)
- [ ] `defclass` - Class definition
- [ ] `defmethod` - Method definition
- [ ] `defgeneric` - Generic function definition
- [ ] Multiple inheritance
- [ ] Method combination
- [ ] Slot options (readers, writers, initargs)
- [ ] `initialize-instance` - Construction protocol
- [ ] `make-instance` - Object creation
- [ ] `with-slots` / `with-accessors` - Slot access
- [ ] Method qualifiers (before, after, around)
- [ ] EQL specializers

### Advanced Features
- [ ] **Multiple Values**
  - [ ] `values` - Return multiple values
  - [ ] `multiple-value-bind` - Receive multiple values
  - [ ] `multiple-value-call` - Call with multiple values
- [ ] **Dynamic Variables**
  - [ ] `defparameter` / `defvar` - Global dynamic variables
  - [ ] `let` with special declarations
  - [ ] Thread-local bindings
- [ ] **Compiler Macros** - Optimization hints
- [ ] **Symbol Macros** - Symbol expansion
- [ ] **Method Combinations** - Complex dispatch
- [ ] **MOP** (Meta-Object Protocol) - Meta-programming

### Standard Library
- [ ] **Math Functions**
  - [ ] Trigonometric functions
  - [ ] Logarithms and exponentials
  - [ ] Random numbers
  - [ ] Bitwise operations
- [ ] **String Operations**
  - [ ] String comparison
  - [ ] String searching
  - [ ] String manipulation
  - [ ] Case conversion
- [ ] **Sequence Operations**
  - [ ] Generic sequence functions
  - [ ] Sequence predicates
  - [ ] Sequence searching
- [ ] **File System**
  - [ ] Directory operations
  - [ ] Path manipulation
  - [ ] File attributes
- [ ] **Time & Date**
  - [ ] Current time
  - [ ] Time arithmetic
  - [ ] Time formatting

### Reader Features
- [ ] Reader macros (`#` dispatch)
- [ ] `#'` - Function quote
- [ ] `#(` - Vector literal
- [ ] `#\` - Character literal
- [ ] `#|...|#` - Multi-line comments
- [ ] `#+` / `#-` - Conditional reading
- [ ] `#.` - Read-time evaluation
- [ ] `#p` - Pathname literal
- [ ] Custom reader macros

### Environment & Introspection
- [ ] `describe` - Object description
- [ ] `inspect` - Interactive inspection
- [ ] `documentation` - Documentation strings
- [ ] `apropos` - Symbol searching
- [ ] `trace` / `untrace` - Function tracing
- [ ] `time` - Performance measurement
- [ ] `room` - Memory usage
- [ ] `disassemble` - Show compiled code

### Performance & Optimization
- [ ] Tail call optimization
- [ ] Inline functions
- [ ] Compiler declarations
- [ ] Type inference
- [ ] Constant folding
- [ ] Dead code elimination
- [ ] Loop optimizations

## Current Limitations

### Implementation Details
- No persistent environment between REPL sessions
- No file loading or module system
- No compilation (interpreter only)
- Limited error messages and debugging
- No garbage collection considerations
- No thread safety

### Type System
- Only supports floating-point numbers
- No distinction between different numeric types
- No type checking or declarations
- No user-defined types

### Memory Management
- Simple cloning-based value semantics
- No consideration for circular references
- No weak references
- No finalization

## Priority for Future Development

### High Priority
1. `let` / `let*` for local bindings
2. `cond` for multi-branch conditionals
3. `append`, `reverse`, `length` for lists
4. `defmacro` for metaprogramming
5. Better error handling with line numbers
6. File loading with `load`

### Medium Priority
1. Numeric types (integers, rationals)
2. `apply` and `funcall`
3. Optional and rest parameters
4. Hash tables
5. String operations
6. `format` for output

### Low Priority
1. CLOS object system
2. Condition system
3. Multiple values
4. Compiler and optimizations
5. MOP
6. Advanced reader macros

## Notes

This implementation focuses on educational simplicity rather than full Common Lisp compliance. Some features may be intentionally simplified or omitted to maintain clarity and ease of understanding.