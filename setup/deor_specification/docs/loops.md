<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: blackboard -->
# Loops

## Collection Iteration
Iterating a list borrows it by default — `item` is a reference to each element, and the collection is still usable after the loop. Write the list name directly after `in`.

Deor:
```deor
for room in rooms
    (name) in room
    (occupied) in room
    if occupied
        print(name)
```

Rust:
```rust
for room in &rooms {
    let name = room.name.clone();
    let occupied = room.occupied;
    if occupied {
        println!("{}", name);
    }
}
```

Use a counted loop when you need the index — see [Counted Iteration](#counted-iteration) below. Use plain collection iteration when you only need the values.

---
## Move Iteration
`for move (item in collection)` consumes the collection instead of borrowing it — `item` is owned, and `collection` cannot be used after the loop. See [Move — Loop Iteration](docs/move.md#loop-iteration) for the full example and when to reach for it.

---
## Counted Iteration
`for idx N times` runs the body `N` times, with `idx` counting from `0` to `N - 1`. `N` can be a number, a variable, or any expression.

Deor:
```deor
for idx 10 times
    print(idx)    # prints 0 through 9

count as 4
for idx count times
    print(idx)    # prints 0 through 3

for idx len(rooms) times
    print(idx)
```

Rust:
```rust
for idx in 0..10 {
    println!("{}", idx);
}
for idx in 0..count {
    println!("{}", idx);
}
for idx in 0..rooms.len() as i64 {
    println!("{}", idx);
}
```

There is no start value. To skip items or start from another number, adjust inside the loop:

```deor
for idx len(rooms) times
    if idx < 1
        continue    # skip the first item

for idx 5 times
    print(idx + 1)    # prints 1 through 5
```

The loop variable follows the usual naming rules (snake_case, at least 3 characters). A count that is a single name works too: `for count times` has no loop variable, while `for idx count times` does.

---
## Repeat Without an Index
When the loop index is not needed, leave the variable out:

```deor
for 10 times
    do_something()

for count times
    do_something()
```

```rust
for _ in 0..10 {
    do_something();
}
for _ in 0..count {
    do_something();
}
```

---
## `break` — Exit a Loop Early
`break` exits the innermost loop immediately. Execution continues after the loop body.

Deor:
```deor
found as false
for room in rooms
    (occupied) in room
    if occupied
        found = true
        break
```

Rust:
```rust
let mut found = false;
for room in &rooms {
    let occupied = room.occupied;
    if occupied {
        found = true;
        break;
    }
}
```
`break` applies to the **innermost** loop only. Labeled breaks (breaking out of an outer loop from an inner one) are not supported.

---
## Condition-Based Loops — `for if`
`for if condition` is Deor's while loop. It loops as long as the condition is true.

Deor:
```deor
for if cur < token_count
    # process token at cur
    cur = cur + 1
```

Rust:
```rust
while cur < token_count {
    // process token at cur
    cur = cur + 1;
}
```

`for if true` is the infinite loop form — use with `break` to exit:

Deor:
```deor
for if true
    if done
        break
    do_work()
```

Rust:
```rust
loop {
    if done { break; }
    do_work();
}
```

`for if true` specifically (the literal condition `true`, not an expression that merely evaluates true) generates a 
bare `loop { ... }` rather than `while true { ... }`. This matters for functions that return a value: Rust gives `loop` the special 
"never returns normally" type when it has no reachable `break`, which lets the compiler accept a function whose only return path is inside 
the loop. `while true { ... }`, even though it runs identically, doesn't get that treatment from Rust — a function ending in `while true { ... }` 
with no value-producing code after it fails to compile with "expected T, found ()", regardless of whether every branch inside actually returns. 
Any other condition (`for if cur < token_count`, `for if not done`, etc.) always generates `while`, unaffected by this.

Deor:
```deor
fn int ask_until_valid()
    for if true
        int val = read_input()
        if is_valid(val)
            return val
```

Rust:
```rust
fn ask_until_valid() -> i64 {
    loop {
        let val: i64 = read_input();
        if is_valid(val) {
            return val;
        }
    }
}
```

`for if` fits the same keyword as collection and counted iteration — `for` is Deor's single loop keyword, and the tokens 
after it determine the form: `item in collection`, `idx N times`, or `if condition`.

---
## `continue` — Skip to Next Iteration

`continue` skips the rest of the current loop body and moves to the next iteration.

Deor:
```deor
for item in items
    (active) in item
    if not active
        continue
    process(item)
```

Rust:
```rust
for item in &items {
    let active = item.active;
    if !active {
        continue;
    }
    process(item);
}
```

Like `break`, `continue` applies to the innermost loop only.