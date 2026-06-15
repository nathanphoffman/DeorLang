# Best Practices

Style recommendations for idiomatic Deor. Not enforced by the transpiler.

---

## Shape Naming — `<descriptor>List` / `<descriptor>Func`

Shape names should end with the kind of thing they represent. The camelCase convention is enforced; the suffix pattern is not — but following it makes shapes immediately readable.

| Shape kind | Suffix | Examples |
|---|---|---|
| List shapes | `List` | `roomList`, `intList`, `rollList`, `nameList` |
| Func shapes | `Func` | `filterFunc`, `predicateFunc`, `handlerFunc`, `transformFunc` |

`roomList` tells you it is a list. `filterFunc` tells you it is a function type. Without the suffix, `rooms` and `filter` look like variables or functions at a glance.

---

---

## Blank Line Before `return`

Add a blank line before `return` in any function body that contains more than one statement. One-liner functions (single expression, no bindings) are exempt.

**Recommended:**
```
fn int sum_rolls(RollResult list rolls)
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
