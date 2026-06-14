# Types

## Validator Types (`type`)

A `type` definition wraps a base type with a predicate. The body is an implicit `bool` expression over the parameter. Construction is validated at use time (or transpile time for literals).

```
type Squarefeet(int n)
    n >= 0 and sqrt(n) == floor(sqrt(n))
```

```rust
#[derive(Clone, Copy, PartialEq, Debug)]
struct Squarefeet(i32);

impl Squarefeet {
    fn new(n: i32) -> Self {
        assert!(n >= 0 && (n as f64).sqrt().fract() == 0.0, "invalid Squarefeet: {}", n);
        Squarefeet(n)
    }
}
```

```
Squarefeet area = 9
```

```rust
let area = Squarefeet::new(9);
```

**Conversion notes:**
- This is a textbook Rust **newtype + smart constructor** — arguably more idiomatic in Rust than in any other target.
- **Compile-time-constant arguments** (`9`) should be validated at transpile time where possible — an invalid *literal* becomes a transpile error, not a runtime panic.
- **Dynamic values** always route through `Squarefeet::new(...)`, which panics on failure.
- `and` / `or` / `not` map to `&&` / `||` / `!`.
- For arithmetic between a validator type and its base type (e.g., `sum + area` where `sum: int`, `area: Squarefeet`), the transpiler unwraps via the generated tuple field (`sum += area.0`) — this `.0` is generated code, not source syntax, so it doesn't violate the no-dots rule.

---

## Structs (`struct` / `struct+` / `struct*`)

```
struct Room
    Squarefeet area
    string name
    bool occupied
```

```rust
#[derive(Clone, PartialEq, Debug)]
struct Room {
    area: Squarefeet,
    name: String,
    occupied: bool,
}
```

### Representation Sigils

| Form | Meaning | Rust representation |
|---|---|---|
| `struct Name` | Transpiler decides | `Name` (value) or `Rc<Name>` (reference), based on size + whether any field is an unsized `List<T>` |
| `struct+ Name` | Force value, always | `Name`, `.clone()` is a full (possibly deep) copy |
| `struct* Name` | Force reference, always | `Rc<Name>`, `.clone()` is a refcount bump |

```
struct House
    string address
    List<Room> rooms       # unsized List -> auto becomes struct*

struct+ House               # explicit override: always a value, full clone on copy
    string address
    List<Room> rooms
```

**Conversion notes:**
- The **struct definition itself is identical** regardless of `+`/`*`/auto — only how *usages* are represented changes (`House` vs `Rc<House>`).
- An **unsized `List<T>` field** makes a struct's clone cost O(n) and unbounded, so it defaults to `*` (reference) unless overridden with `+`.
- `==` is always `#[derive(PartialEq)]` on the underlying struct. `Rc<T>`'s default `PartialEq` already delegates to `T`'s impl in Rust, so structural equality holds for `struct*` types **with no extra work**.
- A struct containing only primitives and/or `List<T, N>` (fixed-size) fields has a fully known size and is `Copy`-eligible if every field is `Copy`.
