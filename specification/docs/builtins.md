# Built-in Functions
These functions are wired directly into the transpiler — no import, no wrapper needed.

Everything else lives in a standard library file under `lib/` or must be wrapped in a `rust` block. See [Libs](libs.md) for the standard library and custom wrapper patterns.

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

`range` is only valid as the iteration source in a `for` loop — it is not a value and cannot be assigned. See [Loops](loops.md) for full usage.

---

## `crash`
Terminates the program immediately with a message. A `string` is recommended — it produces the clearest panic output. The transpiler accepts exactly one argument and does not enforce the type.

Deor:
```
message as "An unknown server problem occurred"
crash(message)
```

Rust:
```rust
panic!("{}", message);
```

The generated `panic!("{}", x)` uses Rust's `Display` trait to format the argument. How other types behave:

| Deor type | Display output |
|---|---|
| `string` | the string value — recommended |
| `int` | decimal integer, e.g. `42` |
| `float` | decimal with fractional part, e.g. `3.14` |
| `bool` | `true` or `false` |

Structs, list shapes, and validator types do not implement `Display` by default and will cause a Rust compile error if passed to `crash`. If you need to crash with a struct or list value, extract a field or build a descriptive string first.
