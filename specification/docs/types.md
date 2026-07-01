# Types

## Primitive Types

Deor's built-in primitive types and their Rust equivalents:

| Deor | Rust | Notes |
|---|---|---|
| `int` | `i64` | General-purpose integer |
| `float` | `f64` | General-purpose decimal |
| `bool` | `bool` | |
| `string` | `String` | Owned; available as `&str` via `.as_str()` in `rust` blocks |

For raw binary data (HTTP bodies, files, crypto, pixel buffers) use a `raw` variable and handle it entirely inside `rust` blocks. See [`raw` Variables](#raw-variables) below.

Integer literals may contain underscores as visual separators:
```
int val = 1_000_000
```
---

## `raw` Variables

Some things are awkward to build in Deor at all — a `HashMap`, a compiled regex, a connection pool. `raw` is the escape hatch: a `rust` block builds the thing once, hands it back as an opaque value, and Deor carries that value around without needing to understand what's inside it. Because Deor can't see inside a `raw`, it also can't validate it — a `raw` has no type annotation, no Deor operators, and can't appear in Deor expressions or struct fields. It's only ever produced by a `rust` block and only ever consumed inside one.

```
raw index = rust
    entries.iter()
        .map(|e| (e.key.clone(), e.value.clone()))
        .collect::<std::collections::HashMap<String, String>>()
```

See [Rust Interop](docs/interop.md) for full documentation, rules, and the build-once pattern.

---

## Validator Types (`type`)

A type that carries its own "is this actually valid?" check, for values that can be built fine but still not make sense — a negative area, an out-of-range roll, or simply nothing assigned yet (Deor's stand-in for `null`/`undefined`). This is a large enough feature to have its own page: see [Validator Types](docs/validator_types.md) for how it works, declaration rules, `is valid`/`is not valid`, `avow`, struct fields, and function returns.

---

## Truthiness

Implicit truthiness hides a decision — is `if my_int` checking for nonzero, or for "was this ever set"? Deor makes you write the comparison you actually mean. **Only `bool` and validator types have a presence check.** Plain `int`, `float`, `string`, `list`, and structs are never truthy or falsy on their own — use explicit comparisons:

```
if len(my_list) > 0    # correct — explicit non-empty check
if my_list             # transpiler error — list has no truthiness

if my_int is not 0     # correct
if my_int              # transpiler error

if my_string is not "" # correct
if my_string           # transpiler error
```

Validator types use `is valid` / `is not valid` — not bare truthiness. See [Validator Types — is valid / is not valid](docs/validator_types.md#is-valid-is-not-valid).

```
if sqft is valid        # correct
if sqft is not valid    # correct
if sqft                 # transpiler error — use is valid/is not valid
```

```rust
if area.is_some() {
    let val: i64 = area.unwrap().0;
}
if area.is_none() {
    // not valid
}
```

---

## Structs (`struct`)

```
struct Room
    Squarefeet area
    string name
    bool occupied
```

```rust
#[derive(Clone, PartialEq, Debug)]
struct Room {
    area: Option<Squarefeet>,
    name: String,
    occupied: bool,
}
```

Struct fields may be primitives, validator types, list shapes, or other structs. Func shapes as struct fields are a transpiler error — structs are pure data.

There are no per-field visibility modifiers — all fields are always accessible via destructuring whenever the struct itself is in scope.
