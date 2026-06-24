# Best Practices
Style recommendations not enforced by the transpiler.

---
## Order of Declaration
1. Imports - Everything else could use it, relies on nothing else in the file
2. Enums - Relies likely on nothing else in the file
3. `const` declarations - They could rely on enums and are important to see (at top)
4. Types - Type validators being types must be defined early
5. Shapes - Shapes can reference almost anything above
6. Structs - Reliant on most everything above but still structural (so above functions)
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
        if value valid
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
## Destructuring

Field order does not matter in destructuring — fields are matched by name. Any subset in any order is valid.
```
struct Employee
    int employee_id
    string first_name
    string last_name

(employee_id, first_name, last_name) in employee
```


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
## Naming External Libs
Because Deor does not support third-party importing, the standard convention is to copy Deor files into the `lib/` folder. The following prefix pattern is recommended.

Standard library wrappers use the **`s_`** prefix — e.g. `s_join`, `s_trim`. These mirror existing Rust std functions closely.

Cargo crate wrappers use the **`cx_`** prefix — e.g. `cx_json_parse`. Note that this prefix is only for thin wrappers around a crate call; a third-party `.deor` file that happens to use cargo internally does not get the `cx_` prefix.

Personal or third-party Deor libs use the **`ex_`** prefix — e.g. `ex_do_cool_thing`. These might use rust blocks, cargo, or pure Deor, but they represent external logic distributed by blog, git, or copy-paste.