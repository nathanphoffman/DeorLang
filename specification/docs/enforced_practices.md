# Enforced Practices
These rules are enforced by the transpiler. Violations produce warnings or compile-time errors.

---
## Naming Conventions
- enums, structs, and custom types (type validators) MUST be PascalCase
  - think structure = PascalCase
  - the logic behind this is it stands out boldly, but blends together as boldness > readability
- shapes must be camelCase
  - think aliasing = camelCase
  - the logic behind this is it stands out fairly well, but blends together as boldness = readability
- variable and function names must be snake_case
  - think runtime items = snake_case
  - the logic behind this is that these are very important to be readable as readability > boldness
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
## Ordering
Imports are required to be at the top of a file, however there are no other restrictions, but there are suggestions for [best practices](docs/best_practices.md).


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

`= empty` is only valid at the point of first declaration for a list shape variable. It is not valid for validator types — declare a validator type variable without a value instead.

**Correct:**
```
roomList rooms = empty    # list shape — use empty
Roll best                 # validator type — no value, starts not valid
```

**Incorrect — transpiler errors:**
```
roomList rooms = [kitchen]
rooms = empty             # cannot reassign empty after declaration

Roll best = empty         # empty not valid for validator types
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

Variable shadowing is allowed. A new declaration with the same name in the same block or an inner block replaces the binding from that point forward.

```
int val = 5
int val = 10    # allowed — val is now 10
print(val)      # 10
```

Inner block shadowing is also allowed and does not affect the outer binding:

```
int val = 5
if condition
    int val = 10    # shadows outer val within this block only
    print(val)      # 10
print(val)          # 5
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
## Unified `()` Rule — Named Variables

Everything placed inside `()` must be a named variable already in scope. This rule applies uniformly to:

| Context | Example |
|---|---|
| Function call | `add(value1, value2)` |
| Struct construction | `Room room = (area, name, occupied)` |
| Tuple return | `return (quotient, remainder)` |

Order does not matter for struct construction — fields are matched by name. Order does matter for function calls and tuple returns, since those are positional.

**Correct:**
```
struct Room
    Squarefeet area
    string name

Squarefeet area = 20
name as "Office"
Room room = (area, name)      # correct
Room room = (name, area)      # also correct — order doesn't matter for structs
```

**Incorrect — transpiler error:**
```
Room room = ("Office", area)  # literal not allowed — name must be a variable
```

For struct destructuring with `in`, field names drive the binding — order does not matter and any subset is valid.

---
## Named Arguments — User-Defined Functions Only
When a user-defined function is called with **2 or 3 arguments**, every argument must be a named variable already in scope. Literals, arithmetic expressions, inline function call results, and inline struct constructions are not valid in that position.

When called with **exactly 1 argument**, the argument does not need to be a named variable — a literal or expression is allowed.

**Correct:**
```
fn int add(int left, int right)
    return left + right

num as 5
amt as 3
int result = add(num, amt)    # 2 args — both must be named
```

```
fn int double(int val)
    return val * 2

int result = double(5)        # 1 arg — literal allowed
```

**Incorrect — transpiler errors:**
```
int result = add(5, 3)               # 2 args — literals not allowed
int result = add(num + 1, amt)       # 2 args — expression not allowed
```

**Built-in functions** accept literals and expressions directly regardless of argument count — no named variable required:

```
print("Hello, world!")
int cnt = len(rooms)
for idx in range(0, 10)
    ...
```

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
