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

Integer literals may contain underscores as visual separators:
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

See [Rust Interop](interop.md) for full documentation, rules, and the build-once pattern.

---

## Validator Types (`type`)

A `type` definition wraps a base primitive with a predicate. **The predicate body is mandatory** — the transpiler errors on a `type` with an empty body. A validator type without a constraint adds no meaning over its base primitive; use the base type directly instead.

The base type must be a primitive (`int`, `float`, `string`, `bool`) or a struct — list shapes are not valid as a validator base type and are a transpiler error:

```
type Foo(int val)       # correct — primitive base type
type Foo(intList val)   # transpiler error — list shapes cannot be validator base types
```

The parameter name cannot shadow the type name or its own base type — both are transpiler errors:

```
type Roll(int Roll)    # transpiler error — parameter name shadows the type name
type Roll(int int)     # transpiler error — parameter name shadows its base type
type Roll(int val)     # correct
```

The body evaluates to a `bool`. Simple predicates are a single boolean expression; predicates that need intermediate values may declare bindings before the final bool expression, following the same rules as a function body.

A validator type is always `Option<T>` under the hood — assignment runs the predicate at runtime; if it passes the value is `Some`, if it fails the value is `None`. Primitives and structs are never null — only validator types carry presence/absence.

**Only the full declaration form runs the predicate.** `TypeName varName = expr` — naming the type explicitly — is the *only* statement shape that constructs through the validator and produces a checked `Option<TypeName>`. A later bare reassignment (`varName = expr`) or `as` binding (`varName as expr`) to that same name does **not** re-run the predicate — `=` will type-mismatch (it's assigning the raw base type into a slot typed `Option<TypeName>`), and `as` will silently shadow the validator-typed variable with a fresh, unrelated binding inferred from the new expression. The transpiler catches both forms and reports an error pointing at the full declaration form instead. If you need to re-validate a new value against the same validator type — for example, retrying user input in a loop until it passes — declare it fresh each time:

```
for if true
    (first) in input()
    Roll attempt = c_string_to_int(first)   # fresh declaration each iteration
    if attempt is valid
        return (avow attempt)
    else
        print("invalid, try again")
```

Shadowing a validator-typed name across loop iterations like this is the normal, expected pattern — it's not a one-time declaration you update, it's a fresh "construct and check" every time you need one.

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



### `is valid` / `is not valid`

A validator type variable is either **valid** (`Some` under the hood — predicate passed) or **not valid** (`None` under the hood — predicate failed or no value assigned). There is no keyword to force an invalid state. A variable becomes not valid in exactly two ways:

- Declared without a value: `Squarefeet sqft` — not valid until assigned
- Assigned a value that fails the predicate: `Squarefeet sqft = -10` — predicate fails, not valid

Check with `is valid` / `is not valid`:

```
Squarefeet sqft = 9
if sqft is valid
    int val = (avow sqft)
if sqft is not valid
    print("no value")
```

This is Deor's only concept of null. Every validator type defines exactly what makes a value invalid — a `customer_id` below 1, a `Squarefeet` that isn't a perfect square. Almost all types conceptually different from their base primitive have a natural constraint; the predicate makes that constraint explicit and enforced.


### Truthiness

**Only `bool` and validator types have a presence check.** Plain `int`, `float`, `string`, `list`, and structs are never truthy or falsy on their own — use explicit comparisons:

```
if len(my_list) > 0    # correct — explicit non-empty check
if my_list             # transpiler error — list has no truthiness

if my_int is not 0     # correct
if my_int              # transpiler error

if my_string is not "" # correct
if my_string           # transpiler error
```

Validator types use `is valid` / `is not valid` — not bare truthiness:

```
if sqft is valid        # correct
if sqft is not valid    # correct
if sqft                 # transpiler error — use is valid/is not valid
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

`empty` is not valid for validator types — using it is a transpiler error. There is no "empty" state for a validator type; not valid is expressed by declaring without a value:

```
Roll best = empty    # transpiler error — empty is not valid for validator types
Roll best            # correct — starts as not valid
```

List shapes use `empty` to initialize — `[]` is a transpiler error:

```
roomList rooms = empty
```

```rust
let mut rooms: Vec<Room> = Vec::new();
```

---

### Forced Unwrap — `avow`

`avow val` (or `(avow val)`) is Deor's equivalent of Rust's `.unwrap()` — it asserts the value is `Some` and extracts the underlying primitive. Panics at runtime if not valid. Use only when you are certain the value is valid — typically inside an `if val is valid` block. Using `avow` on a non-validator-type variable is a transpiler error.

Parentheses are optional, not required. `avow` binds only to the next primary — a bare identifier, literal, or parenthesized group — the same rule `move` already follows. So `avow val + 2` parses unambiguously as `(avow val) + 2`, never as `avow (val + 2)`; there's no scenario where omitting the parens changes what `avow` applies to. Use the parens when they help a reader scan a larger expression (`(avow value) + 2` reads a little more clearly than `avow value + 2`), but they're a style choice, not a syntax requirement.

`avow` gives you the raw primitive beneath the validator type — `int` from a `Roll`, `float` from a `Squarefeet`. When you need to pass a validator type value to a function that accepts that validator type, pass the variable directly — no `avow` needed. Only reach for `avow` when you specifically need the underlying primitive.

`avow` can be used directly as a function argument (`show(avow roll)`) — it does not need to be captured into a variable first.

```
Roll roll = roll_die(d20)
if roll is valid
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

Struct fields typed as a validator type are `Option<T>` under the hood. Extracting them with `in` preserves the Option — the extracted variable must be checked with `is valid` / `is not valid` before use.

```
struct Room
    Squarefeet area
    Roll max_capacity
```

```
(area, max_capacity) in room
if max_capacity is valid
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

The caller checks with `is valid`:

```
Roll crit = find_crit(rolls)
if crit is valid
    int bonus = (avow crit)
```

---

**Conversion notes:**
- Constructor becomes `fn new(n: T) -> Option<Self>` — never panics, returns `None` on predicate failure.
- `is valid` → `.is_some()`, `is not valid` → `.is_none()`.
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
