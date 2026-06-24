# Built-in Functions
These functions are wired directly into the transpiler — no import, no wrapper needed.

Everything else lives in a standard library file under `lib/` or must be wrapped in a `rust` block. See [Libs](docs/libs.md) for the standard library and custom wrapper patterns.

---

## `print`
Writes a value to stdout followed by a newline. Accepts any primitive type.

Deor:
```
print("Hello, world!")
print(count)
```

Rust:
```rust
println!("{}", "Hello, world!");
println!("{}", count);
```

---

## `len`
Returns the number of elements in a list or the number of characters in a string.

Deor:
```
int size = len(rooms)
int chars = len(name)
```

Rust:
```rust
let size: i32 = rooms.len() as i32;
let chars: i32 = name.len() as i32;
```

---

## `range`
Produces an integer sequence for use in `for` loops. Two forms:

| Form | Produces |
|---|---|
| `range(count)` | `0` through `count - 1` |
| `range(start, end)` | `start` through `end - 1` (exclusive upper bound) |

```
for idx in range(10)
    print(idx)

for idx in range(3, 7)
    print(idx)
```

`range` is only valid as the iteration source in a `for` loop — it is not a value and cannot be assigned. See [Loops](docs/loops.md) for full usage.

---

## `crash`
Causes a hard exit / crash from the program, takes a string

Deor
```
message as "An unknown server problem occurred"
Error err = (message)
crash(message)
```

Rust: nearly identical to ```panic!("{}",message)```
