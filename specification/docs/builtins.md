# Built-in Functions

These functions are wired directly into the transpiler — no import, no wrapper needed.

Because they are built-ins, they accept literals directly. User-defined functions require named variables — see [Enforced Practices — Named Arguments](enforced_practices.md#named-arguments--user-defined-functions-only).

Everything else lives in a standard library file under `lib/` or must be wrapped in a `rust` block. See [Shims](shims.md) for copy-paste wrappers.

---

## `print`

Writes a value to stdout followed by a newline. Accepts any primitive type.

```
print("Hello, world!")
print(count)
print(score)
```

```rust
println!("{}", "Hello, world!");
println!("{}", count);
println!("{}", score);
```

---

## `len`

Returns the number of elements in a list or the number of characters in a string.

```
int size = len(rooms)
int chars = len(name)
```

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
