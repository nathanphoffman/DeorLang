# Conditionals

## The Three Uses of `else`

`else` appears in three distinct positions in Deor — the parser distinguishes them by context:

| Form | Where | Meaning |
|---|---|---|
| `if ... else` | Block keyword after `if` or `else if` | Opens the fallback branch |
| `value` *(newline)* `    else branch` | Indented below an assignment | Compact ternary false branch |
| `validatorVar else default` | Inline expression | Null-coalescing — extracts inner primitive or returns default |

Null-coalescing `else` only applies to **validator type** variables. It is never valid on plain `int`, `float`, `string`, `bool`, `list`, or structs.

---

## `if / else if / else`

Standard conditional blocks. No parentheses around the condition. All branches support multi-line bodies. `else if` is a flat two-word keyword pair — not a nested `if` inside an `else` block.

```
if val > 10
    do_something()
    do_more()
else if val > 5
    do_medium()
    also_this()
else if val > 0
    do_small()
else
    do_default()
```

```rust
if val > 10 {
    do_something();
    do_more();
} else if val > 5 {
    do_medium();
    also_this();
} else if val > 0 {
    do_small();
} else {
    do_default();
}
```

Any number of `else if` chains are allowed. `else` is always last and optional.

---

## Compact Ternary

For simple single-expression conditional assignments, the condition and fallback can be written as indented continuation lines directly below the assignment — no blank lines between them.

```
int result = value
    if value > 0
    else 0
```

```rust
let result: i32 = if value > 0 { value } else { 0 };
```

**Rules (enforced):**
- No blank lines between the assignment line, the `if` line, and the `else` line — they must be vertically adjacent. The visual tightness signals they are one expression.
- The `if` and `else` branches are each a single expression. Multi-line branches require a full `if/else` block instead.
- `else` is required in compact ternary form — there is no "conditional assignment with no fallback."

**Correct:**
```
string label = "positive"
    if val > 0
    else "non-positive"
```

**Incorrect — blank line breaks the unit, transpiler errors:**
```
string label = "positive"

    if val > 0
    else "non-positive"
```

**Too complex for compact form — use a full block:**
```
string label = "positive"
    if val > 0
    else
        compute_label(val)    # multi-line branch: use full if/else instead
```

When branches need more than one expression, use a full `if/else` block with explicit assignments:

```
string label = "default"
if val > 0
    string suffix = get_suffix(val)
    label = "positive " + suffix
else
    label = compute_label(val)
```
