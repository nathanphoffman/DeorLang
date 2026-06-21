# Operators

## Arithmetic

| Operator | Meaning | Notes |
|---|---|---|
| `+` | Addition / string concat | See [strings](strings.md) for string `+` behavior |
| `-` | Subtraction | |
| `*` | Multiplication | |
| `/` | Division | Integer division truncates: `5 / 2 = 2` |
| `%` | Modulo / remainder | |
| `-x` | Unary negation | |

**Integer division** truncates toward zero — `5 / 2 = 2`, not `2.5`. Mix `int` and `float` to get a float result: `5.0 / 2 = 2.5`. This follows Rust's behavior and may surprise developers coming from Python or JavaScript.

```
int quo = 5 / 2       # 2 — truncated
float flt = 5.0 / 2   # 2.5
```

Deor has no `**` operator. For exponentiation, use the `pow_i` shim from [Shims — Math](shims.md#math):
```
base as 2
exp as 10
int val = pow_i(base, exp)    # 1024
```

---

## Comparison

| Operator | Meaning | Rust equivalent |
|---|---|---|
| `is` | Structural equality (always deep) | `==` |
| `is not` | Not equal | `!=` |
| `<` | Less than | `<` |
| `>` | Greater than | `>` |
| `<=` | Less than or equal | `<=` |
| `>=` | Greater than or equal | `>=` |

`is` and `is not` are two-word keyword operators — not symbols.

```
val is 5        # equality — val == 5
val is not 5    # inequality — val != 5
```

Forced unwrap of a validator type uses the separate `avow` keyword — it is not part of the `is` operator:

```
(avow val)    # forced unwrap — panics if None (validator types only)
```

See [Types — Forced Unwrap](types.md#forced-unwrap--avow) for full details.

---

## Logical

| Keyword | Meaning | Rust equivalent |
|---|---|---|
| `and` | Logical AND | `&&` |
| `or` | Logical OR | `\|\|` |
| `not` | Logical NOT (unary) | `!` |

```
if val > 0 and val < 100
    ...

if not is_valid
    ...
```

---

## Banned Symbolic Operators

The following are **transpiler errors** — they must never appear in Deor source:

| Banned | Use instead |
|---|---|
| `==` | `is` |
| `!=` | `is not` |
| `&&` | `and` |
| `\|\|` | `or` |

Using symbols where keywords are required reads as an error in source review, not just at compile time.

---

## Operator Precedence

Deor follows Rust's operator precedence. From highest to lowest, the operators you'll use in practice:

| Level | Operators | Notes |
|---|---|---|
| 1 (highest) | `not` | Unary logical NOT |
| 2 | `*` `/` `%` | Multiplicative |
| 3 | `+` `-` | Additive |
| 4 | `is` `is not` `<` `>` `<=` `>=` | Comparison |
| 5 | `and` | Logical AND |
| 6 (lowest) | `or` | Logical OR |

```
not val and flag           # (not val) and flag  — not binds tighter than and
left + right * mul         # left + (right * mul)    — standard math precedence
val is 0 or num > 5      # (val is 0) or (num > 5) — comparisons before logical
val is not 0 and num > 0  # (val is not 0) and (num > 0)
```

When in doubt, use parentheses. Deor has no operator precedence surprises beyond the standard math rules.

---

## No Bitwise Operators

Bitwise operations (`&`, `|`, `^`, `~`, `<<`, `>>`) are not exposed in Deor. Use a `rust` block for any code that requires bitwise manipulation.

```
fn int mask_flags(int flags)
    rust
        flags & 0xFF
```
