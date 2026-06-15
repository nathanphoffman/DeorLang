# Deor Language Specification (Draft)

A small, indentation-based language that transpiles to Rust. Core influences: TypeScript's literal-derived typing (`as const`), Python's indentation and `for x in y`, and Go/C's prefix type declarations (`Type name`).

## Core Principles

- **No dots.** Field access is via destructuring (`area in room`), not `.field`.
- **No colons for blocks.** Indentation alone opens a block after a header keyword (`fn`, `if`, `for`, `type`, `struct`).
- **One statement per line.** Multi-line expressions only wrap inside `()` or `[]`.
- **`as`** = "derive this binding's type from its shape" — for scalar literals, list construction `[items]`, and record update `with`. No explicit type annotation is ever written with `as` (`count as 0`, not `int count as 0`). Use `Type name = expr` for function calls, computations, struct construction, and validator type bindings.
- **`in`** = "extract something from a source" — struct fields, collection elements, or module contents, all one grammar.
- **Structs are immutable.** Primitives and lists are mutable.
- **`is` is always structural equality**, regardless of how a struct is represented internally. `is not` is inequality. `&&`, `||`, `==`, `!=` are transpiler errors — use `and`, `or`, `is`, `is not`.
- **No lambdas.** Only named top-level `fn`s. No built-in `filter`/`map`/`reduce` — write explicit loops.
- **Void functions omit the return type.** `fn run()` returns nothing. No `void` keyword.
- **`shape` for named type aliases.** `shape roomList = list of Room` names a list type. `shape filterFunc = func of Room to bool` names a function signature. Shapes are camelCase and the only way to use parameterized types in Deor. Functions-as-values are passed by name as typed `func` shape parameters — no lambdas, no decorators.
- **Struct construction uses `Type name = (fields)`, always.** `Room room = (area, name)` — the type is always explicit, every field is a variable in scope matching the field name. No `{}`, no `field: value` pairs. Mirrors destructuring: `(area, name) in room` extracts, `Room room = (area, name)` constructs.
- **Validator types are option-types.** A `type` definition produces `Option<T>` under the hood — truthy when `Some`, falsy when `None`. Primitives and structs are never null. Three null-related forms: `Roll roll = none` (declare absent), `(avow roll)` (forced unwrap — panics if None), `roll else 0` (safe default).
- **Rust interop is a first-class escape hatch.** `rust` blocks drop into raw Rust inside any function. External `.rs` files import via `rust:myfile`. Cargo deps declared with `deps` blocks. `deor:` stdlib wraps common crates and `std` modules. `bytes` (`Vec<u8>`) is the boundary type for raw binary data.

## Index

- [Syntax](docs/syntax) — block structure, one statement per line
- [Functions](docs/functions) — `fn`, return rules, no lambdas
- [Variables](docs/variables) — `as`, explicit typing, reassignment
- [Types](docs/types) — validator types (`type`), structs (`struct` / `struct+` / `struct*`)
- [Shapes](docs/shapes) — `shape`, list shapes, func shapes, naming, file ordering
- [Collections](docs/collections) — list shapes, mutation verbs, index read
- [Conditionals](docs/conditionals) — `if`, `else if`, `else`, compact ternary form
- [Loops](docs/loops) — `for` collection and numeric iteration
- [Destructuring](docs/destructuring) — field extraction with `in`
- [Imports](docs/imports) — module imports with `in`
- [Immutability](docs/immutability) — immutability rules, equality, record update (`with`)
- [Examples](docs/examples) — full worked example with Rust translation
- [Operators](docs/operators) — arithmetic, comparison, logical, what's excluded
- [Strings](docs/strings) — concatenation, interpolation, length; [deor:strings](docs/strings) for contains, trim, split, etc.
- [Built-ins](docs/builtins) — `print`, `len`, `range`, math, random, type conversion
- [Open Questions](docs/open-questions) — future work and undecided areas
- [V2 Roadmap](docs/v2) — deferred features
- [Functions](docs/functions) — declaration, parameters (max 3), return rules, entry point
- [Interop](docs/interop) — `rust` blocks, `rust:` file imports, `bytes`, `deps`, `deor:` stdlib
- [Enforced Practices](docs/enforced_practices) — naming and ordering rules the transpiler warns on
- [Best Practices](docs/best_practices) — style recommendations not enforced by the transpiler
