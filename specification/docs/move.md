# Move

By default Deor clones every value — function arguments, loop elements, and assignments all get a fresh copy. This is safe and simple but has a performance cost. `move` opts out of cloning for a specific operation and transfers ownership instead, matching Rust's default behavior. The original variable is no longer accessible after a move.

---

## Function Arguments

Pass a value into a function without cloning:

Deor:
```
do_something(move big_list)
# big_list cannot be used after this point
```

Rust:
```rust
do_something(big_list);
```

Use this when the function takes the last or only use of a large list or struct and cloning would be wasteful.

---

## Loop Iteration

Iterate a collection consuming each element rather than cloning:

Deor:
```
for move (item in collection)
    process(item)
```

Rust:
```rust
for item in collection {
    process(item);
}
```

The collection itself is consumed — it cannot be used after the loop.

---

## Struct Construction

Build a struct from fields without cloning them:

Deor:
```
Score built = move (label, points)
```

Rust:
```rust
let built = Score { label, points };
```

Fields are moved into the struct rather than cloned. Each source variable is consumed and cannot be used after the construction.

---

## Variable Assignment

Transfer ownership into a new binding:

Deor:
```
string new_var = move prev_var
# prev_var is not accessible here and below
```

Rust:
```rust
let new_var: String = prev_var;
```

---

## When to Use

`move` is a performance tool. Deor's default clone-everything behavior is always *correct* — your program will produce the right answers without it. But cloning a large list or struct on every call or loop iteration has a real cost, and `move` eliminates that cost by transferring ownership instead of copying.

The tradeoff: the original variable is gone after a move. If you need the value again, you cannot use `move`. Reach for `move` when:

- A large collection or struct is passed to a function and never used again afterward
- A loop iterates a collection that can be consumed rather than kept
- A struct is built from fields that have no further use after construction

Use `rust` blocks for the most performance-critical paths where even the transpiler layer is too much overhead.
