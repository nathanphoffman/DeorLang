# Enforced Practices

These rules are enforced by the transpiler. Violations produce warnings or compile-time errors.

---

## Naming Conventions

| Category | Convention | Examples |
|---|---|---|
| Built-in primitives and keywords | lowercase | `int`, `float`, `bool`, `string`, `list`, `func` |
| Shapes | camelCase, 3+ chars | `roomList`, `intList`, `filterFunc`, `handlerFunc` |
| Enums | camelCase, 3+ chars | `colorTag`, `statusTag`, `directionTag` |
| Enum variants | PascalCase, 3+ chars | `Red`, `Green`, `Active`, `Pending` |
| User-defined types (structs, validator types) | PascalCase, 3+ chars | `Room`, `RollResult`, `Squarefeet` |
| Functions, variables, parameters, struct fields | snake_case, 3+ chars | `roll_die`, `total_area`, `room_list` |
| Constants | SCREAMING_SNAKE_CASE, 3+ chars | `DELAY_TIME`, `MAX_RETRIES` |

**The key signals:**
- camelCase marks shapes and enums — `roomList` is a shape (type alias, never a value); `colorTag` is an enum (instantiable type, always a value)
- PascalCase marks user-defined types and enum variants — `Room` is a struct or validator type; `Red` is an enum variant

Seeing `roomList` guarantees it is a shape. Seeing `colorTag` guarantees it is an enum. Seeing `Room` guarantees it is a struct or validator type. Seeing `Red` in value position guarantees it is an enum variant.

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

There are no exceptions. All runtime identifiers — variables, parameters, fields, functions, type names, and shape names — must be at least 3 characters.

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
    int raw = random(min, sides)
    Roll value = raw
    string source = label
    RollResult result = (value, source)

    return result
```

**Incorrect — transpiler warns:**
```
fn RollResult roll_die(Die die)
    int raw = random(1, die)
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

## `empty` at Declaration Only

`= empty` is only valid at the point of first declaration for a validator type or list shape variable. Assigning `empty` to a variable after it has been declared is a transpiler error.

**Correct:**
```
Roll best = empty
roomList rooms = empty
```

**Incorrect — transpiler errors:**
```
Roll best = roll_die(d20)
best = empty
```

---

## No `return empty`

Returning `empty` (or the old `none`) directly from a function is a transpiler error. Always return a named typed variable — the presence or absence of a value is determined at assignment time, not at return.

**Correct:**
```
shape rollResultList = list of RollResult

fn Roll find_best(rollResultList rolls)
    Roll best = empty
    for roll in rolls
        value in roll
        if roll
            best = value

    return best
```

**Incorrect — transpiler errors:**
```
shape rollResultList = list of RollResult

fn Roll find_best(rollResultList rolls)
    return empty
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

## Maximum 3 Parameters per Function

Functions may accept at most 3 parameters. If more context is needed, bundle values into a struct first. This is enforced by the transpiler.

```
fn roomList filter(roomList items, string query, filterFunc predicate)    # correct — 3 params
```

```
fn roomList filter(roomList items, string query, int limit, filterFunc predicate)    # transpiler error — 4 params
```

`func` shape parameters count toward the limit the same as data parameters.

---

## No `func` Shapes as Struct Fields

Struct fields must be data types — primitives, validator types, other structs, or list shapes. A `func` shape field would make the struct a closure in disguise, which Deor does not allow.

**Correct:**
```
fn roomList apply(roomList items, filterFunc predicate)    # func as parameter — fine
```

**Incorrect — transpiler error:**
```
struct Config
    roomList items
    filterFunc predicate    # func shape as struct field — not allowed
```

---

## File Declaration Order

Top-level declarations must appear in this order: imports, shapes, structs, functions. The transpiler enforces this ordering. Declaring a struct before shapes, or a function before structs, is a transpiler error.

```
# 1. Imports
(random) in shims

# 2. Shapes
shape roomList = list of Room
shape filterFunc = func of Room to bool

# 3. Structs
struct House
    string address
    roomList rooms

# 4. Functions
fn roomList filter(roomList items, filterFunc predicate)
    ...

fn main()
    ...
```

This mirrors the block-level rule that destructuring (`in`) must appear before logic — declarations flow from abstract to concrete, top to bottom.

---

## Unified `()` Rule — Named Variables, Declaration Order

Everything placed inside `()` must be a named variable already in scope, and must appear in the order the receiving side declares its fields/parameters. This rule applies uniformly to:

| Context | Example | Order source |
|---|---|---|
| Function call | `add(value1, value2)` | Function parameter order |
| Struct construction | `Room room = (area, name, occupied)` | Struct field declaration order |
| Tuple return | `return (quotient, remainder)` | Function return type declaration order |

**Correct:**
```
struct Room
    Squarefeet area
    string name

Squarefeet area = 20
name as "Office"
Room room = (area, name)      # correct — matches declaration order
```

**Incorrect — transpiler errors:**
```
Room room = (name, area)      # wrong order — area must come first
Room room = ("Office", area)  # literal not allowed — name must be a variable
room as (area, name)          # no anonymous struct construction — type required
```

For tuple capture with `in`, your chosen names are positional but not required to match the function's declared return names:
```
fn (int quotient, int remainder) divmod(int left, int right)
    ...

(quo, rem) in divmod(num, div)    # quo = quotient, rem = remainder — names are yours
```

---

## Named Arguments — User-Defined Functions Only

All arguments passed to **user-defined functions** must be named variables already in scope. Literals, arithmetic expressions, inline function call results, and inline struct constructions are not valid arguments to user-defined functions.

**Correct:**
```
fn int add(int left, int right)
    return left + right

num as 5
amt as 3
int result = add(num, amt)
```

```
message as "Parse failed"
body as input
Error err = (message, body)
error_handler(err)
```

**Incorrect — transpiler errors:**
```
int result = add(5, 3)               # literals not allowed in user function call
error_handler((message, body))       # inline struct construction not allowed
int result = add(num + 1, amt)       # expression not allowed in user function call
```

**Built-in functions** accept literals and expressions directly — no named variable required:

```
print("Hello, world!")
int cnt = len(rooms)
for idx in range(0, 10)
    ...
for range(5)
    ...
```

The rationale: named variables make call sites self-documenting for user-defined functions, where the parameter names may not be universally known. Built-ins like `print`, `len`, and `range` are part of the language and universally understood — requiring named variables for them adds ceremony with no clarity benefit. This same logic applies to system constructs: `if` conditions, `for` headers, and compound assignments accept expressions freely.

---

## Visibility — `private`

By default, all top-level declarations (`fn`, `type`, `struct`, `shape`, `const`) are importable by other files. The `private` prefix restricts a declaration to the current file — it cannot be named in an `in` import from anywhere else. Attempting to import a `private` declaration is a transpiler error.

```
private fn build_key(string base)
    ...

private type InternalScore(int val)
    val >= 0 and val <= 255

private shape internalList = list of InternalItem

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

## `raw` — Assigned from `rust` Blocks Only

A `raw` variable must be assigned from a `rust` block return value. Assigning a `raw` variable from a Deor expression is a transpiler error. Consuming a `raw` variable outside a `rust` block is a transpiler error. A `raw` variable cannot be declared as a struct field.

**Correct:**
```
raw index = rust
    entries.iter()
        .map(|e| (e.key.clone(), e.value.clone()))
        .collect::<std::collections::HashMap<String, String>>()

string result = lookup(index, search_key)    # passing raw to a function that uses it in rust — ok
```

**Incorrect — transpiler errors:**
```
raw index = some_list           # raw must come from a rust block
string val = index              # raw cannot be used in a Deor expression
int cnt = len(index)            # len does not accept raw

struct Config
    raw lookup_table            # raw cannot be a struct field
```

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
