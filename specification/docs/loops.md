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

---

## `break` — Exit a Loop Early

`break` exits the innermost loop immediately. Execution continues after the loop body.

```
found as false
for item in items
    matching in item
    if matching
        found = true
        break
```

```rust
let mut found = false;
for item in &items {
    let matching = item.matching;
    if matching {
        found = true;
        break;
    }
}
```

`break` applies to the **innermost** loop only. Labeled breaks (breaking out of an outer loop from an inner one) are not supported in v1 — restructure or use a flag variable.

---

## `continue` — Skip to Next Iteration

`continue` skips the rest of the current loop body and moves to the next iteration.

```
for item in items
    valid in item
    if not valid
        continue
    process(item)
```

```rust
for item in &items {
    let valid = item.valid;
    if !valid {
        continue;
    }
    process(item);
}
```

Like `break`, `continue` applies to the innermost loop only.
