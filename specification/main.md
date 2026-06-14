# Deor Language Specification (Draft)

A small, indentation-based language that transpiles to Rust. Core influences: TypeScript's literal-derived typing (`as const`), Python's indentation and `for x in y`, and Go/C's prefix type declarations (`Type name`).

## Core Principles

- **No dots.** Field access is via destructuring (`area in room`), not `.field`.
- **No colons for blocks.** Indentation alone opens a block after a header keyword (`fn`, `if`, `for`, `type`, `struct`).
- **One statement per line.** Multi-line expressions only wrap inside `()`, `[]`, or `{}`.
- **`as`** = "derive this binding's type/shape from a literal" (compile-time only).
- **`in`** = "extract something from a source" — struct fields, collection elements, or module contents, all one grammar.
- **Structs are immutable.** Primitives and lists are mutable.
- **`==` is always structural**, regardless of how a struct is represented internally.
- **No lambdas.** Only named `fn`s (top-level or nested).

## Index

- [Syntax](docs/syntax) — block structure, one statement per line
- [Functions](docs/functions) — `fn`, return rules, no lambdas
- [Variables](docs/variables) — `as`, explicit typing, reassignment
- [Types](docs/types) — validator types (`type`), structs (`struct` / `struct+` / `struct*`)
- [Collections](docs/collections) — `List<T>`, `List<T, N>`, mutation verbs
- [Loops](docs/loops) — `for` collection and numeric iteration
- [Destructuring](docs/destructuring) — field extraction with `in`
- [Imports](docs/imports) — module imports with `in`
- [Immutability](docs/immutability) — immutability rules, equality, record update (`with`)
- [Examples](docs/examples) — full worked example with Rust translation
- [Open Questions](docs/open-questions) — future work and undecided areas
