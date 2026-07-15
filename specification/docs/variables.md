<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: blackboard -->
# Variables

## `as` — Type-Inferred Bindings
`as` creates a binding whose type is derived from the right-hand side at compile time. It has four valid forms:

### Scalar literals
The type is inferred from the literal value.

Deor:
```deor
sum as 0
label as "Office"
flag as true
rate as 3.14
```

Rust:
```rust
let sum = 0;
let label = "Office".to_string();
let flag = true;
let rate = 3.14_f64;
```

### List construction

A list literal `[item1, item2, ...]` constructs a list. All items must be named variables of the same type already in scope.

Deor:
```deor
rooms as [kitchen, office, bedroom]    # type inferred from items (all Room)
```

Rust:
```rust
let rooms = vec![kitchen.clone(), office.clone(), bedroom.clone()];
```

`[]` is never valid for an empty list — `as` can't infer an element type from nothing, so use `empty` with an explicit shape type instead. See [Collections — Empty List](docs/collections.md#empty-list).

```deor
result as []              # transpiler error — element type unknown
```

### Struct construction

A parenthesised field list `(f1, f2, ...)` constructs a struct. See [Structs — Struct Construction](docs/structs.md#struct-construction) for the full rules and the equivalent explicit-type form.

Deor:
```deor
score as (label, points)    # struct type inferred from field names
```

Rust:
```rust
let score = Score { label: label.clone(), points: points.clone() };
```

### Existing variable

A bare identifier on the right binds directly to an existing variable. Like every other `as` form, this clones — the source stays usable afterward.

Deor
```deor
saved_lines as lines
print(lines)   # still valid
```

Rust:
```rust
let saved_lines = lines.clone();
```

---

**What `as` is not for:**

- **Validator type bindings** — `as` can't tell whether you want a plain `int` or a `Squarefeet` validator (predicate run, `Option<T>` result); use explicit `ValidatorType name = value` instead — see [Validator Type Bindings](#validator-type-bindings) below.
- **Type annotation** — `as` never takes an explicit type prefix.
- **Move transfer** — `as` always clones, so there's nothing for `move` to opt out of; combining them is rejected. Use a typed `=` declaration if you need `move`.

```deor
area as 9             # transpiler error — int or Squarefeet? use Squarefeet area = 9
int count as 0        # transpiler error — annotation not allowed with as; use int count = 0
a as move b           # transpiler error — as always clones, move has nothing to do
```

Record update (`with`) uses `as` — the type is known from the source struct. See [Structs — Record Update](docs/structs.md#record-update-with).

Struct construction and destructuring are documented in full on their own page — see [Structs](docs/structs.md).

---

## Validator Type Bindings

Declaring a variable with a validator type runs the predicate at assignment. The variable is `Option<T>` under the hood — valid (`Some`) if the predicate passes, not valid (`None`) if it fails.

Deor:
```deor
Squarefeet area = 9            # valid — predicate passes
Squarefeet area = -1           # transpiler error — literal fails predicate at compile time
Roll roll = random(min, max)   # valid or not valid depending on the predicate
```

Rust:
```rust
let area: Option<Squarefeet> = Squarefeet::new(9);
let area: Option<Squarefeet> = Squarefeet::new(-1);
let roll: Option<Roll> = Roll::new(random(min, max));
```

### Declaring Without a Value

A validator type variable can be declared without an initial value to start as not valid. It becomes valid once assigned a value that passes the predicate.

Deor:
```deor
Roll best
```

Rust:
```rust
let mut best: Option<Roll> = None;
```

(List shapes use `empty` the same way — see [List Construction](#list-construction) above.)

This is how Deor represents an absent value without a `null`/`undefined` keyword — see [Validator Types — Replacing Null and Undefined](docs/validator_types.md#replacing-null-and-undefined).

### Reassignment

Reassigning a validator type re-runs the predicate. The variable may transition between valid and not valid.

```deor
Squarefeet area = 9   # valid
area = 16             # valid
int raw = get_user_input()
area = raw            # valid or not valid — predicate runs at runtime
```

---

## `const` — Immutable Typed Bindings

`const` declares a typed binding that is explicitly immutable. The transpiler will never emit `let mut` for a `const` 
variable, even if the surrounding code would otherwise infer mutability.

`const` names must be `SCREAMING_SNAKE_CASE` — all caps, words separated by underscores. This distinguishes constants from regular 
variables at a glance and signals that the value is fixed for the lifetime of the scope.

Deor:
```deor
const string PIPE = "|"
const int MAX_RETRIES = 3
```

Rust:
```rust
let PIPE: String = "|".to_string();
let MAX_RETRIES: i64 = 3;
```

**`const` vs plain typed binding:** a plain `string pipe = "|"` is also immutable if never reassigned, but `const` 
makes the intent explicit and guarantees it at the transpiler level. Use `const` for values that should never change.

**`const` vs `as`:** both produce immutable bindings. `const` requires an explicit type; `as` infers the type from the 
literal. Use `const` when the type must be stated, `as` for simple literals where inference is unambiguous.

```deor
const string LABEL = "hello"   # explicit type, immutable, SCREAMING_SNAKE required
label as "hello"               # inferred type, immutable — snake_case name
int count as 0                 # transpiler error — as never takes a type prefix
```

---

## Explicit Typing — Runtime Values

Any value from a function call or other runtime computation uses `Type name = expr`. For list types the type is the shape name.

Deor:
```deor
int val = m_rand_int(min, max)
string pick = random_room_name(rooms)
roomList result = empty
```

Rust:
```rust
let val: i64 = m_rand_int(min, max);
let pick: String = random_room_name(&rooms);
let mut result: Vec<i64> = Vec::new();
```

**Conversion notes:** a list binding that's later appended to must be emitted as `let mut` — the transpiler infers `mut` from usage.

---

## Reassignment

Deor:
```deor
total = total + 1
```

Rust:
```rust
total = total + 1;
```

---

## Numeric Literals

Underscores may appear anywhere in a numeric literal as a visual separator. They are stripped by the transpiler and have no effect on the value.

Deor:
```deor
int population = 1_000_000
float rate = 0.000_001
int port = 8_080
```

Rust:
```rust
let population: i64 = 1_000_000;
let rate: f64 = 0.000_001;
let port: i64 = 8_080;
```

Underscore placement is free-form — `1_000_000`, `10_00_00`, and `1000000` are all the same value.

Hex literals (`0xFF`) and binary literals (`0b1010`) are deferred to v2. Use a `rust` block for code that requires them.

---