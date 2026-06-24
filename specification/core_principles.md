
# Core Principles

---

### Ease

- **Human not Symbols** - think mostly words, occasional `()`, and rare `[]`, but not: `{}` or `;` or `<>` etc.
- **Book Readability** - tabbed blocks, all vars are 3+ chars, no magic data in fn params
- **Simple** - structs are immutable, variables and lists are mutable, everything is safely cloned
- **Uniform Composition** - destructuring order must match, functions limited to 3 parameters
- **Flat Structure** - no namespacing, nested fns, or closures
- **Easy Exception Handling** - validator types with `valid`/`not valid` replace Rust's `Option`/`Some`/`None` pattern

---

### Safety
- **Explicit over Generic** - explicit types are more clear (although `shapes` allow some minimal forms)
- **Validator Types** - `type` exposes a boolean predicate that validates data (e.g. `Positive > 0`)
- **1st-Class not Only-Class** - first-class functions exist but are highly limited, and no lambdas are allowed
- **No OOP** - structs are data, and `with`/`in` exposes easy construction and destructuring
- **Explicit Typing** - types are required, shapes are explicit generics and functions, and enums are available
- **Fns are Always In/Out** - all data flows through parameters and return values; all arguments are cloned by default

---

### Slimness
- **Slim Built-Ins** - only `len`, `range`, `crash`, and `print` -- all others use rust wrappers
- **Only For** - no while/for distinction: `for if` is the while loop, `for in` is collection/range iteration

---

### Rust Power
- **Rust Wrappers** - provides wrapped-Rust libs for advanced built-ins, no need to reinvent the wheel or learn Rust
- **Rust Blocks** - the `rust` block exposes raw Rust power, for when performance (or dict / bytes) matter
- **Rust Libs** - since everything builds to Rust, cargo can be pulled in and rust functions wrapped. see: [Libs](docs/libs.md)
