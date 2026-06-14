# Types

## Primitive Types

Deor's built-in primitive types and their Rust equivalents:

| Deor | Rust | Notes |
|---|---|---|
| `int` | `i32` | General-purpose integer |
| `float` | `f64` | General-purpose decimal |
| `bool` | `bool` | |
| `string` | `String` | Owned; available as `&str` via `.as_str()` in `rust` blocks |
| `bytes` | `Vec<u8>` | Raw binary data — HTTP bodies, files, crypto, pixel buffers |

`bytes` is the bridge for raw binary data crossing the Deor/Rust boundary. Any byte-level processing (bit manipulation, `u8`/`u16` arithmetic, SIMD) happens inside `rust` blocks; `bytes` carries the data in and out.

```
fn bytes read_raw(string path)
    rust
        std::fs::read(path.as_str())
            .unwrap_or_default()

fn int byte_count(bytes data)
    rust
        data.len() as i32
```

---

## Validator Types (`type`)

A `type` definition wraps a base primitive with a predicate. The predicate body is required — a `type` with no constraint adds no meaning over the base type, so use the base type directly instead. The transpiler errors on a `type` definition with an empty body.

The body is an implicit `bool` expression over the parameter. A validator type is always `Option<T>` under the hood — assignment runs the predicate at runtime; if it passes the value is `Some`, if it fails the value is `None`. Primitives and structs are never null — only validator types carry presence/absence.

```
type Squarefeet(int n)
    n >= 0 and sqrt(n) == floor(sqrt(n))
```

```rust
#[derive(Clone, Copy, PartialEq, Debug)]
struct Squarefeet(i32);

impl Squarefeet {
    fn new(n: i32) -> Option<Self> {
        if n >= 0 && (n as f64).sqrt().fract() == 0.0 {
            Some(Squarefeet(n))
        } else {
            None
        }
    }
}
```

```
Squarefeet area = 9     # Some(Squarefeet(9))
Squarefeet bad = -1     # None — predicate fails silently
```

```rust
let area: Option<Squarefeet> = Squarefeet::new(9);
let bad: Option<Squarefeet> = Squarefeet::new(-1);
```

---

### Truthy / Falsy

A validator type variable is truthy when `Some`, falsy when `None`. Use `if` / `if not` to check presence before using the value.

```
Squarefeet area = 9
if area
    int n = (area is known)
if not area
    print("no value")
```

```rust
if area.is_some() {
    let n: i32 = area.unwrap().0;
}
if area.is_none() {
    print("no value");
}
```

---

### Declaring None

A validator type variable can be explicitly initialized to `none` at declaration. This is only valid at the point of first declaration — `= none` after a variable has been declared is a transpiler error.

```
Roll best = none
Squarefeet area = none
```

```rust
let mut best: Option<Roll> = None;
let mut area: Option<Squarefeet> = None;
```

---

### Forced Unwrap — `is known`

`(v is known)` asserts the value is `Some` and extracts the inner primitive. Panics at runtime if `None`. Use only when you are certain the value is present — typically inside an `if v` block where presence is already confirmed.

```
Roll r = roll_die(d20)
if r
    int n = (r is known)    # always safe inside if r
```

```rust
if r.is_some() {
    let n: i32 = r.unwrap().0;
}
```

Outside an `if` check, this is the programmer's assertion that the value is Some:

```
int sum = (value is known) + 2
```

```rust
let sum: i32 = value.unwrap().0 + 2;
```

---

### Safe Default — `else`

`value else default` returns the inner primitive if `Some`, or the default if `None`. Always safe — no panic risk.

```
int n = area else 0
```

```rust
let n: i32 = area.map(|v| v.0).unwrap_or(0);
```

`else` here is null-coalescing — distinct from `if/else` block syntax. `or` and `and` remain the logical `||` / `&&` operators.

---

### Validator Types in Structs

Struct fields typed as a validator type are `Option<T>` under the hood. Extracting them with `in` preserves the Option — the extracted variable is still truthy/falsy and must be checked before use.

```
struct Room
    Squarefeet area
    Roll max_capacity
```

```
(area, max_capacity) in room
if max_capacity
    int cap = (max_capacity is known)
int safe_cap = max_capacity else 0
```

```rust
let (area, max_capacity) = (room.area, room.max_capacity);
if max_capacity.is_some() {
    let cap: i32 = max_capacity.unwrap().0;
}
let safe_cap: i32 = max_capacity.map(|v| v.0).unwrap_or(0);
```

---

### Functions Returning Validator Types

A function whose return type is a validator type may return a `None` value through its return variable. `return none` is a transpiler error — always return a named typed variable. The caller knows the return may be `None` because the return type is a validator type.

```
fn Roll find_crit(list<RollResult> rolls)
    Roll found = none

    for roll in rolls
        value in roll
        if is_critical(roll)
            found = value

    return found
```

```rust
fn find_crit(rolls: &Vec<RollResult>) -> Option<Roll> {
    let mut found: Option<Roll> = None;
    for roll in rolls {
        let value = roll.value;
        if is_critical(roll) {
            found = value;
        }
    }
    found
}
```

The caller uses `if` or `else` to handle the result:

```
Roll crit = find_crit(rolls)
int bonus = crit else 0
```

---

**Conversion notes:**
- Constructor becomes `fn new(n: T) -> Option<Self>` — never panics, returns `None` on predicate failure.
- Truthy/falsy maps to `.is_some()` / `.is_none()`.
- `is known` → `.unwrap().0`; `value else default` → `.map(|v| v.0).unwrap_or(default)`.
- Equality (`==`) falls through to Rust's `Option<T>: PartialEq` — `None == None` is true, `Some(x) == Some(y)` compares inner values structurally.
- `and` / `or` / `not` map to `&&` / `||` / `!`.
- Literal predicate failures (`Squarefeet bad = -1`) will eventually be caught at transpile time — currently runtime `None`.

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
    area: Option<Squarefeet>,
    name: String,
    occupied: bool,
}
```

### Representation Sigils

| Form | Meaning | Rust representation |
|---|---|---|
| `struct Name` | Transpiler decides | `Name` (value) or `Rc<Name>` (reference), based on size + whether any field is an unsized `list<T>` |
| `struct+ Name` | Force value, always | `Name`, `.clone()` is a full (possibly deep) copy |
| `struct* Name` | Force reference, always | `Rc<Name>`, `.clone()` is a refcount bump |

```
struct House
    string address
    list<Room> rooms       # unsized list -> auto becomes struct*

struct+ House               # explicit override: always a value, full clone on copy
    string address
    list<Room> rooms
```

**Conversion notes:**
- The **struct definition itself is identical** regardless of `+`/`*`/auto — only how *usages* are represented changes (`House` vs `Rc<House>`).
- An **unsized `list<T>` field** makes a struct's clone cost O(n) and unbounded, so it defaults to `*` (reference) unless overridden with `+`.
- `==` is always `#[derive(PartialEq)]` on the underlying struct. `Rc<T>`'s default `PartialEq` already delegates to `T`'s impl in Rust, so structural equality holds for `struct*` types **with no extra work**.
- A struct containing only primitives and/or `list<T, N>` (fixed-size) fields has a fully known size and is `Copy`-eligible if every field is `Copy`.
