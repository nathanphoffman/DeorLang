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
    return root_i * root_i is val
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



### Validator Types / Bad / "Null"
A validator type is assigned ```bad``` when its response for its boolean return is falsy. You can do comparisons on whether the value is bad or not, and then avow it if you know it is good to extract the value (avowing a bad result will cause a ```crash``` -- a panic! equivalent in rust)

```
Squarefeet area = 9
if area is not bad
    int val = (avow area)
if area is bad
    print("no value")
```

A validator type is considered bad if it is invalid, but it is the only concept in the language for null.  This was a bit of Deors human-language philosophy. A number being null makes no sense in the real world as it explains no reason as to why it is null, how could a pure mathematical number ever not be a value? However, a customer_id, that makes more sense, as the context tells us that a customer_id could be bad.

This means that for these values you must define a validator type to explain what would make that id bad, for a customer_id it is probably anything below 1.  In almost all cases something that is non-primitive will have a constraint.

If for some reason a validator type never has a constraint of any kind you could simply return ```true``` from the validator type and assign bad like a null value, however this is considered bad practice as almost all types conceptually different than primitives should be narrowerer than their base types and thus definable as where they are "bad"
```
type BadableInt
    true

# bad practice, but allowed for the odd edge cases where you would need this
BadableInt num = bad

```


### Truthy / Falsy



**Only `bool` has truthiness.** Plain `int`, `float`, `string`, `list`, and structs are never truthy or falsy on their own — they have no presence/absence concept. Use explicit comparisons instead:

```
if len(my_list) > 0    # correct — explicit non-empty check
if my_list             # transpiler error — list has no truthiness

if my_int != 0         # correct
if my_int              # transpiler error

if my_string != ""     # correct
if my_string           # transpiler error
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
if roll is not bad
    int val = (avow roll)          # need the raw int — use avow
    bool crit = is_critical(roll)  # function takes Roll — pass directly, no avow
```

```rust
if roll != None {
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

Struct fields typed as a validator type are `Option<T>` under the hood. Extracting them with `in` preserves the Option — the extracted variable is still truthy/falsy and must be checked before use.

```
struct Room
    Squarefeet area
    Roll max_capacity
```

```
(area, max_capacity) in room
if max_capacity is not bad
    int cap = (avow max_capacity)
```

```rust
let (area, max_capacity) = (room.area, room.max_capacity);
if max_capacity != None {
    let cap: i32 = max_capacity.unwrap().0;
}
```

---
### Functions Returning Validator Types
A function whose return type is a validator type can return either a named typed variable or `bad`. Use `bad` when the function genuinely cannot produce a valid value — for example, a function that wraps a negative number into a `Positive` type should return `bad` when given `-1`.

`return empty` is a transpiler error — `empty` represents a temporary uninitialized list state and has no meaning as a return value. `return none` is also a transpiler error — `none` is not a Deor keyword.

```
shape rollResultList = list of RollResult

fn Roll find_crit(rollResultList rolls)
    Roll found = bad

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

The caller uses `if` to handle the result:

```
Roll crit = find_crit(rolls)
if crit is not bad
    int bonus = (avow crit)
```

---

**Conversion notes:**
- Constructor becomes `fn new(n: T) -> Option<Self>` — never panics, returns `None` on predicate failure.
- Truthy/falsy maps to `.is_some()` / `.is_none()`.
- `(avow val)` → `.unwrap().0`.
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

Struct fields may be primitives, validator types, list shapes, or other structs. Func shapes as struct fields are a transpiler error — structs are pure data.

There are no per-field visibility modifiers — all fields are always accessible via destructuring whenever the struct itself is in scope.
