# Loops

## Collection Iteration

```
for room in rooms
    ...
```

```rust
for room in &rooms {
    ...
}
```

**Conversion notes:** the transpiler chooses `&rooms` (borrow) vs `rooms` (move/copy) based on whether `Room` is `Copy` and whether `rooms` is used again afterward.

---

## Numeric Iteration

`range(n)` is a builtin function that produces values from `0` to `n - 1`.

```
for i in range(count)
    ...
```

```rust
for i in 0..count {
    ...
}
```

**Conversion notes:** `range(n)` transpiles to Rust's `0..n` range expression. This keeps the source grammar free of additional punctuation — no `..` operator needed in source.
