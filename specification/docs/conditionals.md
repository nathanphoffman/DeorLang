# Conditionals

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

## No Pattern Matching — By Design

Deor has no `match` keyword and no pattern matching syntax. This is intentional.

Dispatching on enum variants uses `if`/`else if` chains with `is` comparisons — the same operator used for equality everywhere else in Deor:

```
if color is Red
    print(msg_red)
else if color is Green
    print(msg_green)
else if color is Blue
    print(msg_blue)
```

For complex multi-arm dispatch that genuinely requires destructuring, use a `rust` block — Rust's `match` is fully available inside one. Deor's `if`/`else if` chains are the right tool for the tag-only unions Deor supports in v1. See [Enums](enums.md).
