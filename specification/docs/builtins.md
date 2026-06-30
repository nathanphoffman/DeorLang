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

---

## `args()` and `input()` destructuring

Two built-in forms for reading word-split data into named variables. Both use the same `in` destructuring syntax and the same five keywords — any subset, any order.

| Form | Source |
|---|---|
| `(fields) in args()` | command-line arguments passed at launch |
| `(fields) in input()` | one line read from stdin |

```
(first, second, third, input_string, input_list) in args()
(first, second, third, input_string, input_list) in input()
```

| Keyword | Type | Value |
|---|---|---|
| `first` | `string` | first word — empty string `""` if not present |
| `second` | `string` | second word |
| `third` | `string` | third word |
| `input_string` | `string` | all words joined with a single space |
| `input_list` | `strList` | all words as a list |

Missing words default to `""` — no crash. Use `if first is ""` to detect absence.

**`args()` example** — reading CLI flags:
```
(first, second) in args()
print(first)
print(second)
```
Running `./prog hello world` prints `hello` then `world`.

**`input()` example** — prompting the user:
```
(first, input_list) in input()
print(first)
int count = len(input_list)
print(count)
```
If the user types `hello world extra`, prints `hello` then `3`.

For more than three words, use `input_list` directly (`input_list at 3`, `for item in input_list`, etc.).
