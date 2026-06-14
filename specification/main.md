# Deor Language Specification (Draft)

A small, indentation-based language that transpiles to Rust. Core influences: TypeScript's literal-derived typing (`as const`), Python's indentation and `for x in y`, and Go/C's prefix type declarations (`Type name`).

## Core Principles

- **No dots.** Field access is via destructuring (`area in room`), not `.field`.
- **No colons for blocks.** Indentation alone opens a block after a header keyword (`fn`, `if`, `for`, `type`, `struct`).
- **One statement per line.** Multi-line expressions only wrap inside `()` or `[]`.
- **`as`** = "derive this binding's type/shape from a literal" (compile-time only).
- **`in`** = "extract something from a source" — struct fields, collection elements, or module contents, all one grammar.
- **Structs are immutable.** Primitives and lists are mutable.
- **`==` is always structural**, regardless of how a struct is represented internally.
- **No lambdas.** Only named `fn`s (top-level or nested). No built-in `filter`/`map`/`reduce` — write explicit loops.
- **Void functions omit the return type.** `fn run()` returns nothing. No `void` keyword.
- **`[using alias: T->O]` for behavior injection.** Annotate a function to accept a named external function. Call site provides it with `using fn_name`. Alias is called in the body. Required — omitting `using fn_name` at the call site is a transpiler error.
- **Struct construction uses `()`, always.** `room as (area, name)` — every field is a variable already in scope matching the field name. No `{}`, no `field: value` pairs. Mirrors destructuring: `(area, name) in room` extracts, `room as (area, name)` constructs.
- **Validator types are option-types.** A `type` definition produces `Option<T>` under the hood — truthy when `Some`, falsy when `None`. Primitives and structs are never null. Three null-related forms: `Roll r = none` (declare absent), `(r is known)` (forced unwrap — panics if None), `r else 0` (safe default).
- **Rust interop is a first-class escape hatch.** `rust` blocks drop into raw Rust inside any function. External `.rs` files import via `rust:myfile`. Cargo deps declared with `deps` blocks. `deor:` stdlib wraps common crates and `std` modules. `bytes` (`Vec<u8>`) is the boundary type for raw binary data.

## Index

- [Syntax](docs/syntax) — block structure, one statement per line
- [Functions](docs/functions) — `fn`, return rules, no lambdas
- [Variables](docs/variables) — `as`, explicit typing, reassignment
- [Types](docs/types) — validator types (`type`), structs (`struct` / `struct+` / `struct*`)
- [Collections](docs/collections) — `list<T>`, `list<T, N>`, mutation verbs
- [Conditionals](docs/conditionals) — `if`, `else if`, `else`, compact ternary form
- [Loops](docs/loops) — `for` collection and numeric iteration
- [Destructuring](docs/destructuring) — field extraction with `in`
- [Imports](docs/imports) — module imports with `in`
- [Immutability](docs/immutability) — immutability rules, equality, record update (`with`)
- [Examples](docs/examples) — full worked example with Rust translation
- [Open Questions](docs/open-questions) — future work and undecided areas
- [Annotations](docs/functions#annotations) — `[test]`, `[deprecated]`, `[pure]`, `[using]`
- [Interop](docs/interop) — `rust` blocks, `rust:` file imports, `bytes`, `deps`, `deor:` stdlib
- [Enforced Practices](docs/enforced_practices) — naming and ordering rules the transpiler warns on
- [Best Practices](docs/best_practices) — style recommendations not enforced by the transpiler
