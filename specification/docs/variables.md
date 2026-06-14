# Variables

## `as` — Literal-Derived Bindings (Compile-Time Only)

`as` derives a binding's type from a literal value. It cannot be used with runtime expressions.

```
sum as 0
room_list as [kitchen, office, bedroom]
```

```rust
let sum = 0;
let room_list = vec![kitchen.clone(), office.clone(), bedroom.clone()];
```

**Conversion notes:** if `Room` isn't already declared via `struct`, the transpiler must **synthesize a matching struct definition** from the literal's shape — this is the core "derive a type from data" feature inherited from TS's `as const`. String literals become `.to_string()` (owned) or `&str` (borrowed) depending on how the binding is used downstream.

### Struct Construction

Structs are constructed with `()`. Every field must already be a variable in scope, and the variable name must match the struct field name exactly. There is no `field: value` pair syntax. Parens are always required, even for a single field.

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

This mirrors destructuring exactly — `in` and `as` are the direction:

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

---

## Explicit Typing — Runtime Values

Any value that depends on a function call or other runtime computation must use `Type name = expr`.

```
int t = rand(1, 10)
string pick = random_room_name(rooms)
list<int> result = []
```

```rust
let t: i32 = rand(1, 10);
let pick: String = random_room_name(&rooms);
let mut result: Vec<i32> = Vec::new();
```

**Conversion notes:** a `list<T> name = []` binding that's later `append`ed must be emitted as `let mut` even though source never writes a mutability marker — the transpiler infers `mut` from usage.

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
const float PI = 3.14159
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
Roll r = rand(1, 20)    # Some if predicate passes, None if not
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
Roll r = 5      # Some(Roll(5))
r = 150         # None — fails Roll predicate (n <= 100)
r = 10          # Some(Roll(10)) again
```

The transpiler will eventually catch predicate failures on literals at transpile time. Currently all non-literal predicate failures are runtime.
