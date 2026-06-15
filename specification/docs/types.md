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

The body evaluates to a `bool`. Simple predicates are a single boolean expression; predicates that need intermediate values may declare bindings before the final bool expression, following the same rules as a function body.

A validator type is always `Option<T>` under the hood — assignment runs the predicate at runtime; if it passes the value is `Some`, if it fails the value is `None`. Primitives and structs are never null — only validator types carry presence/absence.

```
type Squarefeet(int val)
    float flt = to_float(val)
    NonNegFloat root_nf = sqrt(flt)
    float root_f = root_nf else 0.0
    int root = floor(root_f)
    root * root is val
```

`sqrt` returns `NonNegFloat` (a stdlib validator type — `None` for negative input). Each call result is stored before being passed to the next function. Using `else 0.0` recovers safely: a negative `val` makes `sqrt` return `None`, `else` gives `0.0`, `floor` gives `0`, and `0 * 0 is val` fails — no separate `val >= 0` guard needed.

```rust
#[derive(Clone, Copy, PartialEq, Debug)]
struct Squarefeet(i32);

impl Squarefeet {
    fn new(val: i32) -> Option<Self> {
        let root: i32 = NonNegFloat::new(val as f64).map(|v| v.0).unwrap_or(0.0).floor() as i32;
        if root * root == val {
            Some(Squarefeet(val))
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

**Only validator types and `bool` have truthiness.** Plain `int`, `float`, `string`, `list<T>`, and structs are never truthy or falsy on their own — they have no presence/absence concept. Use explicit comparisons instead:

```
if len(my_list) > 0    # correct — explicit non-empty check
if my_list             # transpiler error — list has no truthiness

if my_int != 0         # correct
if my_int              # transpiler error

if my_string != ""     # correct
if my_string           # transpiler error
```

```
Squarefeet area = 9
if area
    int val = (avow area)
if not area
    print("no value")
```

```rust
if area.is_some() {
    let val: i32 = area.unwrap().0;
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

### Forced Unwrap — `avow`

`(avow val)` asserts the value is `Some` and extracts the inner primitive. Panics at runtime if `None`. Use only when you are certain the value is present — typically inside an `if val` block where presence is already confirmed. Using `avow` on a non-validator-type variable is a transpiler error.

```
Roll roll = roll_die(d20)
if roll
    int val = (avow roll)    # always safe inside if roll
```

```rust
if roll.is_some() {
    let val: i32 = roll.unwrap().0;
}
```

Outside an `if` check, `avow` is the programmer's explicit assertion that the value is Some:

```
int sum = (avow value) + 2
```

```rust
let sum: i32 = value.unwrap().0 + 2;
```

---

### Safe Default — `else`

`value else default` returns the inner primitive if `Some`, or the default if `None`. Always safe — no panic risk.

```
int val = area else 0
```

```rust
let val: i32 = area.map(|v| v.0).unwrap_or(0);
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
    int cap = (avow max_capacity)
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
- `(avow val)` → `.unwrap().0`; `value else default` → `.map(|v| v.0).unwrap_or(default)`.
- Equality (`is` / `is not`) transpiles to `==` / `!=` in Rust and falls through to `Option<T>: PartialEq` — `None == None` is true, `Some(x) == Some(y)` compares inner values structurally.
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
