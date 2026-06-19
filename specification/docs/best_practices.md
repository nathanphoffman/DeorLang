# Best Practices
Style recommendations for idiomatic recommendations, they are not enforced by the transpiler.

---
## Order of Declaration
1. Imports - Everything else could use it, relies on nothing else in the file
2. Enums - Relies likely on nothing else in the file
3. Consts - They could rely on enums and are important to see (at top)
4. Types - Type validators being types must be defined early
5. Structs - Reliant on most everything above but still structural (so above functions)
6. Shapes - Shapes can reference almost anything above, including structs
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
    print(roll_notice);

    # belongs together as it is all part of the loop
    sum as 0
    for roll in rolls
        value in roll
        int val = value else 0
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

All `in` extractions should appear before any logic (assignments, expressions, control flow) within their block. Applies to function bodies, loop bodies, and if/else bodies.

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
