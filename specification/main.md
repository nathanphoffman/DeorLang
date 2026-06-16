# Deor Language Specification (Draft)

A small, indentation-based language that transpiles to Rust. Core influences: TypeScript's literal-derived typing (`as const`), Python's indentation and `for x in y`, and Go/C's prefix type declarations (`Type name`).

## Core Principles

- **No dots.** Field access is via destructuring (`area in room`), not `.field`.
- **No colons for blocks.** Indentation alone opens a block after a header keyword (`fn`, `if`, `for`, `type`, `struct`, `enum`, `using`).
- **One statement per line.** Multi-line expressions only wrap inside `()`.
- **`as`** = "derive this binding's type from its shape" — for scalar literals, list construction `[items]`, and record update `with`. No explicit type annotation is ever written with `as` (`count as 0`, not `int count as 0`). Use `Type name = expr` for function calls, computations, struct construction, and validator type bindings.
- **`in`** = "extract something from a source" — struct fields, collection elements, list slices, or module contents, all one grammar.
- **Structs are immutable.** Primitives and lists are mutable.
- **`is` is always structural equality**, regardless of how a struct is represented internally. `is not` is inequality. `&&`, `||`, `==`, `!=` are transpiler errors — use `and`, `or`, `is`, `is not`.
- **No lambdas.** Only named top-level `fn`s. No built-in `filter`/`map`/`reduce` — write explicit loops.
- **Void functions use `void` as the return type.** `fn void run()` returns nothing. `void` is mandatory — omitting the return type is a transpiler error.
- **`shape` for named type aliases.** `shape roomList = list of Room` names a list type. `shape filterFunc = func of Room to bool` names a function signature. Shapes are camelCase and the only way to use parameterized types in Deor. Functions-as-values are passed by name as typed `func` shape parameters — no lambdas, no decorators. `shape` is always a pure type alias — never a value you hold or compare.
- **`enum` for named variant types.** `enum colorTag` with an indented list of PascalCase variant names declares a discriminated type. Assign with `colorTag color = Red`, check with `if color is Red`. Enums are camelCase names; variants are PascalCase. They are the only instantiable types declared with a camelCase name.
- **Struct construction uses `Type name = (fields)`, always.** `Room room = (area, name)` — the type is always explicit, every field is a variable in scope matching the field name. No `{}`, no `field: value` pairs. Mirrors destructuring: `(area, name) in room` extracts, `Room room = (area, name)` constructs.
- **Validator types are option-types.** A `type` definition produces `Option<T>` under the hood — truthy when `Some`, falsy when `None`. Primitives and structs are never null. Three null-related forms: `Roll roll = none` (declare absent), `(avow roll)` (forced unwrap — panics if None), `roll else 0` (safe default).
- **Named arguments for user-defined functions only.** Arguments to user-defined functions must be named variables in scope — no inline literals or expressions. Built-in functions (`print`, `len`, `range`, `sqrt`, etc.) accept literals and expressions directly.
- **`at` for list index access and write.** `rooms at idx` reads; `rooms at idx = val` replaces; `rooms at end = val` appends. `end` is a reserved keyword meaning "end of this list."
- **`in range()` for slices.** `rooms in range(0, 10)` extracts a sublist. `end` as the upper bound means "length of this list."
- **All built-ins available without import.** `print`, `len`, `range`, math, strings, random, parsing — no import needed. `rust` blocks and `deps` handle everything else.
- **Rust interop is a first-class escape hatch.** `rust` blocks drop into raw Rust inside any function. External `.rs` files import via `rust:myfile`. Cargo deps declared with `deps` blocks. `bytes` (`Vec<u8>`) is the boundary type for raw binary data.

## Index

- [Syntax](docs/syntax.md) — block structure, one statement per line
- [Functions](docs/functions.md) — `fn`, return rules, no lambdas
- [Variables](docs/variables.md) — `as`, explicit typing, reassignment
- [Types](docs/types.md) — validator types (`type`), structs (`struct` / `struct+` / `struct*`)
- [Shapes](docs/shapes.md) — `shape`, list shapes, func shapes, bytes shapes, naming, file ordering
- [Enums](docs/enums.md) — `enum`, variant declaration, assignment, checking with `is`
- [Collections](docs/collections.md) — `at` index access, `at end` append, `remove at`, `in range()` slices
- [Conditionals](docs/conditionals.md) — `if`, `else if`, `else`, compact ternary form
- [Loops](docs/loops.md) — `for` collection and numeric iteration, `for range()` without variable
- [Destructuring](docs/destructuring.md) — field extraction with `in`
- [Using Blocks](docs/using.md) — state-threading through a sequence of function calls
- [Imports](docs/imports.md) — local module and `rust:` file imports
- [Immutability](docs/immutability.md) — immutability rules, equality, record update (`with`)
- [Examples](docs/examples.md) — full worked example with Rust translation
- [Operators](docs/operators.md) — arithmetic, comparison, logical, what's excluded
- [Strings](docs/strings.md) — concatenation, interpolation, length, string built-ins
- [Built-ins](docs/builtins.md) — all built-in functions; no import required
- [Interop](docs/interop.md) — `rust` blocks, `rust:` file imports, `bytes`, `deps`
- [Enforced Practices](docs/enforced_practices.md) — naming and ordering rules the transpiler enforces
- [Best Practices](docs/best_practices.md) — style recommendations not enforced by the transpiler
- [Open Questions](docs/open-questions.md) — future work and undecided areas
- [V2 Roadmap](docs/v2.md) — deferred features
