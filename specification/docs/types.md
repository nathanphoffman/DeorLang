# Types

## Primitive Types

Deor's built-in primitive types and their Rust equivalents:

| Deor | Rust | Notes |
|---|---|---|
| `int` | `i32` | General-purpose integer |
| `float` | `f64` | General-purpose decimal |
| `bool` | `bool` | |
| `string` | `String` | Owned; available as `&str` via `.as_str()` in `rust` blocks |

For raw binary data (HTTP bodies, files, crypto, pixel buffers) use a `raw` variable and handle it entirely inside `rust` blocks. See [`raw` Variables](#raw-variables) below.

---

## `raw` Variables

`raw` is a special variable kind for holding an opaque Rust value — produced by a `rust` block and consumed only inside `rust` blocks. It has no type annotation, no Deor operators, and cannot appear in Deor expressions or struct fields.

```
raw index = rust
    entries.iter()
        .map(|e| (e.key.clone(), e.value.clone()))
        .collect::<std::collections::HashMap<String, String>>()
```

See [Rust Interop — The `raw` Type](interop.md#the-raw-type) for full documentation, rules, and the build-once pattern.

---

## Validator Types (`type`)

A Validator type uses bad over none as of June 18

A `type` definition wraps a base primitive with a predicate. **The predicate body is mandatory** — the transpiler errors on a `type` with an empty body. A validator type without a constraint adds no meaning over its base primitive; use the base type directly instead.

The body evaluates to a `bool`. Simple predicates are a single boolean expression; predicates that need intermediate values may declare bindings before the final bool expression, following the same rules as a function body.

A validator type is always `Option<T>` under the hood — assignment runs the predicate at runtime; if it passes the value is `Some`, if it fails the value is `None`. Primitives and structs are never null — only validator types carry presence/absence.

```
# to_float, sqrt_f, floor_f are shims — see shims.md
type Squarefeet(int val)
    float flt = to_float(val)
    float root = sqrt_f(flt)
    int root_i = floor_f(root)
    root_i * root_i is val
```

`to_float`, `sqrt_f`, and `floor_f` are user-defined shims — copy them from [Shims — Math](shims.md#math). Each intermediate result is stored before being passed to the next call. A negative `val` makes `sqrt_f` return NaN; `floor_f(NaN)` gives `0`; `0 * 0 is val` fails — no separate guard needed.

```rust
#[derive(Clone, Copy, PartialEq, Debug)]
struct Squarefeet(i32);

impl Squarefeet {
    fn new(val: i32) -> Option<Self> {
        let flt: f64 = val as f64;
        let root: i32 = flt.sqrt().floor() as i32;
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
Squarefeet bad = -1     # transpiler error — literal value fails predicate at compile time
```

```rust
let area: Option<Squarefeet> = Squarefeet::new(9);
let bad: Option<Squarefeet> = Squarefeet::new(-1);
```

---

### Truthy / Falsy

A validator type variable is truthy when `Some`, falsy when `None`. Use `if` to check presence and `if X is not` to check absence before using the value.

**Only validator types and `bool` have truthiness.** Plain `int`, `float`, `string`, `list`, and structs are never truthy or falsy on their own — they have no presence/absence concept. Use explicit comparisons instead:

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
if area is not
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

### Initializing to Empty

A validator type variable can be explicitly initialized to `empty` to start absent. List shapes also use `empty` to start with no elements — `[]` is a transpiler error. See [Enforced Practices — empty at Declaration Only](enforced_practices.md#empty-at-declaration-only) for the assignment restriction.

```
Roll best = empty
Squarefeet area = empty
roomList rooms = empty
```

```rust
let mut best: Option<Roll> = None;
let mut area: Option<Squarefeet> = None;
let mut rooms: Vec<Room> = Vec::new();
```

---

### Forced Unwrap — `avow`

`(avow val)` is Deor's equivalent of Rust's `.unwrap()` — it asserts the value is `Some` and extracts the underlying primitive. Panics at runtime if `None`. Use only when you are certain the value is present — typically inside an `if val` block. Using `avow` on a non-validator-type variable is a transpiler error.

The parentheses are always required — this is intentional. Without them, `avow val + 2` would be ambiguous: does `avow` bind to `val` or to `val + 2`? The parens make the boundary explicit, which matters most when `avow` is used inline inside a larger expression like `(avow value) + 2`. Writing `avow val` on its own line would be unambiguous, but the rule is uniform: parens always.

`avow` gives you the raw primitive beneath the validator type — `int` from a `Roll`, `float` from a `Squarefeet`. When you need to pass a validator type value to a function that accepts that validator type, pass the variable directly — no `avow` needed. Only reach for `avow` when you specifically need the underlying primitive.

**Note:** Avow must be captured by a variable declaration you can not try passing it around directly as a function argument, this is intentional, and matches the rust output / compiler limitations.

```
Roll roll = roll_die(d20)
if roll
    int val = (avow roll)          # need the raw int — use avow
    bool crit = is_critical(roll)  # function takes Roll — pass directly, no avow
```

```rust
if roll.is_some() {
    let val: i32 = roll.unwrap().0;
    let crit: bool = is_critical(roll);
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

`else` here is null-coalescing — only valid on validator type variables. It extracts the inner primitive if `Some`, or returns the default if `None`. This is distinct from `if/else` block syntax and from the compact ternary `else` branch — see [Conditionals — The Three Uses of `else`](conditionals.md#the-three-uses-of-else).

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

A function whose return type is a validator type may return a `None` value through its return variable. `return empty` and `return none` are both transpiler errors — always return a named typed variable. The caller knows the return may be `None` because the return type is a validator type.

```
shape rollResultList = list of RollResult

fn Roll find_crit(rollResultList rolls)
    Roll found = empty

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
- Literal predicate failures (`Squarefeet bad = -1`) are caught at transpile time.

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

### Representation Sigils

| Form | Meaning | Rust representation |
|---|---|---|
| `struct Name` | Transpiler decides | `Name` (value) or `Rc<Name>` (reference), based on size + whether any field is an unsized list shape |

```
struct House
    string address
    roomList rooms         # unsized list shape -> auto becomes struct*

```

Struct fields may be primitives, validator types, list shapes, or other structs. Func shapes as struct fields are a transpiler error — structs are pure data.

**Visibility applies to top-level declarations, not to fields.** Structs, shapes, functions, and types are all public by default; marking one `private` prevents it from being imported by other files. There are no per-field visibility modifiers — all fields are always accessible via destructuring whenever the struct itself is in scope.

**Conversion notes:**
- The **struct definition itself is identical** regardless of `+`/`*`/auto — only how *usages* are represented changes (`House` vs `Rc<House>`).
- An **unsized `list` field** makes a struct's clone cost O(n) and unbounded, so it defaults to `*` (reference) unless overridden with `+`.
- `==` is always `#[derive(PartialEq)]` on the underlying struct. `Rc<T>`'s default `PartialEq` already delegates to `T`'s impl in Rust, so structural equality holds for `struct*` types **with no extra work**.
- A struct containing only primitives has a fully known size and is `Copy`-eligible if every field is `Copy`.
