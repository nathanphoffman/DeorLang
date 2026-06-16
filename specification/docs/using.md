# Using Blocks

`using` threads a value through a sequence of function calls, automatically passing the current state as the first argument to each call and capturing each return value as the new state.

## Syntax

```
Type result = using source
    call_one()
    call_two()
    call_three(extra_arg)
```

`source` is any expression. Each line in the block is a function call. After all calls, `result` holds the final state. The source is not modified — it is cloned.

---

## How State Threads

Given a `Cursor` struct:

```
struct Cursor
    int pos
    int depth
```

And these functions:

```
fn Cursor advance(Cursor c)
    (pos, depth) in c
    int pos = pos + 1
    Cursor result = (pos, depth)
    return result

fn Cursor enter_block(Cursor c)
    (pos, depth) in c
    int pos = pos + 1
    int depth = depth + 1
    Cursor result = (pos, depth)
    return result
```

A `using` block threads the cursor through a sequence of steps:

```
fn void process(Cursor start)
    Cursor end = using start
        advance()
        enter_block()
        advance()
    (pos, depth) in end
    rust
        println!("{} {}", pos, depth);
```

Generated Rust:

```rust
fn process(start: Cursor) {
    let mut _state = start.clone();
    _state = advance(_state.clone());
    _state = enter_block(_state.clone());
    _state = advance(_state.clone());
    let end = _state;
    let pos = end.pos.clone();
    let depth = end.depth.clone();
    println!("{} {}", pos, depth);
}
```

Starting at `(0, 0)`: after `advance` → `(1, 0)`, after `enter_block` → `(2, 1)`, after `advance` → `(3, 1)`. Output: `3 1`.

---

## Extra Arguments

Functions in a `using` block can accept additional arguments after the auto-threaded state:

```
Cursor result = using start
    advance()
    advance_by(5)
    enter_block()
```

`advance_by(5)` is called as `advance_by(_state.clone(), 5)`. The current state is always the first argument; any explicit arguments follow.

---

## Why Use It

Without `using`, cursor-walking requires threading the state manually:

```
fn void scan(Cursor c)
    Cursor c = advance(c)
    Cursor c = enter_block(c)
    Cursor c = advance(c)
```

With `using`, the state variable disappears from the call sites:

```
fn void scan(Cursor c)
    Cursor result = using c
        advance()
        enter_block()
        advance()
```

This is particularly useful for token-stream cursors, parse state, and any pattern where a record is repeatedly updated through a sequence of transformations.

---

## Constraints

- The type after `using` must be a struct — the return type of every function in the block must be the same struct type.
- Functions in the block must return the full struct (not void, not a field value).
- The block result is read-only after assignment. Destructure it with `in` to access fields.
- Auto-destructuring of fields inside the block (so bare field names work as args) is not yet implemented — see v2 roadmap.

---

**Conversion notes:** `using` compiles to a `let mut _state = src.clone()` followed by `_state = fn(_state.clone(), ...)` for each call, then `let result = _state`. The `_state` intermediate is not visible in Deor source.
