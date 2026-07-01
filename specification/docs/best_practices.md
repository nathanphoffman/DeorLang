# Best Practices
Style recommendations not enforced by the transpiler.

---
## Order of Declaration
1. Imports - Everything else could use it, relies on nothing else in the file
2. Enums - Relies likely on nothing else in the file
3. Structs - Reliant on most everything above but still structural (so above functions)
4. Types - Type validators being types must be defined early
5. Shapes - Shapes can reference almost anything above
6. Macros
7. Functions - Reliant on everything above

---
## Spacing
Add a blank line before `return` in any function body that contains more than one statement. One-liner functions (single expression, no bindings) are exempt. Try to keep blocks spaced out keeping like concepts with one another.

**Recommended:**
```
shape rollResultList = list of RollResult

fn int sum_rolls(rollResultList rolls)
    # is its own thing, just printing new line after these 2 lines
    roll_notice as "Going to roll"
    print(roll_notice)

    # belongs together as it is all part of the loop
    sum as 0
    for roll in rolls
        (value) in roll
        val as 0
        if value is valid
            val = (avow value)
        sum = sum + val

    return sum
```

**Exempt — one-liner, no blank line needed:**
```
fn int square(int val)
    val * val
```
---
## Construction and Destructuring

Field order does not matter — all construction and destructuring forms are name-matched. Any subset in any order is valid for destructuring; fields in construction are matched by variable name to struct field name.

```
struct Employee
    int employee_id
    string first_name
    string last_name

(employee_id, first_name, last_name) in employee
Employee emp = (employee_id, first_name, last_name)
emp as (employee_id, first_name, last_name)
```

Even though order is not enforced, write fields in the same order they appear in the struct declaration. It makes construction and destructuring sites easier to scan and keeps things consistent across the codebase.

Additionally, all `in` extractions should appear before any logic (assignments, expressions, control flow) within their block. Applies to function bodies, loop bodies, and if/else bodies.

**Correct:**
```
fn RollResult roll_die(Die die)

    (sides, label) in die

    min as 1
    int raw = m_rand_int(min, sides)

    Roll value = raw
    string source = label
    RollResult result = (value, source)

    return result
```
---
## avow

Always capture `avow` into its own binding — do not use `(avow val)` inline inside a larger expression.

**Recommended:**
```
if result is valid
    int val = (avow result)
    total = total + val
```

**Avoid:**
```
if result is valid
    total = total + (avow result)
```

Also always guard `avow` with an `is valid` check immediately above it, or add a comment explaining why the value is guaranteed valid at that point.

---
## Avoid Deep Nesting

Limit nesting to two or three levels. Deeply nested code is hard to read and usually signals that logic should be extracted into a helper function. Prefer early returns and guard clauses over deep `if/else` trees.

**Preferred — early return flattens nesting:**
```
fn string classify(int val)
    if val < 0
        return "negative"
    if val is 0
        return "zero"
    return "positive"
```

**Avoid — deep nesting:**
```
fn string classify(int val)
    if val < 0
        return "negative"
    else
        if val is 0
            return "zero"
        else
            return "positive"
```

---
## Keep Functions Small

A function should do one thing. If a function body is growing long or handles multiple distinct concerns, extract the inner logic into named helper functions. The 3-parameter limit already encourages this — if you need more context, you should already be reaching for a struct.

---
## Functions vs. Macros — Performance in Loops

Prefer functions for reusable logic that is called outside loops. Inside a loop body, prefer macros when the operation is simple and call overhead matters — macros expand inline, whereas function calls add a frame per iteration.

**Use a function — called once or outside a loop:**
```
fn int square(int val)
    val * val

int area = square(side)
```

**Use a macro — called inside a loop:**
```
macro sq(val)
    val * val

for item in items
    int area = sq(val)
```

If the logic is non-trivial, extract it into a function regardless — readability wins over minor call overhead for complex operations.

---
## File Length

Keep files to a reasonable length. There is no hard limit, but when a file starts to feel long, consider splitting it. A natural split point is when the file contains multiple distinct concerns — for example, separate structs and their associated functions into their own files.

---
## Naming External Libs
Because Deor does not support third-party importing, the standard convention is to copy Deor files into the `lib/` folder, prefixed the same two-letter way as the standard library. See [Libs — Naming Convention](docs/libs.md#naming-convention) for the full prefix scheme and examples.