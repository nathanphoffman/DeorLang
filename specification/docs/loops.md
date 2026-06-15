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

`range(cnt)` is a builtin function that produces values from `0` to `cnt - 1`.

```
for idx in range(count)
    ...
```

```rust
for idx in 0..count {
    ...
}
```

**Conversion notes:** `range(cnt)` transpiles to Rust's `0..cnt` range expression. This keeps the source grammar free of additional punctuation — no `..` operator needed in source.

---

## Explicit Range

A 2-tuple `(start, end)` can be used directly in `for` — gives `idx` values from `start` up to but not including `end`.

```
for idx in (1, 11)
    print(idx)    # prints 1 through 10
```

```rust
for idx in 1..11 {
    println!("{}", idx);
}
```

`range(cnt)` is sugar for `(0, cnt)`. Use explicit tuples when you need a non-zero start.

Both `start` and `end` must be integer expressions (variables or literals). `end` is exclusive.
