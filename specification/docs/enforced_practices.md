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
