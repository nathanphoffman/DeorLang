# Deor Language Specification

Deor is a small, highly-procedural, tabbed-block language that transpiles to Rust. It enforces near-book readability, explicit typing, and predictable control flow — and exposes a `rust` block for when you need the full language.

Its goal is to provide a comfier entrance point to Rust with simple syntactical sugar and such uniform composition rules that there is little room for debate, and when there is debate -- then it offers ``rust`` blocks for that.

```
To get started
```

It took inspiration from, the following languages in this order:
- **Python** - block scopes and heavily opinionated readability -- Python doesn't go far enough! ;) 
- **Rust** - transpiles to it, allows raw rust blocks, supports a lot of similar patterns
- **Go** - a simple ultra light-weight base syntax
- **C#** - data types and data typing style
- **DD Design** - built in validator types
- **Haskell/React State** - immutable objects with exposed (de)construction tools (in/with/as)

---

## Core Principles
- **Human Symbols Only** - no [] or {} or ; or <>  etc.
- **Reads Like a Book** - tabbed blocks, all vars are 3+ chars, no magic data in fn params
- **No OOP** - structs are data, however `using` exposes readable piping to make struct composition easier
- **Uniform Composition** - destructuring order must match, functions limited to 3 parameters
- **Explicit over Generic** - explicit types are more clear (although ``shapes`` allow some minimal forms)
- **1st-Class not Only-Class** - 1st class functions exist but are highly limited, and no lambdas are allowed
- **Data**

- **`in` extracts from a source.** Struct fields, loop elements, slices, and imports all share this keyword.
- **`using var` scopes a subject.** Inside a `using` block, zero-arg calls like `step()` are shimmed to `var = step(var.clone())`. Pass an extra argument with `step() with extra`.
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
- **Three built-ins: `print`, `len`, `range`.** Everything else — including string operations — lives in `lib/` or is a shim from [Shims](docs/shims.md).
- **`rust` blocks are the escape hatch.** Drop into raw Rust inside any function. External `.rs` files import via `rust:myfile`. Cargo deps declared with `deps` blocks.

---

## Index

- [Syntax](docs/syntax.md) — block structure, keywords, comments
- [Functions](docs/functions.md) — `fn`, return rules, void, recursion, `using` blocks
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
- [Built-ins](docs/builtins.md) — `print`, `len`, `range`
- [Shims](docs/shims.md) — copy-paste Rust wrappers for math, random, parsing, and more
- [Interop](docs/interop.md) — `rust` blocks, `raw` variables, `rust:` imports, `deps`
- [Enforced Practices](docs/enforced_practices.md) — naming and ordering rules the transpiler enforces
- [Best Practices](docs/best_practices.md) — style recommendations not enforced by the transpiler
- [Examples](docs/examples.md) — full worked example with Rust translation
- [Open Questions](docs/open-questions.md) — future work and undecided areas
- [V2 Roadmap](docs/v2.md) — deferred features
