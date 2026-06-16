# Deor Language Specification (Draft)

Deor is a small, indentation-based language that transpiles to Rust. It enforces explicit types, named variables at every call site, and predictable control flow — then gets out of the way with a `rust` block when you need the full language.

---

## Core Principles

- **No dots.** Field access is via destructuring: `(area) in room`, not `room.area`.
- **No colons for blocks.** Indentation alone opens a block after `fn`, `if`, `for`, `type`, `struct`, or `enum`.
- **One statement per line.** Multi-line expressions only wrap inside `()`.
- **`as` infers type from shape.** Use it for scalar literals and list construction. Use `Type name = expr` for everything else — function calls, validator types, and struct construction.
- **`in` extracts from a source.** Struct fields, loop elements, slices, and imports all share this keyword.
- **Structs are immutable; primitives and lists are mutable.** Update a struct with `with` to get a modified copy.
- **`is` is structural equality.** `is not` is inequality. `&&`, `||`, `==`, `!=` are transpiler errors — use `and`, `or`, `is`, `is not`.
- **No lambdas.** Only named top-level `fn`s. No `filter`/`map`/`reduce` — write explicit loops.
- **`void` is mandatory for functions that return nothing.** `fn void run()` — omitting the return type is a transpiler error.
- **`shape` declares named type aliases.** `shape roomList = list of Room`, `shape filterFunc = func of Room to bool`. Shapes are camelCase and the only way to use parameterized types. Functions-as-values are passed by name as `func` shape parameters.
- **`enum` declares discriminated variant types.** `enum colorTag` with PascalCase variants. Assign with `colorTag color = Red`, check with `if color is Red`.
- **Struct construction is always `Type name = (fields)`.** Every field is a named variable in scope. No `{}`, no `field: value` pairs. Mirrors destructuring.
- **Validator types are `Option<T>`.** A `type` definition wraps a primitive with a predicate — truthy when `Some`, falsy when `None`. Use `empty` to initialize absent, `(avow val)` to forced-unwrap, `val else default` for a safe fallback.
- **Named arguments for user-defined functions only.** Literals and expressions are not valid arguments to user-defined functions. Built-ins (`print`, `len`, `range`) accept them freely.
- **`at` for list access.** `rooms at idx` reads; `rooms at idx = val` replaces; `rooms at end = val` appends.
- **`in range()` for slices.** `rooms in range(0, 10)` extracts a sublist; `end` as the upper bound means "length of this list."
- **Three built-ins: `print`, `len`, `range`.** Plus string shortcuts (`trim`, `contains`, `split`, etc.). Everything else is a shim — a Rust wrapper you copy from [Shims](docs/shims.md).
- **`rust` blocks are the escape hatch.** Drop into raw Rust inside any function. External `.rs` files import via `rust:myfile`. Cargo deps declared with `deps` blocks.

---

## Index

- [Syntax](docs/syntax.md) — block structure, keywords, comments
- [Functions](docs/functions.md) — `fn`, return rules, void, recursion
- [Variables](docs/variables.md) — `as`, explicit typing, struct construction, reassignment
- [Types](docs/types.md) — validator types (`type`), structs (`struct` / `struct+` / `struct*`)
- [Shapes](docs/shapes.md) — `shape`, list shapes, func shapes, naming, file ordering
- [Enums](docs/enums.md) — `enum`, variant declaration, checking with `is`
- [Collections](docs/collections.md) — index access, append, remove, slices
- [Conditionals](docs/conditionals.md) — `if`, `else if`, `else`, compact ternary
- [Loops](docs/loops.md) — `for` collection and numeric iteration
- [Destructuring](docs/destructuring.md) — field extraction with `in`
- [Imports](docs/imports.md) — local module and `rust:` file imports
- [Immutability](docs/immutability.md) — immutability rules, record update (`with`)
- [Operators](docs/operators.md) — arithmetic, comparison, logical, what's excluded
- [Strings](docs/strings.md) — concatenation, escape sequences, string built-ins
- [Built-ins](docs/builtins.md) — `print`, `len`, `range`, and string shortcuts
- [Shims](docs/shims.md) — copy-paste Rust wrappers for math, random, parsing, and more
- [Interop](docs/interop.md) — `rust` blocks, `raw` variables, `rust:` imports, `deps`
- [Enforced Practices](docs/enforced_practices.md) — naming and ordering rules the transpiler enforces
- [Best Practices](docs/best_practices.md) — style recommendations not enforced by the transpiler
- [Examples](docs/examples.md) — full worked example with Rust translation
- [Open Questions](docs/open-questions.md) — future work and undecided areas
- [V2 Roadmap](docs/v2.md) — deferred features
