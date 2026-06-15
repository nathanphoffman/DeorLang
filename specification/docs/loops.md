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

`range(count)` is a builtin function that produces values from `0` to `count - 1`. When called with two arguments, `range(start, end)` produces values from `start` up to but not including `end`. Both arguments must be named integer variables — inline literals are a transpiler error.

```
for idx in range(count)
    ...
```

```rust
for idx in 0..count {
    ...
}
```

**Conversion notes:** `range(count)` transpiles to Rust's `0..count`. `range(start, end)` transpiles to `start..end`.

---

## Explicit Range

`range(start, end)` produces values from `start` up to but not including `end`. Use this when the range does not start at zero. Both arguments must be named integer variables — inline literals are a transpiler error.

```
start as 1
end as 11
for idx in range(start, end)
    print(idx)    # prints 1 through 10
```

```rust
for idx in 1..11 {
    println!("{}", idx);
}
```

`range(count)` is shorthand for `range(0, count)`. `end` is always exclusive.

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

## No `while` Loop — By Design

Deor has no `while` keyword. This is intentional.

Every pattern that tempts you toward `while` falls into one of two categories:

**Bounded iteration with an early exit** — use `for` + `break`. This is strictly better than `while` because the maximum iteration count is explicit and termination is guaranteed:

```
# retry up to a limit — safer than while, bound is visible
for attempt in range(max_retries)
    Result result = try_connect(host)
    if result
        break
```

**Truly unbounded iteration** — game loops, server accept loops, I/O polling. These have no natural collection or bound. Use a `rust` block. This is not a workaround — it is the right tool. Unbounded loops almost always coincide with the need for low-level control: async runtimes, OS threads, syscalls, tight timing. Deor's abstractions do not cover that territory, and a bare `while` keyword would not either. The `rust` block makes the boundary explicit: past this point, you are in systems code.

```
fn void run_server(int port)
    rust
        let listener = std::net::TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
        for stream in listener.incoming() {
            handle(stream.unwrap());
        }
```

The line between "Deor handles this" and "Rust handles this" falls exactly where bounded iteration ends and unbounded iteration begins. That is a clean and defensible boundary.

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
