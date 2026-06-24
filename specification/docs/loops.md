# Loops

## Collection Iteration
```
for room in rooms
    (name) in room
    (occupied) in room
    if occupied
        print(name)
```

---
## Numeric Iteration
`range(count)` produces values from `0` to `count - 1`. `range(a_start_num, an_end_num)` produces values from `a_start_num` up to but not including `an_end_num`. `range` is a built-in, so literals are valid directly.

```
for idx in range(count)
    ...

start as 1
stop as 11
for idx in range(start, stop)
    print(idx)    # prints 1 through 10
```

```rust
for idx in 0..count {
    ...
}
for idx in 1..11 {
    println!("{}", idx);
}
```

**Note:** `range` with two bounds requires both to be named variables — the same rule as any multi-argument call. `end` is a reserved keyword; the conventional names for range bounds are `start` and `stop`.

`range(count)` is shorthand for `range(0, count)`.

---
## Bare Tuple Range
`(start, end)` is an alternative to `range(start, end)` when both bounds are already named variables in scope. Produces identical Rust output — use whichever reads more clearly.

```
for idx in (low, high)
    print(idx)
```

```rust
for idx in low..high {
    println!("{}", idx);
}
```

The index-free form works too:

```
for in (low, high)
    do_something()
```

```rust
for _ in low..high {
    do_something();
}
```

---
## Repeat Without an Index
When the loop index is not needed, write `for in range(n)` — the variable name is omitted but `in` stays:

```
for in range(10)
    do_something()

start as 1
stop as 11
for in range(start, stop)
    do_something()
```

```rust
for _ in 0..10 {
    do_something();
}
for _ in 1..11 {
    do_something();
}
```

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
`break` applies to the **innermost** loop only. Labeled breaks (breaking out of an outer loop from an inner one) are not supported.

---
## Condition-Based Loops — `for if`
`for if condition` is Deor's while loop. It loops as long as the condition is true.

```
for if cur < token_count
    # process token at cur
    cur = cur + 1
```

```rust
while cur < token_count {
    // process token at cur
    cur = cur + 1;
}
```

`for if true` is the infinite loop form — use with `break` to exit:

```
for if true
    if done
        break
    do_work()
```

```rust
while true {
    if done { break; }
    do_work();
}
```

`for if` fits the same keyword as collection and range iteration — `for` is Deor's single loop keyword, and the token after it determines the form: `item in collection`, `in range(n)`, or `if condition`.

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
