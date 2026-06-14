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
int a = 5 / 2       # 2 — truncated
float b = 5.0 / 2   # 2.5
```

Exponentiation uses the `pow` built-in — no `**` operator:
```
int n = pow(2, 10)    # 1024
```

---

## Comparison

| Operator | Meaning |
|---|---|
| `==` | Structural equality (always deep) |
| `!=` | Not equal |
| `<` | Less than |
| `>` | Greater than |
| `<=` | Less than or equal |
| `>=` | Greater than or equal |

---

## Logical

| Keyword | Meaning | Rust equivalent |
|---|---|---|
| `and` | Logical AND | `&&` |
| `or` | Logical OR | `\|\|` |
| `not` | Logical NOT | `!` |

```
if x > 0 and x < 100
    ...

if not is_valid
    ...
```

---

## No Compound Assignment

Deor has no `+=`, `-=`, `*=`, etc. Use explicit reassignment instead:

```
sum = sum + value    # correct
sum += value         # not valid
```

This is intentional — explicit reassignment keeps mutations visible and readable.

---

## No Bitwise Operators

Bitwise operations (`&`, `|`, `^`, `~`, `<<`, `>>`) are not exposed in Deor. Use a `rust` block for any code that requires bitwise manipulation.

```
fn int mask_flags(int flags)
    rust
        flags & 0xFF
```
