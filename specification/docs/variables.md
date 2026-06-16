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

An empty list `[]` cannot be typed by inference — there are no items to infer from:

```
roomList result = []    # correct — shape name gives the type
result as []            # transpiler error — element type unknown
```

---

**What `as` is not for:**

**Struct construction** — `(fields)` alone does not identify which struct is being built. Use explicit `Type name = (fields)` instead — see [Struct Construction](#struct-construction) below.

```
room as (area, name)      # transpiler error — which struct?
Room room = (area, name)  # correct
```

**Validator type bindings** — a literal alone doesn't say whether a validator should run. Use explicit `ValidatorType name = value` — see [Validator Type Bindings](#validator-type-bindings) below.

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

Struct construction always uses `Type name = (fields)`. The type name is mandatory — there is no anonymous struct construction in Deor. Every field must already be a variable in scope matching the field name exactly. No `{}`, no `field: value` pairs.

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

The transpiler matches fields by name, not position — declaration order is not enforced. Mirrors destructuring: `in` pulls fields out of a struct, `= (fields)` pushes variables in. Both work with any subset and any order.

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
Squarefeet bad = -1            # None — predicate fails
Roll roll = random(min, max)   # Some or None depending on the value
```

```rust
let area: Option<Squarefeet> = Squarefeet::new(9);
let bad: Option<Squarefeet> = Squarefeet::new(-1);
let roll: Option<Roll> = Roll::new(random(min, max));
```

### Initializing to Empty

A validator type variable can be declared as `empty` to start explicitly absent. `empty` is only valid at the point of first declaration — assigning `empty` after a variable has been declared is a transpiler error.

```
Roll best = empty
```

```rust
let mut best: Option<Roll> = None;
```

`empty` also works for list shapes, as an alternative to `[]`:

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
area = -1             # None — predicate fails
```

---

## Explicit Typing — Runtime Values

Any value from a function call or other runtime computation uses `Type name = expr`. For list types the type is the shape name.

```
int val = random(min, max)
string pick = random_room_name(rooms)
roomList result = []
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
total = total + 1    # explicit form
total += 1           # compound assignment — equivalent
```

```rust
total += 1;
```

Both forms are valid. `+=`, `-=`, `*=`, `/=`, and `%=` are all supported. See [Operators](operators.md#compound-assignment).

---

## Constants

Constants are immutable primitive bindings. They use `const` as a prefix, must be named in SCREAMING_SNAKE_CASE, and are always explicitly typed. Only primitive types can be `const`.

```
const int DELAY_TIME = 500
const string APP_NAME = "Deor"
const bool DEBUG = false
const float PHI = 1.61803
```

```rust
const DELAY_TIME: i32 = 500;
const APP_NAME: &str = "Deor";
const DEBUG: bool = false;
const PI: f64 = 3.14159;
```

- Valid at any scope — top-level or inside a function
- Reassignment is a compile-time error
- `list`, structs, and validator types cannot be `const` (structs are already immutable; `list` constants are not yet specified)

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

