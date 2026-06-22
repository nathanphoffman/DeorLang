# Variables

## `as` — Type-Inferred Bindings
`as` creates a binding whose type is derived from the right-hand side at compile time. It has two valid forms:

### Scalar literals
The type is inferred from the literal value.

```
sum as 0
label as "Office"
flag as true
rate as 3.14
```

```rust
let sum = 0;
let label = "Office".to_string();
let flag = true;
let rate = 3.14_f64;
```

### List construction

A list literal `[item1, item2, ...]` constructs a list. All items must be named variables of the same type already in scope.

```
rooms as [kitchen, office, bedroom]    # type inferred from items (all Room)
```

```rust
let rooms = vec![kitchen.clone(), office.clone(), bedroom.clone()];
```

`[]` is not valid for initializing an empty list — use `empty` with an explicit shape type. `as []` is also a transpiler error because the element type is unknown:

```
roomList result = empty  # correct
result as []             # transpiler error — element type unknown
roomList result = []     # transpiler error — use empty instead
```

---

**What `as` is not for:**

**Struct construction** — `(fields)` alone does not identify which struct is being built. Use explicit `Type name = (fields)` instead — see [Struct Construction](#struct-construction) below.

```
room as (area, name)      # transpiler error — which struct?
Room room = (area, name)  # correct
```

**Validator type bindings** — `as` has no way to know whether you want a plain `int` (no predicate, stored directly) or a `Squarefeet` validator type (predicate runs, result is `Option<T>`). Without an explicit type the transpiler cannot decide which behaviour you intend, so it errors. Use explicit `ValidatorType name = value` — see [Validator Type Bindings](#validator-type-bindings) below.

```
area as 9             # transpiler error — int or Squarefeet?
Squarefeet area = 9   # correct
```

**Type annotation** — `as` never takes an explicit type prefix:

```
int count as 0      # transpiler error
int count = 0       # correct
```

**Variable copying** — `as` requires a literal or `[items]` on the right:

```
copy as original    # transpiler error — use Room copy = original
```

Record update (`with`) uses `as` — the type is known from the source struct. See [Immutability](immutability.md).

---

## Struct Construction

The primary form of struct construction is `Type name = (fields)`. The type name is mandatory in assignment position — there is no `as (fields)` anonymous struct construction. Every field must already be a variable in scope matching the field name exactly. No `{}`, no `field: value` pairs.

```
Squarefeet area = 9
name as "Office"
occupied as true
Room room = (area, name, occupied)
```

```rust
let area = Squarefeet::new(9);
let name = "Office".to_string();
let occupied = true;
let room = Room { area, name, occupied };
```

The transpiler matches fields by name — order does not matter. Mirrors destructuring: `in` pulls fields out of a struct, `= (fields)` pushes variables in.

### Struct Construction as an Expression

`(fields)` can also appear as a bare expression in return position. The transpiler resolves the struct type by matching the field names against all known structs — no type annotation is needed because the function's return type already determines which struct is expected.

```
struct DivResult
    int quotient
    int remainder

fn DivResult divmod(int a, int b)
    int quotient = a / b
    int remainder = a % b
    return (quotient, remainder)
```

```rust
fn divmod(a: i32, b: i32) -> DivResult {
    let quotient = a / b;
    let remainder = a % b;
    return DivResult { quotient, remainder };
}
```

This is equivalent to assigning to an intermediate variable and returning it — just more concise:

```
fn DivResult divmod(int a, int b)
    int quotient = a / b
    int remainder = a % b
    DivResult result = (quotient, remainder)   # equivalent
    return result
```

`(fields)` as a bare expression is only valid in return position. Assigning with `as` still requires an explicit type:

```
room as (area, name)      # transpiler error — which struct?
Room room = (area, name)  # correct
```

If you need a field name that differs from the variable you have, rename it first:

```
name in other_room
label = name
Entry entry = (label)
```

---

## Validator Type Bindings

Declaring a variable with a validator type runs the predicate at assignment. The variable is `Option<T>` under the hood — truthy (`Some`) if the predicate passes, falsy (`None`) if it fails.

```
Squarefeet area = 9            # Some(Squarefeet(9)) — predicate passes
Squarefeet bad = -1            # transpiler error — literal fails predicate at compile time
Roll roll = random(min, max)   # Some or None depending on the value
```

```rust
let area: Option<Squarefeet> = Squarefeet::new(9);
let bad: Option<Squarefeet> = Squarefeet::new(-1);
let roll: Option<Roll> = Roll::new(random(min, max));
```

### Initializing to Empty

A validator type variable can be declared as `empty` to start explicitly absent. `empty` is also the only valid way to initialize an empty list shape — `[]` is a transpiler error. See [Enforced Practices — empty at Declaration Only](enforced_practices.md#empty-at-declaration-only) for the assignment restriction.

```
Roll best = empty
```

```rust
let mut best: Option<Roll> = None;
```

List shapes:

```
roomList rooms = empty
```

```rust
let mut rooms: Vec<Room> = Vec::new();
```

### Reassignment

Reassigning a validator type re-runs the predicate. The variable may transition between `Some` and `None`.

```
Squarefeet area = 9   # Some(Squarefeet(9))
area = 16             # Some(Squarefeet(16))
int raw = get_user_input()
area = raw            # Some or None — predicate runs at runtime
```

---

## Explicit Typing — Runtime Values

Any value from a function call or other runtime computation uses `Type name = expr`. For list types the type is the shape name.

```
int val = random(min, max)
string pick = random_room_name(rooms)
roomList result = empty
```

```rust
let val: i32 = random(min, max);
let pick: String = random_room_name(&rooms);
let mut result: Vec<i32> = Vec::new();
```

**Conversion notes:** a list binding that's later `insert`ed into must be emitted as `let mut` — the transpiler infers `mut` from usage.

---

## Reassignment

```
total = total + 1
```

```rust
total = total + 1;
```

---

## Constants

`const` declares an immutable typed binding inside a function. The transpiler prevents reassignment — any attempt to write to a `const` name will fail to compile in Rust.

```
const int max_retries = 3
const float threshold = 0.75
const bool verbose = false
const string label = "score"
```

```rust
let max_retries: i32 = 3;
let threshold: f64 = 0.75;
let verbose: bool = false;
let label: String = "score".to_string();
```

- Function scope only — `const` at top level is a transpiler error
- All four primitive types are valid: `int`, `float`, `bool`, `string`

---

## Numeric Literals

Underscores may appear anywhere in a numeric literal as a visual separator. They are stripped by the transpiler and have no effect on the value.

```
int population = 1_000_000
float rate = 0.000_001
int port = 8_080
```

```rust
let population: i32 = 1_000_000;
let rate: f64 = 0.000_001;
let port: i32 = 8_080;
```

Underscore placement is free-form — `1_000_000`, `10_00_00`, and `1000000` are all the same value.

Hex literals (`0xFF`) and binary literals (`0b1010`) are deferred to v2. Use a `rust` block for code that requires them.

---

