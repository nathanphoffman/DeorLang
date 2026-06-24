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

Structs can be constructed with an explicit type (`Type name = (fields)`) or inferred via `as` (`name as (fields)`). The transpiler matches field names to determine the struct type. Every field must already be a variable in scope matching the field name exactly. No `{}`, no `field: value` pairs.

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

fn DivResult divmod(int lft, int rgt)
    int quotient = lft / rgt
    int remainder = lft % rgt
    return (quotient, remainder)
```

```rust
fn divmod(lft: i32, rgt: i32) -> DivResult {
    let quotient = lft / rgt;
    let remainder = lft % rgt;
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

`(fields)` as a bare expression is also valid in return position — the function's declared return type resolves the struct.

If you need a field name that differs from the variable you have, rename it first:

```
name in other_room
label = name
Entry entry = (label)
```

---

## Validator Type Bindings

Declaring a variable with a validator type runs the predicate at assignment. The variable is `Option<T>` under the hood — valid (`Some`) if the predicate passes, not valid (`None`) if it fails.

```
Squarefeet area = 9            # valid — predicate passes
Squarefeet area = -1           # transpiler error — literal fails predicate at compile time
Roll roll = random(min, max)   # valid or not valid depending on the predicate
```

```rust
let area: Option<Squarefeet> = Squarefeet::new(9);
let area: Option<Squarefeet> = Squarefeet::new(-1);
let roll: Option<Roll> = Roll::new(random(min, max));
```

### Declaring Without a Value

A validator type variable can be declared without an initial value to start as not valid. It becomes valid once assigned a value that passes the predicate.

```
Roll best
```

```rust
let mut best: Option<Roll> = None;
```

List shapes use `empty` to initialize — `[]` is a transpiler error:

```
roomList rooms = empty
```

```rust
let mut rooms: Vec<Room> = Vec::new();
```

### Reassignment

Reassigning a validator type re-runs the predicate. The variable may transition between valid and not valid.

```
Squarefeet area = 9   # valid
area = 16             # valid
int raw = get_user_input()
area = raw            # valid or not valid — predicate runs at runtime
```

---

## `const` — Immutable Typed Bindings

`const` declares a typed binding that is explicitly immutable. The transpiler will never emit `let mut` for a `const` variable, even if the surrounding code would otherwise infer mutability.

```
const string pipe = "|"
const int max_retries = 3
```

```rust
let pipe: String = "|".to_string();
let max_retries: i32 = 3;
```

**`const` vs plain typed binding:** a plain `string pipe = "|"` is also immutable if never reassigned, but `const` makes the intent explicit and guarantees it at the transpiler level. Use `const` for values that should never change.

**`const` vs `as`:** both produce immutable bindings. `const` requires an explicit type; `as` infers the type from the literal. Use `const` when the type must be stated, `as` for simple literals where inference is unambiguous.

```
const string label = "hello"   # explicit type, immutable
label as "hello"               # inferred type, immutable — equivalent here
int count as 0                 # transpiler error — as never takes a type prefix
```

---

## Explicit Typing — Runtime Values

Any value from a function call or other runtime computation uses `Type name = expr`. For list types the type is the shape name.

```
int val = m_rand_int(min, max)
string pick = random_room_name(rooms)
roomList result = empty
```

```rust
let val: i32 = m_rand_int(min, max);
let pick: String = random_room_name(&rooms);
let mut result: Vec<i32> = Vec::new();
```

**Conversion notes:** a list binding that's later appended to must be emitted as `let mut` — the transpiler infers `mut` from usage.

---

## Reassignment

```
total = total + 1
```

```rust
total = total + 1;
```

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

