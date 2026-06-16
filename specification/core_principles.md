
# Core Principles

---

### Ease

- **Human not Symbols** - think words and ``()`` not: ``[]`` or ``{}`` or ``;`` or ``<>``  etc.
- **Book Readability** - tabbed blocks, all vars are 3+ chars, no magic data in fn params
- **Simple Safety** - structs are immutable (ref if containing list or struct), variables are mutable value-types
- **Uniform Composition** - destructuring order must match, functions limited to 3 parameters
- **Flat Structure** - no namespacing, nested fns, or closures

---

### Safety
- **Explicit over Generic** - explicit types are more clear (although ``shapes`` allow some minimal forms)
- **Validatior Types** - ``type`` expose a boolean return signature, which validates data (ie. Positive > 0)
- **1st-Class not Only-Class** - 1st class functions exist but are highly limited, and no lambdas are allowed
- **No OOP** - structs are data, but `using` exposes piping and ``with``/``in`` exposes easy (de)(con)struction
- **Explicit Typing** - types are required, shapes are explicit generics and functions, and enums are available
- **Fns are Always In/Out** - all functions are pure and therefore require in and out (except: main())

---

### Slimness
- **Slim Built-Ins** - only ``len``, ``range``, ``throw``, and ``print`` -- all others use rust wrappers
- **Only For**
- **Easy Exception Handling** - avow provides an easy test

---

### Rust Power
- **Rust Wrappers** - provides wrapped-Rust libs for advanced built-ins, no need to reinvent the wheel or learn Rust
- **Rust Blocks** - the ``rust`` block exposes raw Rust power, for when performance (or dict / bytes) matter
