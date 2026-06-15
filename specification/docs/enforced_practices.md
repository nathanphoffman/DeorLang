# Enforced Practices

These rules are enforced by the transpiler. Violations produce warnings or compile-time errors.

---

## Naming Conventions

| Category | Convention | Examples |
|---|---|---|
| Built-in primitives | lowercase | `int`, `float`, `bool`, `string`, `bytes` |
| Built-in generics | lowercase | `list` |
| User-defined types (structs, validator types) | PascalCase, 3+ chars | `Room`, `RollResult`, `Squarefeet` |
| Functions, variables, parameters, struct fields | snake_case, 3+ chars | `roll_die`, `total_area`, `room_list` |
| Constants | SCREAMING_SNAKE_CASE, 3+ chars | `DELAY_TIME`, `MAX_RETRIES` |

**The key signal:** PascalCase exclusively marks user-defined types. Seeing `Room` or `SquareFeet` in code guarantees it was declared with `struct` or `type` — never a built-in, never a function, never a variable.

---

## Minimum Name Length — 3 Characters

All identifiers must be at least 3 characters long. This applies to every named thing in Deor source: variables, function parameters, function names, struct names, validator type names, struct field names, and list names.

```
int val = 5      # correct
int vl = 5       # transpiler error — 2 characters
int v = 5        # transpiler error — 1 character

fn int add(int left, int right)   # correct
fn int add(int a, int b)          # transpiler error — parameters too short

type Roll(int val)    # correct
type Roll(int n)      # transpiler error — parameter too short
```

**The only exception** is generic type parameters declared with `[shape: T]` — single-letter type parameter names (`T`, `U`) follow universal convention and are exempt. Runtime identifiers (variables, parameters, fields, functions, types) are never exempt.

---

## Field Extraction Order

Struct field extraction with `in` must follow declaration order — the same rule as struct construction with `as`. The field you write first must match the first declared field of the struct.

**Correct:**
```
struct Room
    Squarefeet area
    string name

(area, name) in room    # correct — matches declaration order
```

**Incorrect — transpiler errors:**
```
(name, area) in room    # wrong order — area is declared first
```

Single-field extraction has no ordering constraint.

---

## Destructuring at Top of Block

All `in` extractions must appear before any logic (assignments, expressions, control flow) within their block. Applies to function bodies, loop bodies, and if/else bodies.

**Correct:**
```
fn RollResult roll_die(Die die)
    (sides, label) in die

    min as 1
    int raw = rand(min, sides)
    Roll value = raw
    string source = label
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
type Positive(int val)
    val > 0
```

**Incorrect — transpiler errors:**
```
type Positive(int val)
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
fn Roll find_best(RollResult list rolls)
    Roll best = none
    for roll in rolls
        value in roll
        if roll
            best = value

    return best
```

**Incorrect — transpiler errors:**
```
fn Roll find_best(RollResult list rolls)
    return none
```

---

## `as` — No Type Annotation, No Variable Rebinding

`as` is the type-inferring binding form. Two things are always transpiler errors with `as`:

**Type annotation with `as`:** when you have an explicit type, use `=` instead.

```
count as 0          # correct — int inferred
int count as 0      # transpiler error — annotation not allowed with as
int count = 0       # correct
```

**Rebinding from an existing variable:** `as` requires a structural RHS — a scalar literal, `(fields)`, `[items]`, or `name with field`. Pointing it at a plain variable name is not a structural form.

```
copy as original    # transpiler error — use Type name = original
```

---

## Variable Shadowing

Re-declaring a variable with the same name in the same block is a transpiler error. Shadowing in an inner block (inside an `if`, `for`, or nested `fn` body) is allowed.

**Correct — inner block shadows outer:**
```
int val = 5
if condition
    int val = 10    # shadows outer val within this block only
    print(val)      # 10
print(val)          # 5
```

**Incorrect — same block re-declaration:**
```
int val = 5
int val = 10    # transpiler error
```

Use reassignment instead:
```
int val = 5
val = 10    # correct
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
fn (int quotient, int remainder) divmod(int left, int right)
    ...

(quo, rem) in divmod(num, div)    # quo = quotient, rem = remainder — names are yours
```

---

## Named Arguments — No Inline Literals, Expressions, or Inline Construction in Calls

All arguments passed to any function — user-defined or built-in — must be named variables already in scope. Literals, arithmetic expressions, inline function call results, and inline struct constructions are not valid as arguments.

**Correct:**
```
fn int add(int left, int right)
    return left + right

num as 5
amt as 3
int result = add(num, amt)
```

```
min as 1
max as 6
int roll = rand(min, max)
```

```
msg as "Hello"
print(msg)
```

```
message as "Parse failed"
body as input
Error err = (message, body)
error_handler(err)
```

**Incorrect — transpiler errors:**
```
int result = add(5, 3)           # literals not allowed
int roll = rand(1, 6)            # literals not allowed — even for builtins
print("hello")                   # literal not allowed — even for builtins
int val = pow(2, 10)             # literals not allowed
error_handler((message, body))   # inline struct construction not allowed
int idx = rand(0, len(rooms) - 1)  # expression not allowed
```

This rule applies uniformly to all function calls. The rationale: named variables make every argument self-documenting at the call site, and intermediate bindings keep expressions readable and debuggable. There are no exemptions for built-ins.

---

## Visibility — `private`

By default, all top-level declarations (`fn`, `type`, `struct`, `const`) are importable by other files. The `private` prefix restricts a declaration to the current file — it cannot be named in an `in` import from anywhere else. Attempting to import a `private` declaration is a transpiler error.

```
private fn build_key(string base)
    ...

private type InternalScore(int val)
    val >= 0 and val <= 255

private const int MAX_RETRIES = 3
```

`private` is file-level only. It has no effect on visibility within the same file — everything in a file can see everything else in that file regardless of `private`. Struct fields have no visibility modifier; when a struct is importable, all its fields are accessible via destructuring.

---

## Top-to-Bottom Declaration Order

All top-level declarations must appear before any code that references them. This applies to `struct`, `type`, `const`, and `fn` definitions within a file.

**Correct:**
```
type Roll(int val)
    val >= 1 and val <= 20

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
fn bool is_valid(int val)
    return val > 0

fn string describe(int val)
    if is_valid(val)
        return "positive"
    return "invalid"
```

**Incorrect — transpiler errors:**
```
fn string describe(int val)
    fn bool is_valid(int num)    # not allowed
        return num > 0
    if is_valid(val)
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
