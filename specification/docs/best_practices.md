# Best Practices

Style recommendations for idiomatic Deor. Not enforced by the transpiler.

---

## Blank Line Before `return`

Add a blank line before `return` in any function body that contains more than one statement. One-liner functions (single expression, no bindings) are exempt.

**Recommended:**
```
fn int sum_rolls(list<RollResult> rolls)
    sum as 0

    for roll in rolls
        value in roll
        sum = sum + value

    return sum
```

**Exempt — one-liner, no blank line needed:**
```
fn int square(int x)
    x * x
```
