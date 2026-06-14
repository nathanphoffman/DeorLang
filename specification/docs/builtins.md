# Built-in Functions

These are part of the `deor:` standard library and available without an explicit import.

---

## Output

| Function | Signature | Notes |
|---|---|---|
| `print(value)` | any type → void | Converts value to string and writes to stdout with newline |

```
print("Hello, world!")
print(count)
```

```rust
println!("{}", "Hello, world!");
println!("{}", count);
```

---

## Length

| Function | Signature | Notes |
|---|---|---|
| `len(value)` | `string` or `list<T>` → `int` | Number of characters or elements |

```
int n = len(name)
int size = len(rooms)
```

---

## Numeric Range

| Function | Signature | Notes |
|---|---|---|
| `range(n)` | `int` → range tuple | Produces values `0` through `n-1`; see [Loops](loops.md) |

`range(n)` is sugar for `(0, n)` in a `for` loop. Use an explicit tuple `(start, end)` for non-zero starts.

---

## Math

| Function | Signature | Notes |
|---|---|---|
| `pow(base, exp)` | `int, int` → `int` | Integer exponentiation |
| `sqrt(x)` | `float` → `float` | Square root |
| `abs(x)` | `int` → `int` | Absolute value |
| `floor(x)` | `float` → `int` | Round down |
| `ceil(x)` | `float` → `int` | Round up |
| `round(x)` | `float` → `int` | Round to nearest |
| `min(a, b)` | `int, int` → `int` | Smaller of two values |
| `max(a, b)` | `int, int` → `int` | Larger of two values |

```
int n = pow(2, 10)      # 1024
float r = sqrt(2.0)     # 1.414...
int small = min(a, b)
```

---

## Random

| Function | Signature | Notes |
|---|---|---|
| `rand(min, max)` | `int, int` → `int` | Random integer in `[min, max]` inclusive |

```
int roll = rand(1, 6)
```

```rust
// transpiles using rand crate (included in deor: stdlib)
use rand::Rng;
let roll: i32 = rand::thread_rng().gen_range(1..=6);
```

---

## Type Conversion

These convert between primitive types explicitly. `as` is not used for type conversion — it already carries three meanings in Deor (literal binding, struct construction, import aliasing).

| Function | Signature | Notes |
|---|---|---|
| `to_float(x)` | `int` → `float` | Widens integer to float |
| `to_int(x)` | `float` → `int` | Truncates float toward zero |
| `to_string(x)` | any primitive → `string` | Formats value as string |

```
float f = to_float(count)
int n = to_int(ratio)
string label = to_string(score)
```

```rust
let f: f64 = count as f64;
let n: i32 = ratio as i32;
let label: String = score.to_string();
```

`to_int` truncates — `to_int(2.9)` = `2`, `to_int(-2.9)` = `-2`. Use `floor`/`ceil`/`round` before converting if rounding behavior matters.

---

## Fallible Parsing

`parse_int` and `parse_float` parse strings that may or may not be valid numbers. They return stdlib validator types (`ParsedInt`, `ParsedFloat`) — `Some` on success, `None` on failure. The caller handles the result the same way as any other validator type.

| Function | Signature | Notes |
|---|---|---|
| `parse_int(s)` | `string` → `ParsedInt` | `None` if `s` is not a valid integer |
| `parse_float(s)` | `string` → `ParsedFloat` | `None` if `s` is not a valid decimal |

```
ParsedInt result = parse_int(user_input)
if result
    int n = (result is known)
    print(n)
```

```rust
let result: Option<ParsedInt> = user_input.parse::<i32>().ok().map(ParsedInt);
if let Some(r) = result {
    let n: i32 = r.0;
    println!("{}", n);
}
```

`ParsedInt` and `ParsedFloat` are stdlib validator types whose predicate is always `true` — `None` comes from the parse itself failing, not a domain constraint. From the caller's perspective they behave identically to any other validator type: check presence, then unwrap or provide a default.

```
int port = parse_int(port_str) else 8080    # default if unparseable
```
