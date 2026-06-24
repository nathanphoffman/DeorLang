# Move

By default Deor clones every value — function arguments, loop elements, and assignments all get a fresh copy. This is safe and simple but has a performance cost. `move` opts out of cloning for a specific operation and transfers ownership instead, matching Rust's default behavior. The original variable is no longer accessible after a move.

---

## Function Arguments

Pass a value into a function without cloning:

```
do_something(move big_list)
# big_list cannot be used after this point
```

```rust
do_something(big_list);
```

Use this when the function takes the last or only use of a large list or struct and cloning would be wasteful.

---

## Loop Iteration

Iterate a collection consuming each element rather than cloning:

```
for move (item in collection)
    process(item)
```

```rust
for item in collection {
    process(item);
}
```

The collection itself is consumed — it cannot be used after the loop.

---

## Struct Construction

Build a struct from fields without cloning them:

```
Score built = move (label, points)
```

```rust
let built = Score { label, points };
```

Fields are moved into the struct rather than cloned. Each source variable is consumed and cannot be used after the construction.

---

## Variable Assignment

Transfer ownership into a new binding:

```
string new_var = move prev_var
# prev_var is not accessible here and below
```

```rust
let new_var: String = prev_var;
```

---

## When to Use

`move` is a performance tool — reach for it when profiling shows clone overhead on large collections or structs. Standard Deor (no `move`) is always correct; `move` trades safety (original gone) for speed (no copy). Use `rust` blocks for the most performance-critical paths.
