
# Core Principles

---

### Ease

- **Human not Symbols** - think mostly words, occasional ``()``, and rare ```[]```, but not: ``{}`` or ``;`` or ``<>``  etc.
- **Book Readability** - tabbed blocks, all vars are 3+ chars, no magic data in fn params
- **Simple** - structs are immutable, variables and lists are mutable, everything is safely cloned
- **Uniform Composition** - destructuring order must match, functions limited to 3 parameters
- **Flat Structure** - no namespacing, nested fns, or closures
- **Easy Exception Handling** - avow and validator types replace Rust's Some/Option/None pattern

---

### Safety
- **Explicit over Generic** - explicit types are more clear (although ``shapes`` allow some minimal forms)
- **Validatior Types** - ``type`` exposes a boolean return signature, which validates data (ie. Positive > 0)
- **1st-Class not Only-Class** - 1st class functions exist but are highly limited, and no lambdas are allowed
- **No OOP** - structs are data, but `using` exposes piping and ``with``/``in`` exposes easy (de)(con)struction
- **Explicit Typing** - types are required, shapes are explicit generics and functions, and enums are available
- **Fns are Always In/Out** - all functions are pure, all data is cloned

---

### Slimness
- **Slim Built-Ins** - only ``len``, ``range``, ``throw``, and ``print`` -- all others use rust wrappers
- **Only For** - no while loop / for loop distinction, just for if and for in
---

### Rust Power
- **Rust Wrappers** - provides wrapped-Rust libs for advanced built-ins, no need to reinvent the wheel or learn Rust
- **Rust Blocks** - the ``rust`` block exposes raw Rust power, for when performance (or dict / bytes) matter
- **Rust Libs** - since everything builds to Rust, cargo can be pulled in and rust functions wrapped. see: [Libs and Shims](docs/shims.md)
