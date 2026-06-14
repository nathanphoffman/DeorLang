# Enforced Practices

These rules are enforced by the transpiler. Violations produce warnings or compile-time errors.

---

## Naming Conventions

| Category | Convention | Examples |
|---|---|---|
| Built-in primitives | lowercase | `int`, `float`, `bool`, `string` |
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
