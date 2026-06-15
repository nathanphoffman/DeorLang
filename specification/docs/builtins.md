# Built-in Functions

These are part of the `deor:` standard library and available without an explicit import. For string operations beyond `len` and concatenation, see [deor:strings](strings.md) — those require an explicit import.

---

## Output

| Function | Signature | Notes |
|---|---|---|
| `print(value)` | any type → void | Converts value to string and writes to stdout with newline |

```
msg as "Hello, world!"
print(msg)
print(count)
```

```rust
let msg = "Hello, world!".to_string();
println!("{}", msg);
println!("{}", count);
```

---

## Length

| Function | Signature | Notes |
|---|---|---|
| `len(value)` | `string` or `list` → `int` | Number of characters or elements |

```
int cnt = len(name)
int size = len(rooms)
```

---

## Numeric Range

| Function | Signature | Notes |
|---|---|---|
| `range(cnt)` | `int` → range tuple | Produces values `0` through `cnt-1`; see [Loops](loops.md) |

`range(cnt)` is sugar for `(0, cnt)` in a `for` loop. Use an explicit tuple `(start, end)` for non-zero starts.

---

## Math

| Function | Signature | Notes |
|---|---|---|
| `pow(base, exp)` | `int, NonNeg` → `int` | Integer exponentiation; negative exponent is a transpiler error |
| `sqrt(val)` | `float` → `NonNegFloat` | Square root; `None` if `val < 0` |
| `abs(val)` | `int` → `int` | Absolute value |
| `floor(val)` | `float` → `int` | Round down |
| `ceil(val)` | `float` → `int` | Round up |
| `round(val)` | `float` → `int` | Round to nearest |
| `min(left, right)` | `int, int` → `int` | Smaller of two values |
| `max(left, right)` | `int, int` → `int` | Larger of two values |

```
base as 2
exp as 10
int val = pow(base, exp)                  # 1024

num as 2.0
NonNegFloat res = sqrt(num)               # Some(1.414...)
float root = res else 0.0                 # 1.414...

num2 as 4.0
NonNegFloat res2 = sqrt(num2)
float root2 = res2 else 0.0               # 2.0

low as 3
high as 7
int small = min(low, high)
```

`sqrt` returns `NonNegFloat` — a stdlib validator type — so the result must be unwrapped before use in arithmetic. Use `else` for a safe default or `avow` when you are certain the input is non-negative.

---

## Random

| Function | Signature | Notes |
|---|---|---|
| `rand(min, max)` | `int, int` → `int` | Random integer in `[min, max]` inclusive; `throw` if `min > max` |

```
min as 1
max as 6
int roll = rand(min, max)
```

```rust
// transpiles using rand crate (included in deor: stdlib)
use rand::Rng;
let min: i32 = 1;
let max: i32 = 6;
// transpiler emits a guard: if min > max { panic!("rand: min > max") }
let roll: i32 = rand::thread_rng().gen_range(min..=max);
```

`min > max` is a programming error, not a data error — `rand` throws rather than returning `None`. If the bounds come from user input, validate them before calling `rand`.

---

## Type Conversion

These convert between primitive types explicitly. `as` is not used for type conversion — it already carries three meanings in Deor (literal binding, struct construction, import aliasing).

| Function | Signature | Notes |
|---|---|---|
| `to_float(val)` | `int` → `float` | Widens integer to float |
| `to_int(val)` | `float` → `int` | Truncates float toward zero |
| `to_string(val)` | any primitive → `string` | Formats value as string |

```
float flt = to_float(count)
int val = to_int(ratio)
string label = to_string(score)
```

```rust
let flt: f64 = count as f64;
let val: i32 = ratio as i32;
let label: String = score.to_string();
```

`to_int` truncates — `to_int(2.9)` = `2`, `to_int(-2.9)` = `-2`. Use `floor`/`ceil`/`round` before converting if rounding behavior matters.

---

## Fallible Parsing

`parse_int` and `parse_float` parse strings that may or may not be valid numbers. They return stdlib validator types (`ParsedInt`, `ParsedFloat`) — `Some` on success, `None` on failure. The caller handles the result the same way as any other validator type.

| Function | Signature | Notes |
|---|---|---|
| `parse_int(str)` | `string` → `ParsedInt` | `None` if `str` is not a valid integer |
| `parse_float(str)` | `string` → `ParsedFloat` | `None` if `str` is not a valid decimal |

```
ParsedInt result = parse_int(user_input)
if result
    int val = (avow result)
    print(val)
```

```rust
let result: Option<ParsedInt> = user_input.parse::<i32>().ok().map(ParsedInt);
if let Some(r) = result {
    let val: i32 = r.0;
    println!("{}", val);
}
```

`ParsedInt` and `ParsedFloat` are stdlib validator types whose predicate is always `true` — `None` comes from the parse itself failing, not a domain constraint. From the caller's perspective they behave identically to any other validator type: check presence, then unwrap or provide a default.

```
int port = parse_int(port_str) else 8080    # default if unparseable
```

---

## Stdlib Numeric Types

These validator types are part of the `deor:` stdlib and available without an explicit import. They represent constrained numeric domains and are used as parameter and return types for builtins that operate on restricted ranges.

| Type | Base | Predicate | Use |
|---|---|---|---|
| `NonNeg` | `int` | `val >= 0` | Non-negative integers — array indices, sizes, exponents |
| `Positive` | `int` | `val > 0` | Strictly positive integers — divisors, counts |
| `NonNegFloat` | `float` | `val >= 0.0` | Non-negative floats — `sqrt` return type, lengths |
| `PositiveFloat` | `float` | `val > 0.0` | Strictly positive floats — logarithm inputs, rates |

These behave identically to user-defined `type` declarations — they are `Option<T>` under the hood, support `avow`, `else`, and `if`/`if not` checks, and can be used as struct fields.

```
NonNeg exp = 10
base as 2
if exp
    int result = pow(base, exp)     # exp is NonNeg — passes type check

num as 4.0
NonNegFloat res = sqrt(num)            # Some(2.0)
float root = res else 0.0             # 2.0

neg as -1.0
NonNegFloat bad = sqrt(neg)        # None
float safe = bad else 0.0          # 0.0
```

**Conversion notes:**
- Each type compiles to a Rust newtype struct with a `fn new(n: T) -> Option<Self>` constructor, identical to user-defined validator types.
- `NonNeg` → `struct NonNeg(i32)`, `NonNegFloat` → `struct NonNegFloat(f64)`, etc.
- The stdlib provides these definitions — the transpiler never synthesizes them from user code.
