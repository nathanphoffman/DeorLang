# Best Practices

Style recommendations for idiomatic Deor. Not enforced by the transpiler.

---

## Shape Naming

See [Shapes — Naming Convention](shapes.md#naming-convention) for the `List`/`Func` suffix pattern and the camelCase requirement.

---

## Blank Line Before `return`

Add a blank line before `return` in any function body that contains more than one statement. One-liner functions (single expression, no bindings) are exempt.

**Recommended:**
```
shape rollResultList = list of RollResult

fn int sum_rolls(rollResultList rolls)
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
