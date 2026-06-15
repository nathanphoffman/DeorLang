# Variables

## `as` — Shape-Derived Bindings

`as` creates a binding whose type is derived from the shape of the right-hand side at compile time. It is used in three forms:

### Scalar literals

The type is inferred from the literal value. String, int, float, and bool literals each produce their respective types.

```
sum as 0
label as "Office"
flag as true
```

```rust
let sum = 0;
let label = "Office".to_string();
let flag = true;
```

### Struct construction

Structs are constructed with `()`. Every field must already be a variable in scope, and the variable name must match the struct field name exactly. There is no `field: value` pair syntax. Parens are always required, even for a single field. The type is inferred from which declared struct matches the given field names.

```
area as 9
name as "Office"
occupied as true
room as (area, name, occupied)
```

```rust
let area = Squarefeet::new(9);
let name = "Office".to_string();
let occupied = true;
let room = Room { area, name, occupied };
```

This mirrors destructuring exactly — `in` and `as` are opposite directions:

```
(area, name) in room      # extract fields from a struct
room as (area, name)      # construct a struct from variables
```

If you need a field name that differs from the variable you have, rename it first:

```
name in other_room
label = name
entry as (label)
```

### List construction

A list literal `[item1, item2, ...]` constructs a `list<T>` whose element type is inferred from the items. All items must be named variables of the same type already in scope.

```
room_list as [kitchen, office, bedroom]
```

```rust
let room_list = vec![kitchen.clone(), office.clone(), bedroom.clone()];
```

An empty list `[]` cannot infer its element type — use an explicit typed declaration instead:

```
list<Room> result = []    # correct — type explicit
result as []              # transpiler error — element type unknown
```

---

**What `as` is not for:**

`as` is the type-inferring form — like `:=` in Go. An explicit type annotation is never written with `as`. When you have a type, use `=`.

```
count as 0          # correct — type inferred as int
int count as 0      # transpiler error — type annotation not allowed with as
int count = 0       # correct — explicit type with =
```

`as` is also not for function calls, arithmetic, or expressions where the type does not come from structural shape. Those use `Type name = expr`:

```
int val = rand(1, 10)          # runtime computation — explicit type required
Room room = some_function()  # function return — explicit type required
```

Rebinding from an existing variable is not a valid use of `as`:

```
copy as original    # transpiler error — use Type name = original instead
```

Record update (`with`) also uses `as` — see [Immutability](immutability.md).

---

## Explicit Typing — Runtime Values

Any value that depends on a function call or other runtime computation must use `Type name = expr`.

```
int val = rand(1, 10)
string pick = random_room_name(rooms)
list<int> result = []
```

```rust
let val: i32 = rand(1, 10);
let pick: String = random_room_name(&rooms);
let mut result: Vec<i32> = Vec::new();
```

**Conversion notes:** a `list<T> name = []` binding that's later `insert`ed into must be emitted as `let mut` even though source never writes a mutability marker — the transpiler infers `mut` from usage.

---

## Reassignment

```
total = total + 1
```

```rust
total += 1;
```

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

## Validator Type Variables

Declaring a variable with a validator type makes it an option-type. Assignment runs the predicate — the variable is truthy (`Some`) or falsy (`None`) as a result.

```
Roll roll = rand(1, 20)    # Some if predicate passes, None if not
```

### Initializing to None

A validator type variable can be declared as `none` to start explicitly absent. Valid only at first declaration — `= none` after that point is a transpiler error.

```
Roll best = none
```

```rust
let mut best: Option<Roll> = None;
```

This is the preferred way to declare a "not yet determined" validator type value. It is more readable than relying on a known-failing literal (e.g. `Roll best = 0`), even though both produce `None`.

### Reassignment

Reassigning a validator type re-runs the predicate. The variable may transition between `Some` and `None` through a failing value — this is expected behavior, not an error.

```
Roll roll = 5      # Some(Roll(5))
roll = 150         # None — fails Roll predicate (val <= 100)
roll = 10          # Some(Roll(10)) again
```

The transpiler will eventually catch predicate failures on literals at transpile time. Currently all non-literal predicate failures are runtime.
