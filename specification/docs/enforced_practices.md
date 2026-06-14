# Enforced Practices

These rules are enforced by the transpiler. Violations produce warnings or compile-time errors.

---

## Naming Conventions

| Category | Convention | Examples |
|---|---|---|
| Built-in primitives | lowercase | `int`, `float`, `bool`, `string`, `bytes` |
| Built-in generics | lowercase | `list<T>`, `list<T, N>` |
| User-defined types (structs, validator types) | PascalCase | `Room`, `RollResult`, `Squarefeet` |
| Functions, variables, parameters, struct fields | snake_case | `roll_die`, `total_area`, `room_list` |
| Constants | SCREAMING_SNAKE_CASE | `DELAY_TIME`, `MAX_RETRIES` |

**The key signal:** PascalCase exclusively marks user-defined types. Seeing `Room` or `SquareFeet` in code guarantees it was declared with `struct` or `type` — never a built-in, never a function, never a variable.

---

## Destructuring at Top of Block

All `in` extractions must appear before any logic (assignments, expressions, control flow) within their block. Applies to function bodies, loop bodies, and if/else bodies.

**Correct:**
```
fn RollResult roll_die(Die die)
    (sides, label) in die

    int raw = rand(1, sides)
    Roll value = raw
    result as (value, source)

    return result
```

**Incorrect — transpiler warns:**
```
fn RollResult roll_die(Die die)
    int raw = rand(1, die)
    (sides, label) in die
```

This keeps blocks consistent: you always see what a block unpacks before reading its logic, the same way function parameters are declared before the body.

---

## Compact Ternary — No Blank Lines

In a compact ternary expression, the assignment line, `if` line, and `else` line must be vertically adjacent with no blank lines between them. The visual tightness is the signal that they form one expression.

**Correct:**
```
int result = value
    if value > 0
    else 0
```

**Incorrect — transpiler errors:**
```
int result = value

    if value > 0
    else 0
```

---

## Validator Type Predicate Required

A `type` definition must have a predicate body. A type with no constraint adds no meaning over the base type — use the base type directly instead.

**Correct:**
```
type Positive(int n)
    n > 0
```

**Incorrect — transpiler errors:**
```
type Positive(int n)
```

---

## `none` at Declaration Only

`= none` is only valid at the point of first declaration for a validator type variable. Assigning `none` to a variable after it has been declared is a transpiler error.

**Correct:**
```
Roll best = none
```

**Incorrect — transpiler errors:**
```
Roll best = roll_die(d20)
best = none
```

---

## No `return none`

Returning `none` directly from a function is a transpiler error. Always return a named validator type variable — its `None`-ness is determined by the type system, not by an explicit `return none`.

**Correct:**
```
fn Roll find_best(list<RollResult> rolls)
    Roll best = none
    for roll in rolls
        value in roll
        if roll
            best = value

    return best
```

**Incorrect — transpiler errors:**
```
fn Roll find_best(list<RollResult> rolls)
    return none
```

---

## Variable Shadowing

Re-declaring a variable with the same name in the same block is a transpiler error. Shadowing in an inner block (inside an `if`, `for`, or nested `fn` body) is allowed.

**Correct — inner block shadows outer:**
```
int x = 5
if condition
    int x = 10    # shadows outer x within this block only
    print(x)      # 10
print(x)          # 5
```

**Incorrect — same block re-declaration:**
```
int x = 5
int x = 10    # transpiler error
```

Use reassignment instead:
```
int x = 5
x = 10    # correct
```

---

## Multiple `using` Clauses — No Blank Lines, Indented

When a function call has multiple `using` clauses (`using shape` and/or `using fn_name`), they must be written on consecutive indented lines immediately below the call with no blank lines between them. `using shape` always comes first.

**Correct:**
```
filter(rooms)
    using shape Room
    using match_name
```

**Incorrect — blank line between clauses:**
```
filter(rooms)
    using shape Room

    using match_name
```

**Incorrect — on same line as call:**
```
filter(rooms) using shape Room using match_name
```

---

## `[using]` — Required at Call Site

Calling a `[using]`-annotated function without `using fn_name` is a transpiler error. The injection slot must always be filled.

**Correct:**
```
filter(rooms) using match_name
```

**Incorrect — transpiler errors:**
```
filter(rooms)
```

The alias name used in the body must match exactly what was declared in `[using alias: T]`.

---

## Unified `()` Rule — Named Variables, Declaration Order

Everything placed inside `()` must be a named variable already in scope, and must appear in the order the receiving side declares its fields/parameters. This rule applies uniformly to:

| Context | Example | Order source |
|---|---|---|
| Function call | `add(value1, value2)` | Function parameter order |
| Struct construction | `room as (area, name, occupied)` | Struct field declaration order |
| Tuple return | `return (quotient, remainder)` | Function return type declaration order |

**Correct:**
```
struct Room
    Squarefeet area
    string name

area as 20
name as "Office"
room as (area, name)      # correct — matches declaration order
```

**Incorrect — transpiler errors:**
```
room as (name, area)      # wrong order — area must come first
room as ("Office", area)  # literal not allowed — name must be a variable
```

For tuple capture with `in`, your chosen names are positional but not required to match the function's declared return names:
```
fn (int quotient, int remainder) divmod(int a, int b)
    ...

(q, r) in divmod(a, b)    # q = quotient, r = remainder — names are yours
```

---

## Named Arguments — No Inline Literals in Calls

All arguments passed to a user-defined function must be named variables defined before the call. Passing literals directly as function arguments is a transpiler error.

**Correct:**
```
fn int add(int a, int b)
    return a + b

int x = 5
int y = 3
int result = add(x, y)
```

**Incorrect — transpiler errors:**
```
int result = add(5, 3)    # literals not allowed
```

This applies to all user-defined function calls regardless of parameter count. Built-in functions (`print`, `len`, `range`, `rand`, etc.) are exempt — their parameters are documented by the language itself, so brief literal arguments are legible there.

The rationale: named variables make long parameter lists self-documenting. `connect(host, port, timeout, retries)` is clear; `connect("localhost", 8080, 30, 3)` is not.

---

## Top-to-Bottom Declaration Order

All top-level declarations must appear before any code that references them. This applies to `struct`, `type`, `const`, and `fn` definitions within a file.

**Correct:**
```
type Roll(int n)
    n >= 1 and n <= 20

struct RollResult
    Roll value
    string source

fn RollResult roll_die(Die die)
    ...
```

**Incorrect — transpiler errors:**
```
fn RollResult roll_die(Die die)    # uses RollResult before it's declared
    ...

struct RollResult
    Roll value
    string source
```

Recommended file layout:
1. `const` declarations
2. `type` definitions
3. `struct` definitions
4. Helper functions (called by others below them)
5. Main logic functions
6. Entry point (`fn main`)

This makes files readable top-to-bottom without needing to jump around to understand what a name means.

---

## No Nested Functions

Functions may only be declared at the top level of a file. Defining a `fn` inside another `fn` body is a transpiler error.

**Correct:**
```
fn bool is_valid(int n)
    return n > 0

fn string describe(int n)
    if is_valid(n)
        return "positive"
    return "invalid"
```

**Incorrect — transpiler errors:**
```
fn string describe(int n)
    fn bool is_valid(int x)    # not allowed
        return x > 0
    if is_valid(n)
        return "positive"
    return "invalid"
```

Move all helper functions to the top level of the file and call them by name.

---

## `rust` Blocks Must Be Indented

A `rust` block always requires a newline and indented body. One-liner `rust` is not allowed — the block form makes inline Rust visually obvious.

**Correct:**
```
fn string read_file(string path)
    rust
        std::fs::read_to_string(path.as_str())
            .unwrap_or_default()
```

**Incorrect — transpiler errors:**
```
fn string read_file(string path)
    rust std::fs::read_to_string(path.as_str()).unwrap_or_default()
```
