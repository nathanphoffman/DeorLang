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

For integers seperators are allowed such as
```
int val = 1_000_000
```
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

A `type` definition wraps a base primitive with a predicate. **The predicate body is mandatory** — the transpiler errors on a `type` with an empty body. A validator type without a constraint adds no meaning over its base primitive; use the base type directly instead.

The body evaluates to a `bool`. Simple predicates are a single boolean expression; predicates that need intermediate values may declare bindings before the final bool expression, following the same rules as a function body.

A validator type is always `Option<T>` under the hood — assignment runs the predicate at runtime; if it passes the value is `Some`, if it fails the value is `None`. Primitives and structs are never null — only validator types carry presence/absence.

```
# import lib/math.deor and lib/convert.deor for these functions
type Squarefeet(int val)
    float flt = c_int_to_float(val)
    float root = m_sqrt(flt)
    int root_i = m_floor(root)
    return root_i * root_i is val
```

`c_int_to_float`, `m_sqrt`, and `m_floor` are from `lib/convert.deor` and `lib/math.deor` — see [Libs](libs.md). Each intermediate result is stored before being passed to the next call. A negative `val` makes `m_sqrt` return NaN; `m_floor(NaN)` gives `0`; `0 * 0 is val` fails — no separate guard needed.

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
Squarefeet area = 9     # valid — predicate passes
Squarefeet area = -1    # transpiler error — literal value fails predicate at compile time
```

```rust
let area: Option<Squarefeet> = Squarefeet::new(9);
let area: Option<Squarefeet> = Squarefeet::new(-1);
```

---



### Validator Types / valid / not valid

A validator type variable is either **valid** (`Some` under the hood — predicate passed) or **not valid** (`None` under the hood — predicate failed or no value assigned). There is no keyword to force an invalid state. A variable becomes not valid in exactly two ways:

- Declared without a value: `Squarefeet sqft` — not valid until assigned
- Assigned a value that fails the predicate: `Squarefeet sqft = -10` — predicate fails, not valid

Check with `valid` / `not valid`:

```
Squarefeet sqft = 9
if sqft valid
    int val = (avow sqft)
if sqft not valid
    print("no value")
```

This is Deor's only concept of null. Every validator type defines exactly what makes a value invalid — a `customer_id` below 1, a `Squarefeet` that isn't a perfect square. Almost all types conceptually different from their base primitive have a natural constraint; the predicate makes that constraint explicit and enforced.


### Truthiness

**Only `bool` and validator types have a presence check.** Plain `int`, `float`, `string`, `list`, and structs are never truthy or falsy on their own — use explicit comparisons:

```
if len(my_list) > 0    # correct — explicit non-empty check
if my_list             # transpiler error — list has no truthiness

if my_int != 0         # correct
if my_int              # transpiler error

if my_string != ""     # correct
if my_string           # transpiler error
```

Validator types use `valid` / `not valid` — not bare truthiness:

```
if sqft valid           # correct
if sqft not valid       # correct
if sqft                 # transpiler error — use valid/not valid
```

```rust
if area.is_some() {
    let val: i32 = area.unwrap().0;
}
if area.is_none() {
    // not valid
}
```

---

### Declaring Without a Value

A validator type variable declared without an initial value starts as not valid. Assign a value later to make it valid.

```
Roll best
Squarefeet area
```

```rust
let mut best: Option<Roll> = None;
let mut area: Option<Squarefeet> = None;
```

List shapes still use `empty` to initialize — `[]` is a transpiler error:

```
roomList rooms = empty
```

```rust
let mut rooms: Vec<Room> = Vec::new();
```

---

### Forced Unwrap — `avow`

`(avow val)` is Deor's equivalent of Rust's `.unwrap()` — it asserts the value is `Some` and extracts the underlying primitive. Panics at runtime if not valid. Use only when you are certain the value is valid — typically inside an `if val valid` block. Using `avow` on a non-validator-type variable is a transpiler error.

The parentheses are always required — this is intentional. Without them, `avow val + 2` would be ambiguous: does `avow` bind to `val` or to `val + 2`? The parens make the boundary explicit, which matters most when `avow` is used inline inside a larger expression like `(avow value) + 2`. Writing `avow val` on its own line would be unambiguous, but the rule is uniform: parens always.

`avow` gives you the raw primitive beneath the validator type — `int` from a `Roll`, `float` from a `Squarefeet`. When you need to pass a validator type value to a function that accepts that validator type, pass the variable directly — no `avow` needed. Only reach for `avow` when you specifically need the underlying primitive.

**Note:** Avow must be captured by a variable declaration you can not try passing it around directly as a function argument, this is intentional, and matches the rust output / compiler limitations.

```
Roll roll = roll_die(d20)
if roll valid
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
### Validator Types in Structs

Struct fields typed as a validator type are `Option<T>` under the hood. Extracting them with `in` preserves the Option — the extracted variable must be checked with `valid` / `not valid` before use.

```
struct Room
    Squarefeet area
    Roll max_capacity
```

```
(area, max_capacity) in room
if max_capacity valid
    int cap = (avow max_capacity)
```

```rust
let (area, max_capacity) = (room.area, room.max_capacity);
if max_capacity.is_some() {
    let cap: i32 = max_capacity.unwrap().0;
}
```

---
### Functions Returning Validator Types

A function returning a validator type returns a variable that may or may not be valid. To return a not-valid result, either declare the variable without a value and return it unassigned, or assign a value that fails the predicate.

`return empty` and `return none` are both transpiler errors — neither is a Deor keyword in return position.

```
shape rollResultList = list of RollResult

fn Roll find_crit(rollResultList rolls)
    Roll found

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

The caller checks with `valid`:

```
Roll crit = find_crit(rolls)
if crit valid
    int bonus = (avow crit)
```

---

**Conversion notes:**
- Constructor becomes `fn new(n: T) -> Option<Self>` — never panics, returns `None` on predicate failure.
- `valid` → `.is_some()`, `not valid` → `.is_none()`.
- `(avow val)` → `.unwrap().0`.
- Equality (`is` / `is not`) transpiles to `==` / `!=` in Rust and falls through to `Option<T>: PartialEq` — `None == None` is true, `Some(x) == Some(y)` compares inner values structurally.
- `and` / `or` / `not` map to `&&` / `||` / `!`.
- Literal predicate failures (`Squarefeet area = -1`) are caught at transpile time.

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
